# ─────────────────────────────────────────────
# Etapa 1: Chef — precalcula dependencias
# Usamos cargo-chef para cachear deps de Rust
# ─────────────────────────────────────────────
FROM rust:1.78-slim AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

# ─────────────────────────────────────────────
# Etapa 2: Planner — analiza qué dependencias
# necesita el proyecto (genera recipe.json)
# ─────────────────────────────────────────────
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ─────────────────────────────────────────────
# Etapa 3: Builder — compila todo
# Esta capa se cachea si recipe.json no cambia
# (es decir, si no cambiaste Cargo.toml)
# ─────────────────────────────────────────────
FROM chef AS builder

# Dependencias del sistema para openssl (necesario para CSD/FIEL)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Cachea SOLO las dependencias (paso costoso, ~3-5 min)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Ahora copia el código fuente y compila
COPY . .

# Genera los queries SQLx en tiempo de compilación
ENV SQLX_OFFLINE=true

RUN cargo build --release --bin cfdi-api

# ─────────────────────────────────────────────
# Etapa 4: Runtime — imagen final mínima
# Usamos distroless: sin shell, sin apt, seguro
# ─────────────────────────────────────────────
FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

# Copia el binario compilado
COPY --from=builder /app/target/release/cfdi-api /app/cfdi-api

# Copia las migraciones (SQLx las aplica al arrancar)
COPY --from=builder /app/crates/db/migrations /app/migrations

# Copia los XSLTs del SAT para la cadena original
COPY --from=builder /app/assets/xslt /app/assets/xslt

# Usuario no-root por seguridad
USER nonroot:nonroot

EXPOSE 8080

ENTRYPOINT ["/app/cfdi-api"]
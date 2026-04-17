CREATE TABLE IF NOT EXISTS emisores (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    rfc             VARCHAR(13)  NOT NULL UNIQUE,
    nombre          VARCHAR(300) NOT NULL,
    regimen_fiscal  VARCHAR(3)   NOT NULL,
    codigo_postal   VARCHAR(5)   NOT NULL,
    activo          BOOLEAN      NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_emisores_rfc ON emisores(rfc);
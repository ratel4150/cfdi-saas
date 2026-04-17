-- File: crates/db/migrations/005_catalogos_sat.sql
CREATE TABLE IF NOT EXISTS cat_claves_prod_serv (
    clave       VARCHAR(10) PRIMARY KEY,
    descripcion TEXT        NOT NULL,
    incluye_iva_trasladado BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS cat_claves_unidad (
    clave       VARCHAR(10)  PRIMARY KEY,
    nombre      VARCHAR(100) NOT NULL,
    descripcion TEXT
);

CREATE TABLE IF NOT EXISTS cat_codigos_postales (
    codigo_postal VARCHAR(5)   NOT NULL,
    estado        VARCHAR(100) NOT NULL,
    municipio     VARCHAR(100) NOT NULL,
    colonia       VARCHAR(200) NOT NULL,
    PRIMARY KEY (codigo_postal, colonia)
);
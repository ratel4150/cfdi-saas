DO $$ BEGIN
    CREATE TYPE estado_cfdi AS ENUM (
        'generado', 'sellado', 'timbrado', 'cancelado', 'error'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS cfdis (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    emisor_id           UUID          NOT NULL REFERENCES emisores(id),
    uuid_sat            UUID          UNIQUE,
    serie               VARCHAR(25),
    folio               VARCHAR(40),
    fecha               TIMESTAMPTZ   NOT NULL,
    tipo_comprobante    VARCHAR(1)    NOT NULL,
    subtotal            NUMERIC(15,2) NOT NULL,
    total               NUMERIC(15,2) NOT NULL,
    moneda              VARCHAR(3)    NOT NULL DEFAULT 'MXN',
    receptor_rfc        VARCHAR(13)   NOT NULL,
    receptor_nombre     VARCHAR(300)  NOT NULL,
    receptor_cp         VARCHAR(5)    NOT NULL,
    receptor_uso_cfdi   VARCHAR(4)    NOT NULL,
    xml_generado        TEXT          NOT NULL,
    xml_timbrado        TEXT,
    cadena_original     TEXT          NOT NULL,
    sello               TEXT,
    sello_sat           TEXT,
    no_certificado      VARCHAR(20),
    no_certificado_sat  VARCHAR(20),
    fecha_timbrado      TIMESTAMPTZ,
    estado              estado_cfdi   NOT NULL DEFAULT 'generado',
    error_mensaje       TEXT,
    tiene_carta_porte   BOOLEAN       NOT NULL DEFAULT false,
    metadata            JSONB,
    created_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cfdis_emisor_id     ON cfdis(emisor_id);
CREATE INDEX IF NOT EXISTS idx_cfdis_uuid_sat      ON cfdis(uuid_sat);
CREATE INDEX IF NOT EXISTS idx_cfdis_receptor_rfc  ON cfdis(receptor_rfc);
CREATE INDEX IF NOT EXISTS idx_cfdis_estado        ON cfdis(estado);
CREATE INDEX IF NOT EXISTS idx_cfdis_fecha         ON cfdis(fecha DESC);
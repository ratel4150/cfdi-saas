-- File: crates/db/migrations/004_carta_porte.sql
CREATE TABLE IF NOT EXISTS cartas_porte (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cfdi_id           UUID NOT NULL REFERENCES cfdis(id) ON DELETE CASCADE,
    version           VARCHAR(5)  NOT NULL DEFAULT '3.1',
    trans_internac    BOOLEAN     NOT NULL DEFAULT false,
    total_dist_rec    NUMERIC(10,2),
    ubicaciones       JSONB       NOT NULL DEFAULT '[]',
    mercancias        JSONB       NOT NULL DEFAULT '{}',
    figura_transporte JSONB       NOT NULL DEFAULT '[]',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_cartas_porte_cfdi_id ON cartas_porte(cfdi_id);
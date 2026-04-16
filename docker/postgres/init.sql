-- File: docker/postgres/init.sql
-- Habilita extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "vector";        -- para embeddings de catálogos SAT

-- Schema principal
CREATE SCHEMA IF NOT EXISTS cfdi;
CREATE SCHEMA IF NOT EXISTS sat;                -- catálogos SAT en su propio schema
CREATE SCHEMA IF NOT EXISTS tenants;            -- datos multi-tenant por RFC

-- Permisos
GRANT ALL PRIVILEGES ON SCHEMA cfdi    TO cfdi;
GRANT ALL PRIVILEGES ON SCHEMA sat     TO cfdi;
GRANT ALL PRIVILEGES ON SCHEMA tenants TO cfdi;
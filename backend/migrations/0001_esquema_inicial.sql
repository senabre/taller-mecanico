-- Esquema inicial de Talleres Rueda: TODAS las tablas del sistema (specs/000-base-comun/data-model.md).
-- Cualquier cambio posterior va en una migración nueva (NNNN_descripcion.sql) revisada por las dos
-- personas. Las reglas que dependen de otras filas o de «hoy» (km ≥ anterior, rol del mecánico,
-- días de cita, cupo, presupuesto emitido vigente…) no van aquí: son de cada spec.

-- =============================================================================================
-- Parte 1 · Tipos enumerados y tablas de la base común
-- =============================================================================================

CREATE TYPE rol AS ENUM ('recepcion', 'mecanico', 'almacen', 'jefe');

CREATE TYPE estado_orden AS ENUM (
    'recibida',
    'en_diagnostico',
    'en_reparacion',
    'en_espera_piezas',
    'pendiente_conformidad',
    'terminada',
    'facturada'
);

CREATE TYPE tipo_presupuesto AS ENUM ('inicial', 'ampliacion');

-- «caducado» no es un valor: se calcula (emitido y caduca_el < hoy).
CREATE TYPE estado_presupuesto AS ENUM ('borrador', 'emitido', 'aceptado', 'rechazado');

CREATE TYPE tipo_linea AS ENUM ('mano_obra', 'pieza');

CREATE TYPE estado_pedido AS ENUM ('pendiente', 'recibido');

CREATE TABLE usuario (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    nombre          TEXT NOT NULL,
    login           TEXT NOT NULL UNIQUE CHECK (login = lower(login)),
    hash_contrasena TEXT NOT NULL,  -- argon2id; nunca se devuelve en la API
    rol             rol NOT NULL,
    activo          BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE sesion (
    token_hash TEXT PRIMARY KEY,  -- SHA-256 del token de la cookie (research R8)
    usuario_id BIGINT NOT NULL REFERENCES usuario ON DELETE CASCADE,
    expira_en  TIMESTAMPTZ NOT NULL
);

-- Números correlativos sin huecos (research R5).
CREATE TABLE contador (
    nombre TEXT PRIMARY KEY,
    ultimo BIGINT NOT NULL DEFAULT 0
);

INSERT INTO contador (nombre) VALUES ('orden'), ('factura');

-- =============================================================================================
-- Parte 2 · Cliente, vehículo, cita y orden (spec-001, spec-002)
-- =============================================================================================

-- Principio V: ningún otro dato personal.
CREATE TABLE cliente (
    id       BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    nombre   TEXT NOT NULL,
    telefono TEXT NOT NULL,
    email    TEXT NOT NULL
);

CREATE TABLE vehiculo (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    -- Normalizada: mayúsculas, sin espacios ni guiones (matricula::normalizar_matricula).
    matricula  TEXT NOT NULL UNIQUE CHECK (matricula ~ '^[A-Z0-9]+$'),
    marca      TEXT NOT NULL,
    modelo     TEXT NOT NULL,
    cliente_id BIGINT NOT NULL REFERENCES cliente
);

CREATE TABLE cita (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    vehiculo_id BIGINT NOT NULL REFERENCES vehiculo,
    fecha       DATE NOT NULL,
    motivo      TEXT NOT NULL CHECK (btrim(motivo) <> ''),
    creada_en   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX cita_fecha ON cita (fecha);

CREATE TABLE orden (
    id                      BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    numero                  BIGINT NOT NULL UNIQUE,  -- contador('orden')
    cita_id                 BIGINT NOT NULL UNIQUE REFERENCES cita,
    vehiculo_id             BIGINT NOT NULL REFERENCES vehiculo,
    mecanico_id             BIGINT NOT NULL REFERENCES usuario,
    km                      INTEGER NOT NULL CHECK (km >= 0),
    danos                   TEXT NOT NULL,
    fecha_entrega_prometida DATE NOT NULL,
    diagnostico             TEXT NULL,
    estado                  estado_orden NOT NULL DEFAULT 'recibida',
    abierta_en              TIMESTAMPTZ NOT NULL DEFAULT now(),
    terminada_en            TIMESTAMPTZ NULL
);

CREATE INDEX orden_estado ON orden (estado);
CREATE INDEX orden_vehiculo ON orden (vehiculo_id);

-- =============================================================================================
-- Parte 3 · Piezas, presupuestos e imputaciones (spec-003, spec-005, spec-006)
-- =============================================================================================

CREATE TABLE pieza (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    referencia   TEXT NOT NULL UNIQUE CHECK (referencia = btrim(referencia) AND referencia <> ''),
    descripcion  TEXT NOT NULL,
    precio_venta NUMERIC(12,2) NOT NULL CHECK (precio_venta >= 0),
    stock        INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)
);

CREATE TABLE presupuesto (
    id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    orden_id       BIGINT NOT NULL REFERENCES orden,
    tipo           tipo_presupuesto NOT NULL,
    estado         estado_presupuesto NOT NULL DEFAULT 'borrador',
    token          TEXT NULL UNIQUE,         -- se genera al emitir (research R9)
    iva_porcentaje NUMERIC(5,2) NULL,        -- base, IVA y total se fijan al emitir
    base           NUMERIC(12,2) NULL,
    iva            NUMERIC(12,2) NULL,
    total          NUMERIC(12,2) NULL,
    emitido_en     TIMESTAMPTZ NULL,
    caduca_el      DATE NULL,
    respondido_en  TIMESTAMPTZ NULL
);

-- Como mucho un borrador por orden. Que no haya además un «emitido» sin caducar lo comprueba
-- spec-003 en el código, porque depende de hoy (data-model, spec-003 FR-008).
CREATE UNIQUE INDEX presupuesto_borrador_unico ON presupuesto (orden_id) WHERE estado = 'borrador';

CREATE TABLE linea_presupuesto (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    presupuesto_id  BIGINT NOT NULL REFERENCES presupuesto ON DELETE CASCADE,
    tipo            tipo_linea NOT NULL,
    concepto        TEXT NOT NULL,
    pieza_id        BIGINT NULL REFERENCES pieza,
    cantidad        NUMERIC(10,2) NOT NULL CHECK (cantidad > 0),
    precio_unitario NUMERIC(12,2) NOT NULL,
    importe         NUMERIC(12,2) NOT NULL,
    CHECK (tipo <> 'pieza' OR pieza_id IS NOT NULL)
);

CREATE TABLE imputacion (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    orden_id        BIGINT NOT NULL REFERENCES orden,
    tipo            tipo_linea NOT NULL,
    concepto        TEXT NOT NULL,
    pieza_id        BIGINT NULL REFERENCES pieza,
    cantidad        NUMERIC(10,2) NOT NULL CHECK (cantidad > 0),
    precio_unitario NUMERIC(12,2) NOT NULL,
    importe         NUMERIC(12,2) NOT NULL,
    usuario_id      BIGINT NOT NULL REFERENCES usuario,
    creada_en       TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Horas en múltiplos de 0,25.
    CHECK (tipo <> 'mano_obra' OR cantidad * 4 = trunc(cantidad * 4)),
    CHECK (tipo <> 'pieza' OR pieza_id IS NOT NULL)
);

CREATE INDEX imputacion_orden ON imputacion (orden_id);

-- =============================================================================================
-- Parte 4 · Proveedores, pedidos y facturas (spec-006, spec-007)
-- =============================================================================================

-- Precargados: la app no los crea, edita ni borra (spec-006 FR-010).
CREATE TABLE proveedor (
    id     BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    nombre TEXT NOT NULL UNIQUE
);

CREATE TABLE pedido (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    proveedor_id BIGINT NOT NULL REFERENCES proveedor,
    pieza_id     BIGINT NOT NULL REFERENCES pieza,
    cantidad     INTEGER NOT NULL CHECK (cantidad > 0),
    orden_id     BIGINT NOT NULL REFERENCES orden,
    estado       estado_pedido NOT NULL DEFAULT 'pendiente',
    pedido_en    TIMESTAMPTZ NOT NULL DEFAULT now(),
    recibido_en  TIMESTAMPTZ NULL
);

-- Como mucho un pedido pendiente por orden y referencia (spec-006 FR-004a).
CREATE UNIQUE INDEX pedido_pendiente_unico ON pedido (orden_id, pieza_id) WHERE estado = 'pendiente';

-- Factura simplificada (decisión Q2-A): copia del nombre del cliente y la matrícula.
CREATE TABLE factura (
    id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    numero         BIGINT NOT NULL UNIQUE,  -- contador('factura')
    orden_id       BIGINT NOT NULL UNIQUE REFERENCES orden,
    fecha          DATE NOT NULL,
    cliente_nombre TEXT NOT NULL,
    matricula      TEXT NOT NULL,
    iva_porcentaje NUMERIC(5,2) NOT NULL,
    base           NUMERIC(12,2) NOT NULL,
    iva            NUMERIC(12,2) NOT NULL,
    total          NUMERIC(12,2) NOT NULL,
    aviso_enviado  BOOLEAN NOT NULL DEFAULT false
);

-- Copia de cada imputación: la factura no cambia aunque cambie el catálogo.
CREATE TABLE linea_factura (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    factura_id      BIGINT NOT NULL REFERENCES factura ON DELETE CASCADE,
    concepto        TEXT NOT NULL,
    cantidad        NUMERIC(10,2) NOT NULL,
    precio_unitario NUMERIC(12,2) NOT NULL,
    importe         NUMERIC(12,2) NOT NULL
);

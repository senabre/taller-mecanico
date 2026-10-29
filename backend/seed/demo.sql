-- Datos de demostración. Las credenciales y tokens de este fichero son ficticios (constitución
-- v1.2.1, principio V). Nunca cargar en producción. Se carga sobre una base recién migrada:
--   psql "$DATABASE_URL" -f seed/demo.sql
-- Para empezar de cero: cargo sqlx database reset (y volver a cargar este fichero).

BEGIN;

-- ---------------------------------------------------------------------------------------------
-- Parte 1 · Usuarios demo (quickstart). Contraseña de los cinco: demo1234.
-- El hash (argon2id) se generó una vez con «cargo run --bin crear-usuario» contra una base local.
-- ---------------------------------------------------------------------------------------------

INSERT INTO usuario (nombre, login, hash_contrasena, rol) VALUES
    ('Ana',   'ana',   '$argon2id$v=19$m=19456,t=2,p=1$RtY++uElOlgnINbZWN3/hw$Cja15Pctev17fvU0CNH84FnylSfiRGbHsxN7Iv3qmKc', 'recepcion'),
    ('Luis',  'luis',  '$argon2id$v=19$m=19456,t=2,p=1$RtY++uElOlgnINbZWN3/hw$Cja15Pctev17fvU0CNH84FnylSfiRGbHsxN7Iv3qmKc', 'mecanico'),
    ('Marta', 'marta', '$argon2id$v=19$m=19456,t=2,p=1$RtY++uElOlgnINbZWN3/hw$Cja15Pctev17fvU0CNH84FnylSfiRGbHsxN7Iv3qmKc', 'mecanico'),
    ('Pepe',  'pepe',  '$argon2id$v=19$m=19456,t=2,p=1$RtY++uElOlgnINbZWN3/hw$Cja15Pctev17fvU0CNH84FnylSfiRGbHsxN7Iv3qmKc', 'almacen'),
    ('Paco',  'paco',  '$argon2id$v=19$m=19456,t=2,p=1$RtY++uElOlgnINbZWN3/hw$Cja15Pctev17fvU0CNH84FnylSfiRGbHsxN7Iv3qmKc', 'jefe');

-- ---------------------------------------------------------------------------------------------
-- Parte 2 · Catálogo: piezas y proveedores precargados (spec-006 FR-010)
-- ---------------------------------------------------------------------------------------------

INSERT INTO pieza (referencia, descripcion, precio_venta, stock) VALUES
    ('PF-123', 'Pastillas freno', 25.00, 4),  -- la del quickstart (§3, §5.2: stock 4)
    ('BJ-7',   'Bujía',            8.00, 0);  -- solo para el pedido demo pendiente (parte 3)

INSERT INTO proveedor (nombre) VALUES
    ('Recambios Sur'),
    ('Autopartes Norte');

-- ---------------------------------------------------------------------------------------------
-- Parte 3 · Clientes, vehículos, citas y órdenes en los 7 estados
--   Las citas son de días pasados para no gastar el cupo de los próximos días (quickstart §1.4).
--   Ninguna matrícula es 1234BCD: el quickstart §1 la da de alta como nueva.
--
--   nº  estado                 mecánico  para qué
--   1   recibida               luis      B: diagnóstico (spec-003)
--   2   en_reparacion          luis      B: imputaciones y pedidos (005, 006)
--   3   en_reparacion          luis      B: imputaciones y pedidos (005, 006)
--   4   en_diagnostico         luis      B: presupuesto (003); pedido rechazado (006)
--   5   en_diagnostico         marta     A: presupuesto emitido con token conocido (004)
--   6   en_espera_piezas       marta     pedido pendiente de BJ-7 (006)
--   7   pendiente_conformidad  luis      ampliación (005)
--   8   terminada              marta     facturar (007); entrega ya pasada: «Retrasada» (008)
--   9   facturada              luis      historial (007); no sale en el tablero (008)
-- ---------------------------------------------------------------------------------------------

INSERT INTO cliente (nombre, telefono, email) VALUES
    ('María López',     '600111222', 'maria.lopez@ejemplo.test'),
    ('Jorge Martín',    '600333444', 'jorge.martin@ejemplo.test'),
    ('Lucía Fernández', '600555666', 'lucia.fernandez@ejemplo.test'),
    ('Andrés Gil',      '600777888', 'andres.gil@ejemplo.test');

INSERT INTO vehiculo (matricula, marca, modelo, cliente_id)
SELECT v.matricula, v.marca, v.modelo, c.id
  FROM (VALUES
        ('1111BBB', 'Seat',       'Ibiza',   'maria.lopez@ejemplo.test'),
        ('2222CCC', 'Renault',    'Clio',    'jorge.martin@ejemplo.test'),
        ('3333DDD', 'Peugeot',    '208',     'lucia.fernandez@ejemplo.test'),
        ('4444FFF', 'Volkswagen', 'Golf',    'andres.gil@ejemplo.test'),
        ('5555GGG', 'Toyota',     'Corolla', 'maria.lopez@ejemplo.test'),
        ('6666HHH', 'Ford',       'Focus',   'jorge.martin@ejemplo.test'),
        ('7777JJJ', 'Citroën',    'C3',      'lucia.fernandez@ejemplo.test'),
        ('8888KKK', 'Kia',        'Ceed',    'andres.gil@ejemplo.test'),
        ('9999LLL', 'Opel',       'Corsa',   'maria.lopez@ejemplo.test')
       ) AS v (matricula, marca, modelo, email)
  JOIN cliente c ON c.email = v.email;

-- Una cita por vehículo, n días atrás.
INSERT INTO cita (vehiculo_id, fecha, motivo)
SELECT ve.id, current_date - d.dias, d.motivo
  FROM (VALUES
        ('1111BBB',  1, 'Revisión de los 60.000 km'),
        ('2222CCC',  4, 'Ruido al frenar'),
        ('3333DDD',  5, 'Vibración del volante'),
        ('4444FFF',  2, 'Testigo del motor encendido'),
        ('5555GGG',  3, 'Ruido al frenar'),
        ('6666HHH',  6, 'Tirones al acelerar'),
        ('7777JJJ',  8, 'Pastillas de freno gastadas'),
        ('8888KKK',  7, 'Chirrido al frenar'),
        ('9999LLL', 20, 'Cambio de pastillas')
       ) AS d (matricula, dias, motivo)
  JOIN vehiculo ve USING (matricula);

-- La orden se abre el día de su cita. «entrega»: días desde hoy (negativo = ya pasó).
INSERT INTO orden (numero, cita_id, vehiculo_id, mecanico_id, km, danos, fecha_entrega_prometida,
                   diagnostico, estado, abierta_en, terminada_en)
SELECT o.numero, ci.id, ve.id, u.id, o.km, o.danos, current_date + o.entrega, o.diagnostico,
       o.estado::estado_orden,
       (ci.fecha + time '09:30') AT TIME ZONE 'Europe/Madrid',
       CASE WHEN o.estado IN ('terminada', 'facturada')
            THEN (ci.fecha + 2 + time '18:00') AT TIME ZONE 'Europe/Madrid' END
  FROM (VALUES
        (1, '1111BBB', 'luis',   60120, 'Sin daños visibles',              3, NULL::text,                           'recibida'),
        (2, '2222CCC', 'luis',   98500, 'Rayón en el paragolpes trasero',  2, 'Pastillas delanteras desgastadas',   'en_reparacion'),
        (3, '3333DDD', 'luis',   45200, 'Sin daños visibles',              4, 'Discos de freno deformados',         'en_reparacion'),
        (4, '4444FFF', 'luis',  132000, 'Golpe en la puerta delantera',    5, 'Sensor de oxígeno defectuoso',       'en_diagnostico'),
        (5, '5555GGG', 'marta',  71300, 'Sin daños visibles',              3, 'Pastillas delanteras desgastadas',   'en_diagnostico'),
        (6, '6666HHH', 'marta',  88700, 'Faro derecho rajado',             2, 'Bujías en mal estado',               'en_espera_piezas'),
        (7, '7777JJJ', 'luis',   54900, 'Sin daños visibles',              1, 'Pastillas y discos desgastados',     'pendiente_conformidad'),
        (8, '8888KKK', 'marta',  39800, 'Arañazo en la aleta izquierda',  -1, 'Pastillas delanteras desgastadas',   'terminada'),
        (9, '9999LLL', 'luis',  120400, 'Sin daños visibles',            -15, 'Pastillas delanteras desgastadas',   'facturada')
       ) AS o (numero, matricula, mecanico, km, danos, entrega, diagnostico, estado)
  JOIN vehiculo ve USING (matricula)
  JOIN cita ci ON ci.vehiculo_id = ve.id
  JOIN usuario u ON u.login = o.mecanico;

-- Sin este pedido pendiente la orden 6 no podría salir nunca de «en espera de piezas».
INSERT INTO pedido (proveedor_id, pieza_id, cantidad, orden_id, estado, pedido_en)
SELECT pr.id, pz.id, 1, o.id, 'pendiente', o.abierta_en + interval '1 day'
  FROM proveedor pr, pieza pz, orden o
 WHERE pr.nombre = 'Recambios Sur' AND pz.referencia = 'BJ-7' AND o.numero = 6;

-- La primera orden real recibe el número 10, sin huecos.
UPDATE contador SET ultimo = (SELECT max(numero) FROM orden) WHERE nombre = 'orden';

-- ---------------------------------------------------------------------------------------------
-- Parte 4 · Presupuestos, imputaciones y factura
--   Todos los presupuestos tienen las mismas dos líneas: 1,5 h × 40,00 = 60,00 y
--   PF-123 × 2 × 25,00 = 50,00 → base 110,00, IVA 21 % 23,10, total 133,10 (dinero.rs).
--   Las imputaciones demo no descuentan stock: PF-123 se queda con 4 (quickstart §5.2).
-- ---------------------------------------------------------------------------------------------

-- Presupuesto inicial aceptado en las órdenes que ya están en reparación o más allá. Su token no
-- se usa en ninguna prueba: se genera al azar.
INSERT INTO presupuesto (orden_id, tipo, estado, token, iva_porcentaje, base, iva, total,
                         emitido_en, caduca_el, respondido_en)
SELECT o.id, 'inicial', 'aceptado', replace(gen_random_uuid()::text, '-', ''),
       21.00, 110.00, 23.10, 133.10,
       o.abierta_en + interval '3 hours',
       (o.abierta_en AT TIME ZONE 'Europe/Madrid')::date + 15,
       o.abierta_en + interval '5 hours'
  FROM orden o
 WHERE o.numero IN (2, 3, 6, 7, 8, 9);

-- Presupuesto emitido y sin responder, con el token ficticio publicado en el quickstart.
INSERT INTO presupuesto (orden_id, tipo, estado, token, iva_porcentaje, base, iva, total,
                         emitido_en, caduca_el)
SELECT o.id, 'inicial', 'emitido', 'demo-presupuesto-emitido',
       21.00, 110.00, 23.10, 133.10, now(), current_date + 15
  FROM orden o
 WHERE o.numero = 5;

INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, pieza_id, cantidad,
                               precio_unitario, importe)
SELECT p.id, l.tipo::tipo_linea, l.concepto, pz.id, l.cantidad, l.precio, l.importe
  FROM presupuesto p
 CROSS JOIN (VALUES
        ('mano_obra', 'Cambio de pastillas',    NULL::text, 1.50, 40.00, 60.00),
        ('pieza',     'PF-123 Pastillas freno', 'PF-123',   2.00, 25.00, 50.00)
       ) AS l (tipo, concepto, referencia, cantidad, precio, importe)
  LEFT JOIN pieza pz ON pz.referencia = l.referencia;

-- Imputaciones coherentes con el estado (presupuestado 110,00 €; 110 % = 121,00 €):
--   2 → 90,00 · 3 → 40,00 · 6 → 40,00 (≤ 121,00, siguen trabajando)
--   7 → 130,00 (> 121,00: pendiente de conformidad)
--   8 y 9 → 110,00 (terminada lista para facturar; facturada)
INSERT INTO imputacion (orden_id, tipo, concepto, pieza_id, cantidad, precio_unitario, importe,
                        usuario_id, creada_en)
SELECT o.id, i.tipo::tipo_linea, i.concepto, pz.id, i.cantidad, i.precio, i.importe,
       o.mecanico_id, o.abierta_en + interval '1 day'
  FROM (VALUES
        (2, 'mano_obra', 'Cambio de pastillas',    NULL::text, 1.00, 40.00, 40.00),
        (2, 'pieza',     'PF-123 Pastillas freno', 'PF-123',   2.00, 25.00, 50.00),
        (3, 'mano_obra', 'Revisión de frenos',     NULL,       1.00, 40.00, 40.00),
        (6, 'mano_obra', 'Desmontaje de bujías',   NULL,       1.00, 40.00, 40.00),
        (7, 'mano_obra', 'Cambio de pastillas',    NULL,       2.00, 40.00, 80.00),
        (7, 'pieza',     'PF-123 Pastillas freno', 'PF-123',   2.00, 25.00, 50.00),
        (8, 'mano_obra', 'Cambio de pastillas',    NULL,       1.50, 40.00, 60.00),
        (8, 'pieza',     'PF-123 Pastillas freno', 'PF-123',   2.00, 25.00, 50.00),
        (9, 'mano_obra', 'Cambio de pastillas',    NULL,       1.50, 40.00, 60.00),
        (9, 'pieza',     'PF-123 Pastillas freno', 'PF-123',   2.00, 25.00, 50.00)
       ) AS i (numero, tipo, concepto, referencia, cantidad, precio, importe)
  JOIN orden o ON o.numero = i.numero
  LEFT JOIN pieza pz ON pz.referencia = i.referencia;

-- Factura simplificada de la orden 9 (Q2-A), con copia de sus imputaciones.
INSERT INTO factura (numero, orden_id, fecha, cliente_nombre, matricula, iva_porcentaje,
                     base, iva, total, aviso_enviado)
SELECT 1, o.id, (o.terminada_en AT TIME ZONE 'Europe/Madrid')::date, c.nombre, v.matricula,
       21.00, 110.00, 23.10, 133.10, true
  FROM orden o
  JOIN vehiculo v ON v.id = o.vehiculo_id
  JOIN cliente c ON c.id = v.cliente_id
 WHERE o.numero = 9;

INSERT INTO linea_factura (factura_id, concepto, cantidad, precio_unitario, importe)
SELECT f.id, i.concepto, i.cantidad, i.precio_unitario, i.importe
  FROM factura f
  JOIN imputacion i ON i.orden_id = f.orden_id
 ORDER BY i.id;

UPDATE contador SET ultimo = (SELECT max(numero) FROM factura) WHERE nombre = 'factura';

COMMIT;

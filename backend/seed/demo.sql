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

COMMIT;

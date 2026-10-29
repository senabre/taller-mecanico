-- spec-005 · Las piezas se imputan en unidades enteras (research D2; data-model `imputacion`),
-- como en `linea_presupuesto` (0002). El CHECK de horas en múltiplos de 0,25 ya está en 0001.

ALTER TABLE imputacion
    ADD CONSTRAINT imputacion_piezas_enteras
        CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad));

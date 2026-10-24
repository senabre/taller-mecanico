-- spec-003 · Cantidades de las líneas de presupuesto (research D1; data-model `linea_presupuesto`):
-- mano de obra en múltiplos de 0,25 h y piezas en unidades enteras, como en `imputacion`.

ALTER TABLE linea_presupuesto
    ADD CONSTRAINT linea_presupuesto_horas_cuarto
        CHECK (tipo <> 'mano_obra' OR cantidad * 4 = trunc(cantidad * 4)),
    ADD CONSTRAINT linea_presupuesto_piezas_enteras
        CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad));

// Tipos comunes del contrato (specs/000-base-comun/contracts/api.md). Los tipos propios de cada
// spec (Orden, Presupuesto, Pieza, Pedido, Factura…) los añade cada spec.

/** Importe en euros como string decimal con punto: "133.10". */
export type Importe = string;
/** Fecha sin hora: "2026-10-20". */
export type Fecha = string;
/** Instante ISO 8601 con zona: "2026-10-16T10:32:00+02:00". */
export type Instante = string;

export type Rol = 'recepcion' | 'mecanico' | 'almacen' | 'jefe';

export type EstadoOrden =
	| 'recibida'
	| 'en_diagnostico'
	| 'en_reparacion'
	| 'en_espera_piezas'
	| 'pendiente_conformidad'
	| 'terminada'
	| 'facturada';

export type TipoLinea = 'mano_obra' | 'pieza';

export interface Yo {
	id: number;
	nombre: string;
	rol: Rol;
}

export interface Mecanico {
	id: number;
	nombre: string;
}

export interface OrdenResumen {
	id: number;
	numero: number;
	matricula: string;
	estado: EstadoOrden;
	mecanico: Mecanico;
	fecha_entrega_prometida: Fecha;
	retrasada: boolean;
}

export interface Linea {
	id: number;
	tipo: TipoLinea;
	concepto: string;
	pieza_id: number | null;
	cantidad: string;
	precio_unitario: Importe;
	importe: Importe;
}

export interface Totales {
	base: Importe;
	iva_porcentaje: Importe;
	iva: Importe;
	total: Importe;
}

export interface RespuestaError {
	error: string;
	detalle?: Record<string, unknown>;
}

// --- spec-001 · Citas ---

/** GET /api/publico/vehiculos/{matricula}: nunca trae datos del cliente ni la fecha de la cita. */
export interface VehiculoPublico {
	matricula: string;
	marca: string;
	modelo: string;
	cita_pendiente: boolean;
}

/** POST /api/publico/citas: `vehiculo` y `cliente` solo si la matrícula es nueva. */
export interface PeticionCita {
	matricula: string;
	fecha: Fecha;
	motivo: string;
	vehiculo?: { marca: string; modelo: string };
	cliente?: { nombre: string; telefono: string; email: string };
}

export interface CitaCreada {
	fecha: Fecha;
	motivo: string;
	matricula: string;
}

/** GET /api/citas: la lista de recepción nunca incluye el correo (spec-001 FR-009). */
export interface CitaDelDia {
	id: number;
	matricula: string;
	marca: string;
	modelo: string;
	motivo: string;
	cliente: { nombre: string; telefono: string };
	orden_id: number | null;
	/** Número de la orden de la cita, para «Orden Nº N» en la lista (spec-002 FR-008). */
	orden_numero: number | null;
}

// --- spec-002 · Órdenes ---

/** POST /api/ordenes. `km` puede ir como número o como texto con cifras (contrato común). */
export interface PeticionOrden {
	cita_id: number;
	km: number | string;
	danos: string;
	mecanico_id: number;
	fecha_entrega_prometida: Fecha;
}

/**
 * GET /api/ordenes/{id}: la ficha de la orden. Los campos de spec-002 más los de cada sección:
 * `presupuestos` (spec-003), `total_presupuestado`, `coste_acumulado` e `imputaciones` (spec-005),
 * `pedidos` (spec-006) y `factura_id` (spec-007).
 */
export interface Orden extends OrdenResumen {
	cita: { fecha: Fecha; motivo: string };
	vehiculo: { marca: string; modelo: string };
	km: number;
	danos: string;
	diagnostico: string | null;
	total_presupuestado: Importe;
	coste_acumulado: Importe;
	presupuestos: PresupuestoResumen[];
	imputaciones: Linea[];
	pedidos: Pedido[];
	factura_id: number | null;
}

// --- spec-003 · Diagnóstico y presupuesto ---

export type EstadoPresupuesto = 'borrador' | 'emitido' | 'aceptado' | 'rechazado';
export type TipoPresupuesto = 'inicial' | 'ampliacion';

export interface Presupuesto extends Totales {
	id: number;
	orden_id: number;
	tipo: TipoPresupuesto;
	estado: EstadoPresupuesto;
	/** Emitido y pasada su fecha de caducidad (no es un estado guardado). */
	caducado: boolean;
	lineas: Linea[];
	emitido_en: Instante | null;
	caduca_el: Fecha | null;
	respondido_en: Instante | null;
	enlace: string | null;
	/** Solo en la respuesta de emitir: si el correo con el enlace salió bien. */
	aviso_enviado?: boolean;
}

export interface PresupuestoResumen {
	id: number;
	tipo: TipoPresupuesto;
	estado: EstadoPresupuesto;
	caducado: boolean;
	total: Importe;
	emitido_en: Instante | null;
	caduca_el: Fecha | null;
	/** Cuándo respondió el cliente (spec-004 FR-004): null si aún no ha respondido. */
	respondido_en: Instante | null;
}

// --- spec-005 · Imputaciones ---

/** Imputaciones de una orden con su acumulado y lo presupuestado (suma de los aceptados). */
export interface ResumenImputaciones {
	orden_id: number;
	estado: EstadoOrden;
	coste_acumulado: Importe;
	total_presupuestado: Importe;
	imputaciones: Linea[];
}

/** Lo que se envía para añadir una línea de presupuesto o una imputación. */
export type NuevaLinea =
	| { tipo: 'mano_obra'; concepto: string; cantidad: string }
	| { tipo: 'pieza'; pieza_id: number; cantidad: string };

// --- spec-006 · Almacén y pedidos ---

export interface Pieza {
	id: number;
	referencia: string;
	descripcion: string;
	precio_venta: Importe;
	stock: number;
}

export interface Proveedor {
	id: number;
	nombre: string;
}

export type EstadoPedido = 'pendiente' | 'recibido';

export interface Pedido {
	id: number;
	proveedor: Proveedor;
	pieza: { id: number; referencia: string; descripcion: string };
	cantidad: number;
	orden: { id: number; numero: number };
	estado: EstadoPedido;
	pedido_en: Instante;
	recibido_en: Instante | null;
}

/** Evento `orden` de GET /api/eventos (SSE). */
export interface EventoOrden {
	id: number;
	estado: EstadoOrden;
}

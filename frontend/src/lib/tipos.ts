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

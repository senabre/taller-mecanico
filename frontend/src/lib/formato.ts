// Formato español de todo lo que se ve en pantalla (FR-006, research R4 y R12):
// importes «1.234,56 €», fechas dd/mm/aaaa y horas HH:mm en hora de Madrid.
import type { EstadoOrden, Fecha, Importe, Instante, Rol } from './tipos';

// Sin useGrouping: 'always', es-ES escribe «1234,56 €» (sin punto de miles en 4 cifras).
const formatoEuros = new Intl.NumberFormat('es-ES', {
	style: 'currency',
	currency: 'EUR',
	useGrouping: 'always'
});

const formatoNumero = new Intl.NumberFormat('es-ES', {
	useGrouping: 'always',
	maximumFractionDigits: 2
});

const formatoMadrid = new Intl.DateTimeFormat('es-ES', {
	timeZone: 'Europe/Madrid',
	day: '2-digit',
	month: '2-digit',
	year: 'numeric',
	hour: '2-digit',
	minute: '2-digit',
	hourCycle: 'h23'
});

export function euros(importe: Importe | number): string {
	return formatoEuros.format(Number(importe));
}

/** "2026-10-20" → "20/10/2026", sin pasar por Date para no mover el día por la zona horaria. */
export function fecha(f: Fecha): string {
	const [anio, mes, dia] = f.split('-');
	return `${dia}/${mes}/${anio}`;
}

function partesEnMadrid(i: Instante): Record<string, string> {
	const partes: Record<string, string> = {};
	for (const p of formatoMadrid.formatToParts(new Date(i))) partes[p.type] = p.value;
	return partes;
}

export function hora(i: Instante): string {
	const p = partesEnMadrid(i);
	return `${p.hour}:${p.minute}`;
}

export function fechaHora(i: Instante): string {
	const p = partesEnMadrid(i);
	return `${p.day}/${p.month}/${p.year} ${p.hour}:${p.minute}`;
}

/** Km, horas y unidades: 85000 → «85.000», "1.25" → «1,25». */
export function numero(n: number | string): string {
	return formatoNumero.format(Number(n));
}

export const NOMBRE_ESTADO: Record<EstadoOrden, string> = {
	recibida: 'Recibida',
	en_diagnostico: 'En diagnóstico',
	en_reparacion: 'En reparación',
	en_espera_piezas: 'En espera de piezas',
	pendiente_conformidad: 'Pendiente de conformidad',
	terminada: 'Terminada',
	facturada: 'Facturada'
};

/** Los estados de una orden abierta (todos menos facturada), en el orden de la máquina de estados. */
export const ESTADOS_ABIERTOS: EstadoOrden[] = [
	'recibida',
	'en_diagnostico',
	'en_reparacion',
	'en_espera_piezas',
	'pendiente_conformidad',
	'terminada'
];

export const NOMBRE_ROL: Record<Rol, string> = {
	recepcion: 'Recepción',
	mecanico: 'Mecánico',
	almacen: 'Almacén',
	jefe: 'Jefe de taller'
};

/** Página a la que va cada rol al entrar. */
export const INICIO_ROL: Record<Rol, string> = {
	recepcion: '/citas',
	mecanico: '/ordenes',
	almacen: '/almacen',
	jefe: '/tablero'
};

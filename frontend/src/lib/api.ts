// Llamadas a la API con manejo de errores {"error": "..."} en español (contracts/api.md).
import { goto } from '$app/navigation';
import type { RespuestaError } from './tipos';

export class ErrorApi extends Error {
	constructor(
		mensaje: string,
		public estado: number,
		public detalle?: Record<string, unknown>
	) {
		super(mensaje);
		this.name = 'ErrorApi';
	}
}

type Metodo = 'GET' | 'POST' | 'PUT' | 'DELETE';

/** Páginas que se usan sin entrar (FR-003): entrar, pedir cita y responder al presupuesto. */
export function esRutaPublica(ruta: string): boolean {
	return ruta === '/entrar' || ruta === '/cita' || ruta.startsWith('/presupuesto/');
}

export async function api<T>(metodo: Metodo, ruta: string, cuerpo?: unknown): Promise<T> {
	let respuesta: Response;
	try {
		respuesta = await fetch('/api' + ruta, {
			method: metodo,
			credentials: 'same-origin',
			headers: cuerpo === undefined ? undefined : { 'Content-Type': 'application/json' },
			body: cuerpo === undefined ? undefined : JSON.stringify(cuerpo)
		});
	} catch {
		throw new ErrorApi('No se pudo conectar con el servidor', 0);
	}

	if (respuesta.status === 204) return undefined as T;

	if (!respuesta.ok) {
		// Sesión caducada (12 h sin actividad): de vuelta a /entrar si la página no es pública.
		if (respuesta.status === 401 && ruta !== '/sesion' && !esRutaPublica(location.pathname)) {
			await goto('/entrar');
		}
		let error: RespuestaError | undefined;
		try {
			error = await respuesta.json();
		} catch {
			error = undefined;
		}
		throw new ErrorApi(
			error?.error ?? 'Ha ocurrido un error inesperado',
			respuesta.status,
			error?.detalle
		);
	}

	return (await respuesta.json()) as T;
}

export const get = <T>(ruta: string) => api<T>('GET', ruta);
export const post = <T>(ruta: string, cuerpo?: unknown) => api<T>('POST', ruta, cuerpo);
export const put = <T>(ruta: string, cuerpo?: unknown) => api<T>('PUT', ruta, cuerpo);
export const del = <T>(ruta: string) => api<T>('DELETE', ruta);

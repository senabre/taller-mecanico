// SPA sin SSR (research R2): el backend sirve el index.html y todo se pinta en el navegador.
import type { Yo } from '$lib/tipos';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = false;

// Quién ha entrado. Sin sesión (401) devuelve null y el layout decide si lleva a /entrar; por eso
// no se usa api.ts, que redirigiría por su cuenta.
export const load: LayoutLoad = async ({ fetch }) => {
	const respuesta = await fetch('/api/yo');
	if (respuesta.status === 401) return { yo: null };
	if (!respuesta.ok) throw new Error('No se pudo comprobar la sesión');
	return { yo: (await respuesta.json()) as Yo };
};

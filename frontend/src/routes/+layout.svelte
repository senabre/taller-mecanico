<script lang="ts">
	import '../app.css';
	import { goto, invalidateAll } from '$app/navigation';
	import { page } from '$app/state';
	import { del, esRutaPublica } from '$lib/api';
	import { NOMBRE_ROL } from '$lib/formato';
	import type { Rol } from '$lib/tipos';

	let { data, children } = $props();

	// Menú por rol (FR-001, SC-002). El jefe ve todas las entradas (FR-002).
	const MENU: { texto: string; ruta: string; roles: Rol[] }[] = [
		{ texto: 'Tablero', ruta: '/tablero', roles: ['jefe'] },
		{ texto: 'Citas', ruta: '/citas', roles: ['recepcion', 'jefe'] },
		{ texto: 'Órdenes', ruta: '/ordenes', roles: ['recepcion', 'jefe'] },
		{ texto: 'Mis órdenes', ruta: '/ordenes', roles: ['mecanico'] },
		{ texto: 'Almacén', ruta: '/almacen', roles: ['almacen', 'jefe'] },
		{ texto: 'Pedidos', ruta: '/pedidos', roles: ['almacen', 'jefe'] },
		{ texto: 'Historial', ruta: '/historial', roles: ['recepcion', 'mecanico', 'almacen', 'jefe'] }
	];

	// Pedir cita, responder al presupuesto y entrar se usan sin sesión (FR-003).
	const publica = $derived(esRutaPublica(page.url.pathname));
	const menu = $derived(data.yo ? MENU.filter((e) => e.roles.includes(data.yo!.rol)) : []);

	$effect(() => {
		if (!publica && !data.yo) goto('/entrar', { replaceState: true });
	});

	async function salir() {
		try {
			await del('/sesion');
		} finally {
			await invalidateAll();
			await goto('/entrar');
		}
	}
</script>

{#if publica}
	{@render children()}
{:else if data.yo}
	<header class="bg-surface-100-900 border-surface-200-800 border-b">
		<nav class="mx-auto flex max-w-6xl flex-wrap items-center gap-x-6 gap-y-2 px-4 py-3">
			<a href="/" class="text-lg font-bold">Talleres Rueda</a>
			<ul class="flex flex-wrap gap-4">
				{#each menu as entrada (entrada.texto)}
					<li>
						<a
							href={entrada.ruta}
							class="hover:underline"
							class:font-semibold={page.url.pathname.startsWith(entrada.ruta)}
						>
							{entrada.texto}
						</a>
					</li>
				{/each}
			</ul>
			<div class="ml-auto flex items-center gap-3">
				<span class="text-sm">{data.yo.nombre} · {NOMBRE_ROL[data.yo.rol]}</span>
				<button type="button" class="btn preset-tonal btn-sm" onclick={salir}>Salir</button>
			</div>
		</nav>
	</header>
	<main class="mx-auto max-w-6xl px-4 py-6">
		{@render children()}
	</main>
{/if}

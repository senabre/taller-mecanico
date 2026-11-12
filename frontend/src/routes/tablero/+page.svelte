<script lang="ts">
	// Tablero de órdenes abiertas (spec-008, HU-08): solo para el jefe (FR-006). Solo consulta:
	// las tarjetas no son enlaces ni se arrastran (principio III). Diseño en research D7.
	import { ErrorApi, get } from '$lib/api';
	import { conectarEventos } from '$lib/eventos';
	import { fecha, NOMBRE_ESTADO } from '$lib/formato';
	import { programarMedianoche } from '$lib/medianoche';
	import { crearRecarga } from '$lib/recarga';
	import type { Tablero } from '$lib/tipos';

	let tablero: Tablero | null = $state(null);
	let error = $state('');
	let prohibido = $state(false);
	let sinConexion = $state(false);

	/** Pide el tablero entero. Con un 403 solo queda el mensaje del servidor (D4). */
	async function cargar() {
		try {
			tablero = await get<Tablero>('/tablero');
			error = '';
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 403) prohibido = true;
			// Sin conexión, api.ts lanza un ErrorApi con estado 0. Con un 401, api.ts ya lleva a /entrar.
			error =
				e instanceof ErrorApi && e.estado !== 0 ? e.message : 'No se pudo cargar el tablero';
		}
	}

	// Una sola petición a la vez, más una si llegan avisos mientras tanto (D5).
	const pedir = crearRecarga(cargar);

	$effect(() => {
		let activa = true;
		let cerrarEventos: (() => void) | undefined;
		let cancelarMedianoche: (() => void) | undefined;

		(async () => {
			await cargar();
			// En todos los casos salvo con un 403 (E2, N2): si la primera carga falló por la red, el
			// primer onopen la repite y el tablero se recupera solo.
			if (!activa || prohibido) return;
			cerrarEventos = conectarEventos({
				// El tablero entero en cada apertura: la primera y cada vez que se reabre tras un error (Q3).
				alAbrir: () => {
					sinConexion = false;
					pedir();
				},
				alEvento: pedir,
				// Un error no pide el tablero, salvo con la conexión cerrada: una vez, para que un 401
				// lleve a /entrar (E1, N3).
				alError: (cerrada) => {
					sinConexion = true;
					if (cerrada) pedir();
				}
			});
			cancelarMedianoche = programarMedianoche(pedir);
		})();

		return () => {
			activa = false;
			cerrarEventos?.();
			cancelarMedianoche?.();
		};
	});
</script>

<svelte:head><title>Tablero · Talleres Rueda</title></svelte:head>

<div class="space-y-4">
	<h1 class="h2">Tablero</h1>

	{#if error}
		<p class="text-error-500" role="alert">{error}</p>
	{/if}
	{#if sinConexion}
		<p class="card preset-tonal-warning p-3" role="status">Sin conexión, reintentando…</p>
	{/if}

	{#if tablero}
		<!-- Carga por mecánico (FR-003): los activos aunque tengan 0; los de baja, solo con órdenes. -->
		<section aria-labelledby="carga" class="space-y-2">
			<h2 id="carga" class="font-semibold">Carga por mecánico</h2>
			<!-- «Luis 5 · Marta 3», como lo describe el quickstart §8. -->
			<ul class="flex flex-wrap items-center gap-2">
				{#each tablero.carga_mecanicos as mecanico, i (mecanico.id)}
					{#if i > 0}<li aria-hidden="true">·</li>{/if}
					<li class="badge preset-tonal text-base">
						{mecanico.nombre}{#if !mecanico.activo}&nbsp;(de baja){/if}
						<strong>{mecanico.ordenes_abiertas}</strong>
					</li>
				{/each}
			</ul>
		</section>

		<!-- ≥ xl: rejilla de 6 columnas. Por debajo, una tira que se desplaza en horizontal dentro de
		     su contenedor, para que la página no se mueva de lado a 390 px (D7). -->
		<div
			class="flex snap-x snap-mandatory gap-4 overflow-x-auto pb-2 xl:grid xl:grid-cols-6 xl:overflow-x-visible"
		>
			{#each tablero.columnas as columna (columna.estado)}
				<section
					class="bg-surface-100-900 flex w-[85%] shrink-0 snap-start flex-col rounded-lg sm:w-72 xl:w-auto"
					aria-label={NOMBRE_ESTADO[columna.estado]}
				>
					<h2 class="border-surface-200-800 border-b px-3 py-2 font-semibold">
						{NOMBRE_ESTADO[columna.estado]}
					</h2>
					{#if columna.ordenes.length === 0}
						<p class="text-surface-600-400 px-3 py-4 text-sm">Sin órdenes</p>
					{:else}
						<ul class="max-h-[calc(100dvh-15rem)] space-y-2 overflow-y-auto p-2">
							{#each columna.ordenes as orden (orden.id)}
								<!-- Retrasada (FR-004): borde de 2 px y la insignia con texto, que se ve también
								     sin color (SC-001). -->
								<li
									class={[
										'card bg-surface-50-950 space-y-1 p-3',
										orden.retrasada
											? 'border-error-500 border-2'
											: 'preset-outlined-surface-200-800'
									]}
								>
									<div class="flex items-center justify-between gap-2">
										<p class="text-sm">Nº {orden.numero}</p>
										{#if orden.retrasada}
											<span class="badge preset-filled-error-500">Retrasada</span>
										{/if}
									</div>
									<p class="font-mono text-xl font-bold">{orden.matricula}</p>
									<p class="text-sm">{orden.mecanico.nombre}</p>
									<p class="text-sm">Entrega {fecha(orden.fecha_entrega_prometida)}</p>
								</li>
							{/each}
						</ul>
					{/if}
				</section>
			{/each}
		</div>
	{/if}
</div>

<script lang="ts">
	// Factura simplificada para ver e imprimir (spec-007 US1, FR-003). «Imprimir» usa la impresión
	// del navegador, que también permite guardarla como PDF; al imprimir solo sale la factura.
	import { page } from '$app/state';
	import { ErrorApi, get } from '$lib/api';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import Totales from '$lib/componentes/Totales.svelte';
	import { fecha } from '$lib/formato';
	import type { Factura } from '$lib/tipos';

	let factura = $state<Factura | null>(null);
	let error = $state('');

	$effect(() => {
		const id = page.params.id;
		get<Factura>(`/facturas/${id}`)
			.then((f) => {
				factura = f;
				error = '';
			})
			.catch((e) => {
				error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
			});
	});
</script>

<svelte:head>
	<title>{factura ? `Factura nº ${factura.numero}` : 'Factura'} · Talleres Rueda</title>
</svelte:head>

{#if error}
	<p class="text-error-500" role="alert">{error}</p>
{:else if factura}
	<div class="no-imprimir mb-6 flex justify-end">
		<button type="button" class="btn preset-filled-primary-500" onclick={() => window.print()}>
			Imprimir
		</button>
	</div>

	<article class="factura mx-auto max-w-3xl space-y-6">
		<header class="flex flex-wrap justify-between gap-4">
			<div class="flex items-center gap-4">
				<!-- Solo si el taller tiene logo configurado (TALLER_LOGO); si no, ni imagen ni hueco. -->
				{#if factura.taller.logo}
					<img
						class="logo"
						src={factura.taller.logo}
						alt="Logo de {factura.taller.nombre}"
					/>
				{/if}
				<div>
					<p class="text-lg font-bold">{factura.taller.nombre}</p>
					<p>NIF {factura.taller.nif}</p>
					<p>{factura.taller.direccion}</p>
				</div>
			</div>
			<div class="text-right">
				<h1 class="h3">Factura nº {factura.numero}</h1>
				<p>Fecha: {fecha(factura.fecha)}</p>
				<p class="text-sm">Factura simplificada</p>
			</div>
		</header>

		<section>
			<p>Cliente: <strong>{factura.cliente_nombre}</strong></p>
			<p>Matrícula: <strong>{factura.matricula}</strong></p>
		</section>

		<TablaLineas lineas={factura.lineas} />
		<Totales totales={factura} />
	</article>
{/if}

<style>
	.logo {
		max-height: 2cm;
		max-width: 5cm;
		object-fit: contain;
	}

	/* Al imprimir, solo la factura: sin el menú de la aplicación ni el botón. */
	@media print {
		:global(header:has(> nav)),
		.no-imprimir {
			display: none !important;
		}
		/* El margen lo pone la factura, no la página: así el navegador no tiene sitio para sus
		   encabezados y pies (fecha, título, dirección localhost… y número de página). */
		:global(main) {
			padding: 0 !important;
			max-width: none !important;
		}
		.factura {
			max-width: none;
			padding: 1.5cm 1.8cm;
		}
	}

	@page {
		margin: 0;
	}
</style>

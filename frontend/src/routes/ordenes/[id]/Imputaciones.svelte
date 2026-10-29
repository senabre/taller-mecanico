<script lang="ts">
	// Sección «Imputaciones» de la ficha de la orden (spec-005). La ficha (`+page.svelte`, spec-002)
	// le pasa la orden, su resumen (`resumen_imputaciones`) y si el usuario es su mecánico o el jefe.
	// No hay «Quitar»: las imputaciones no se anulan ni se corrigen en la v1.
	import { ErrorApi, post } from '$lib/api';
	import FormularioLinea from '$lib/componentes/FormularioLinea.svelte';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import { euros, NOMBRE_ESTADO } from '$lib/formato';
	import type { NuevaLinea, ResumenImputaciones } from '$lib/tipos';

	let {
		resumen: inicial,
		puedeEditar,
		onCambio
	}: {
		resumen: ResumenImputaciones;
		puedeEditar: boolean;
		onCambio: () => void;
	} = $props();

	// Cada imputación devuelve el resumen al día; la ficha lo vuelve a pasar al recargar.
	let resumen = $state<ResumenImputaciones | null>(null);
	const actual = $derived(resumen ?? inicial);
	$effect(() => {
		if (inicial) resumen = null;
	});

	let error = $state('');

	const enReparacion = $derived(actual.estado === 'en_reparacion');
	const pasado = $derived(Number(actual.coste_acumulado) > Number(actual.total_presupuestado));

	async function imputar(linea: NuevaLinea) {
		resumen = await post<ResumenImputaciones>(`/ordenes/${actual.orden_id}/imputaciones`, linea);
		onCambio();
	}

	async function terminar() {
		if (
			!confirm(
				'¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas.'
			)
		)
			return;
		error = '';
		try {
			resumen = await post<ResumenImputaciones>(`/ordenes/${actual.orden_id}/terminar`);
			onCambio();
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		}
	}
</script>

<section class="space-y-4">
	<div class="flex flex-wrap items-center gap-4">
		<h2 class="h4">Imputaciones</h2>
		<p class:text-error-600-400={pasado} class:font-semibold={pasado}>
			Acumulado {euros(actual.coste_acumulado)} de {euros(actual.total_presupuestado)} presupuestados
		</p>
		{#if puedeEditar && enReparacion && actual.imputaciones.length > 0}
			<button type="button" class="btn preset-filled-primary-500 btn-sm ml-auto" onclick={terminar}>
				Terminar reparación
			</button>
		{/if}
	</div>

	{#if error}
		<p class="text-error-500" role="alert">{error}</p>
	{/if}

	{#if actual.estado === 'pendiente_conformidad'}
		<p class="text-warning-700-300" role="alert">
			El coste supera en más de un 10 % lo presupuestado: prepara una ampliación del presupuesto.
		</p>
	{:else if !enReparacion && actual.estado !== 'terminada' && actual.estado !== 'facturada'}
		<p>La orden está «{NOMBRE_ESTADO[actual.estado]}»: solo se imputa en órdenes en reparación.</p>
	{/if}

	<TablaLineas lineas={actual.imputaciones} />

	{#if puedeEditar && enReparacion}
		<FormularioLinea titulo="Nueva imputación" textoBoton="Imputar" onenviar={imputar} />
	{/if}
</section>

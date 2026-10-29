<script lang="ts">
	// Historial de un vehículo por matrícula: sus reparaciones facturadas, de la más reciente a la
	// más antigua (spec-007 US3, FR-007). Lo puede consultar cualquier empleado.
	import { ErrorApi, get } from '$lib/api';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import { euros, fecha, numero } from '$lib/formato';
	import type { Historial } from '$lib/tipos';

	let matricula = $state('');
	let historial = $state<Historial | null>(null);
	let sinHistorial = $state(false);
	let buscando = $state(false);
	let error = $state('');

	async function buscar(evento: SubmitEvent) {
		evento.preventDefault();
		buscando = true;
		error = '';
		historial = null;
		sinHistorial = false;
		try {
			const h = await get<Historial>(
				`/vehiculos/${encodeURIComponent(matricula.trim())}/historial`
			);
			historial = h;
			sinHistorial = h.reparaciones.length === 0;
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 404) sinHistorial = true;
			else error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		} finally {
			buscando = false;
		}
	}
</script>

<svelte:head><title>Historial · Talleres Rueda</title></svelte:head>

<div class="space-y-6">
	<h1 class="h2">Historial del vehículo</h1>

	<form class="flex max-w-md flex-wrap items-end gap-3" onsubmit={buscar}>
		<label class="label grow">
			<span class="label-text">Matrícula</span>
			<input
				class="input"
				name="matricula"
				placeholder="1234 BCD"
				autocomplete="off"
				required
				bind:value={matricula}
			/>
		</label>
		<button type="submit" class="btn preset-filled-primary-500" disabled={buscando}>
			{buscando ? 'Buscando…' : 'Buscar'}
		</button>
	</form>

	{#if error}
		<p class="text-error-500" role="alert">{error}</p>
	{:else if sinHistorial}
		<p>No hay historial para esa matrícula.</p>
	{/if}

	{#if historial && historial.reparaciones.length > 0}
		<p class="text-lg">
			<strong>{historial.matricula}</strong> · {historial.marca} {historial.modelo}
		</p>
		{#each historial.reparaciones as r (r.factura_numero)}
			<section class="card bg-surface-100-900 space-y-3 p-4">
				<h2 class="h5">
					{fecha(r.fecha)} · Orden {r.orden_numero} · Factura nº {r.factura_numero} ·
					{numero(r.km)} km
				</h2>
				{#if r.diagnostico}
					<p>Diagnóstico: {r.diagnostico}</p>
				{/if}
				<TablaLineas lineas={r.lineas} />
				<p class="text-right font-bold">Total {euros(r.total)}</p>
			</section>
		{/each}
	{/if}
</div>

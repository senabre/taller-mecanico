<script lang="ts">
	// «Órdenes» (recepción y jefe) y «Mis órdenes» (mecánicos): la exigen el menú común y el
	// quickstart §3, y está aquí porque spec-002 crea las órdenes (hallazgo E1).
	import { page } from '$app/state';
	import { ErrorApi, get } from '$lib/api';
	import { fecha as fechaEs, NOMBRE_ESTADO } from '$lib/formato';
	import type { OrdenResumen } from '$lib/tipos';

	const soloMias = $derived(page.data.yo?.rol === 'mecanico');
	const titulo = $derived(soloMias ? 'Mis órdenes' : 'Órdenes');

	let ordenes: OrdenResumen[] = $state([]);
	let cargada = $state(false);
	let error = $state('');

	async function cargar(mias: boolean) {
		try {
			ordenes = await get<OrdenResumen[]>(mias ? '/ordenes?mias=true' : '/ordenes');
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'No se pudo cargar la lista de órdenes';
		} finally {
			cargada = true;
		}
	}

	$effect(() => {
		cargar(soloMias);
	});
</script>

<svelte:head><title>{titulo} · Talleres Rueda</title></svelte:head>

<div class="space-y-6">
	<h1 class="h2">{titulo}</h1>

	{#if error}
		<p class="text-error-500" role="alert">{error}</p>
	{:else if cargada && ordenes.length === 0}
		<p>No hay órdenes</p>
	{:else if cargada}
		<div class="table-wrap">
			<table class="table">
				<thead>
					<tr>
						<th>Número</th>
						<th>Matrícula</th>
						<th>Estado</th>
						<th>Mecánico</th>
						<th>Entrega prometida</th>
					</tr>
				</thead>
				<tbody>
					{#each ordenes as orden (orden.id)}
						<tr>
							<td><a class="anchor" href={`/ordenes/${orden.id}`}>{orden.numero}</a></td>
							<td class="font-mono">{orden.matricula}</td>
							<td>{NOMBRE_ESTADO[orden.estado]}</td>
							<td>{orden.mecanico.nombre}</td>
							<td>
								{fechaEs(orden.fecha_entrega_prometida)}
								{#if orden.retrasada}
									<span class="text-error-500 font-semibold">Retrasada</span>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

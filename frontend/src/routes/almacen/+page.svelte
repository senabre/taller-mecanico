<script lang="ts">
	// Almacén: referencias de piezas con su stock y alta de referencias (spec-006 US1).
	import { ErrorApi, get, post } from '$lib/api';
	import { euros } from '$lib/formato';
	import type { Pieza } from '$lib/tipos';

	let piezas: Pieza[] = $state([]);
	let errorLista = $state('');

	let referencia = $state('');
	let descripcion = $state('');
	let precio = $state('');
	// Campo numérico: vacío es null (sin stock inicial → el servidor pone 0).
	let stock: number | null = $state(null);
	let enviando = $state(false);
	let error = $state('');
	let aviso = $state('');

	async function cargar() {
		try {
			piezas = await get<Pieza[]>('/piezas');
			errorLista = '';
		} catch (e) {
			errorLista = e instanceof ErrorApi ? e.message : 'No se pudo cargar la lista de piezas';
		}
	}

	$effect(() => {
		cargar();
	});

	async function guardar(evento: SubmitEvent) {
		evento.preventDefault();
		enviando = true;
		error = '';
		aviso = '';
		try {
			// El precio se escribe con coma («12,50») y viaja con punto («12.50»), como dice el contrato.
			const cuerpo: Record<string, unknown> = {
				referencia,
				descripcion,
				precio_venta: precio.trim().replace(',', '.')
			};
			if (stock != null) cuerpo.stock = stock;
			const nueva = await post<Pieza>('/piezas', cuerpo);
			piezas = [...piezas, nueva].sort((a, b) => (a.referencia < b.referencia ? -1 : 1));
			aviso = `Referencia ${nueva.referencia} dada de alta`;
			referencia = '';
			descripcion = '';
			precio = '';
			stock = null;
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		} finally {
			enviando = false;
		}
	}
</script>

<svelte:head><title>Almacén · Talleres Rueda</title></svelte:head>

<div class="space-y-8">
	<h1 class="h2">Almacén</h1>

	<section class="card bg-surface-100-900 max-w-3xl space-y-4 p-6">
		<h2 class="h4">Dar de alta una referencia</h2>
		<form class="grid gap-4 sm:grid-cols-2" onsubmit={guardar}>
			<label class="label">
				<span class="label-text">Referencia</span>
				<input class="input" name="referencia" required bind:value={referencia} />
			</label>
			<label class="label">
				<span class="label-text">Descripción</span>
				<input class="input" name="descripcion" required bind:value={descripcion} />
			</label>
			<label class="label">
				<span class="label-text">Precio de venta (€)</span>
				<input
					class="input"
					name="precio"
					inputmode="decimal"
					placeholder="25,00"
					required
					bind:value={precio}
				/>
			</label>
			<label class="label">
				<span class="label-text">Stock inicial (opcional)</span>
				<input
					class="input"
					name="stock"
					type="number"
					min="0"
					step="1"
					placeholder="0"
					bind:value={stock}
				/>
			</label>

			<div class="flex flex-wrap items-center gap-4 sm:col-span-2">
				<button type="submit" class="btn preset-filled-primary-500" disabled={enviando}>
					{enviando ? 'Guardando…' : 'Dar de alta'}
				</button>
				{#if error}
					<p class="text-error-500" role="alert">{error}</p>
				{:else if aviso}
					<p class="text-success-700-300" role="status">{aviso}</p>
				{/if}
			</div>
		</form>
	</section>

	<section class="space-y-4">
		<h2 class="h4">Referencias</h2>
		{#if errorLista}
			<p class="text-error-500" role="alert">{errorLista}</p>
		{:else if piezas.length === 0}
			<p>Todavía no hay referencias dadas de alta.</p>
		{:else}
			<div class="table-wrap">
				<table class="table">
					<thead>
						<tr>
							<th>Referencia</th>
							<th>Descripción</th>
							<th class="text-right">Precio</th>
							<th class="text-right">Stock</th>
						</tr>
					</thead>
					<tbody>
						{#each piezas as pieza (pieza.id)}
							<tr>
								<td class="font-mono">{pieza.referencia}</td>
								<td>{pieza.descripcion}</td>
								<td class="text-right">{euros(pieza.precio_venta)}</td>
								<td class="text-right">{pieza.stock}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</div>

<script lang="ts">
	// Formulario de una línea de mano de obra o de pieza (compartido: presupuesto de spec-003 e
	// imputaciones de spec-005). Quien lo usa hace la llamada en `onenviar`; si lanza un error, se
	// muestra su mensaje y el formulario no se limpia.
	import { ErrorApi, get } from '$lib/api';
	import type { NuevaLinea, Pieza } from '$lib/tipos';

	let {
		titulo,
		textoBoton,
		onenviar
	}: {
		titulo: string;
		textoBoton: string;
		onenviar: (linea: NuevaLinea) => Promise<void>;
	} = $props();

	const mensaje = (e: unknown) =>
		e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';

	let tipo: 'mano_obra' | 'pieza' = $state('mano_obra');
	let concepto = $state('');
	let horas = $state('');
	let unidades: number | null = $state(1);
	let busqueda = $state('');
	let resultados: Pieza[] = $state([]);
	let pieza = $state<Pieza | null>(null);
	let enviando = $state(false);
	let error = $state('');
	let temporizador: ReturnType<typeof setTimeout> | undefined;

	function buscar() {
		pieza = null;
		clearTimeout(temporizador);
		const texto = busqueda.trim();
		if (texto === '') {
			resultados = [];
			return;
		}
		temporizador = setTimeout(async () => {
			try {
				resultados = await get<Pieza[]>('/piezas?q=' + encodeURIComponent(texto));
			} catch (e) {
				error = mensaje(e);
			}
		}, 250);
	}

	function elegir(p: Pieza) {
		pieza = p;
		busqueda = `${p.referencia} · ${p.descripcion}`;
		resultados = [];
	}

	async function enviar(evento: SubmitEvent) {
		evento.preventDefault();
		error = '';
		if (tipo === 'pieza' && !pieza) {
			error = 'Busca y elige una referencia de la lista';
			return;
		}
		enviando = true;
		try {
			// Las horas se escriben con coma («1,5») y viajan con punto («1.5»), como dice el contrato.
			await onenviar(
				tipo === 'mano_obra'
					? { tipo, concepto, cantidad: horas.trim().replace(',', '.') }
					: { tipo, pieza_id: pieza!.id, cantidad: String(unidades ?? '') }
			);
			concepto = '';
			horas = '';
			unidades = 1;
			busqueda = '';
			pieza = null;
		} catch (e) {
			error = mensaje(e);
		} finally {
			enviando = false;
		}
	}
</script>

<form class="card bg-surface-100-900 grid gap-4 p-4 sm:grid-cols-2" onsubmit={enviar}>
	<h3 class="h5 sm:col-span-2">{titulo}</h3>
	<label class="label">
		<span class="label-text">Tipo</span>
		<select class="select" bind:value={tipo}>
			<option value="mano_obra">Mano de obra</option>
			<option value="pieza">Pieza</option>
		</select>
	</label>

	{#if tipo === 'mano_obra'}
		<label class="label">
			<span class="label-text">Concepto</span>
			<input class="input" name="concepto" required bind:value={concepto} />
		</label>
		<label class="label">
			<span class="label-text">Horas (en cuartos: 0,25 · 0,5 · 1,5…)</span>
			<input
				class="input"
				name="horas"
				inputmode="decimal"
				placeholder="1,5"
				required
				bind:value={horas}
			/>
		</label>
	{:else}
		<div class="label relative">
			<label class="label-text" for="pieza">Referencia</label>
			<input
				id="pieza"
				class="input"
				autocomplete="off"
				placeholder="Busca por referencia o descripción"
				required
				bind:value={busqueda}
				oninput={buscar}
			/>
			{#if resultados.length > 0}
				<ul
					class="card bg-surface-50-950 border-surface-200-800 absolute z-10 mt-1 max-h-60 w-full overflow-auto border"
				>
					{#each resultados as r (r.id)}
						<li>
							<button
								type="button"
								class="hover:bg-surface-200-800 w-full px-3 py-2 text-left"
								onclick={() => elegir(r)}
							>
								<span class="font-mono">{r.referencia}</span> · {r.descripcion}
								<span class="text-surface-600-400 text-sm">(stock {r.stock})</span>
							</button>
						</li>
					{/each}
				</ul>
			{/if}
		</div>
		<label class="label">
			<span class="label-text">Unidades</span>
			<input
				class="input"
				name="unidades"
				type="number"
				min="1"
				step="1"
				required
				bind:value={unidades}
			/>
		</label>
	{/if}

	<div class="flex flex-wrap items-center gap-4 sm:col-span-2">
		<button type="submit" class="btn preset-tonal-primary" disabled={enviando}>
			{enviando ? 'Guardando…' : textoBoton}
		</button>
		{#if error}
			<p class="text-error-500" role="alert">{error}</p>
		{/if}
	</div>
</form>

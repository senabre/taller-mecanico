<script lang="ts">
	// Sección «Presupuesto» de la ficha de la orden (spec-003 US2 y US3). La ficha (`+page.svelte`,
	// spec-002) le pasa la orden, sus presupuestos (`presupuestos_de_orden`) y si el usuario es su
	// mecánico o el jefe. El enlace y «Copiar enlace» los añade spec-004 (FR-008) en esta sección.
	import { del, ErrorApi, get, post } from '$lib/api';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import Totales from '$lib/componentes/Totales.svelte';
	import { fecha, fechaDe } from '$lib/formato';
	import type {
		EstadoOrden,
		EstadoPresupuesto,
		Linea,
		Pieza,
		Presupuesto,
		PresupuestoResumen
	} from '$lib/tipos';

	let {
		orden,
		presupuestos,
		puedeEditar,
		onCambio
	}: {
		orden: { id: number; estado: EstadoOrden };
		presupuestos: PresupuestoResumen[];
		puedeEditar: boolean;
		onCambio: () => void;
	} = $props();

	const NOMBRE_ESTADO: Record<EstadoPresupuesto, string> = {
		borrador: 'Borrador',
		emitido: 'Emitido',
		aceptado: 'Aceptado',
		rechazado: 'Rechazado'
	};

	const mensaje = (e: unknown) =>
		e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';

	let presupuesto = $state<Presupuesto | null>(null);
	let error = $state('');
	let avisoCorreo = $state('');

	// Se muestra el más reciente; la lista llega del más antiguo al más reciente.
	const ultimo = $derived(presupuestos.at(-1));
	const hayAbierto = $derived(
		presupuestos.some((p) => p.estado === 'borrador' || (p.estado === 'emitido' && !p.caducado))
	);
	const puedeCrear = $derived(
		puedeEditar &&
			!hayAbierto &&
			(orden.estado === 'en_diagnostico' || orden.estado === 'pendiente_conformidad')
	);
	const enBorrador = $derived(puedeEditar && presupuesto?.estado === 'borrador');

	$effect(() => {
		const id = ultimo?.id;
		if (id === undefined) {
			presupuesto = null;
			return;
		}
		get<Presupuesto>(`/presupuestos/${id}`)
			.then((p) => (presupuesto = p))
			.catch((e) => (error = mensaje(e)));
	});

	async function crear() {
		error = '';
		try {
			presupuesto = await post<Presupuesto>(`/ordenes/${orden.id}/presupuestos`);
			onCambio();
		} catch (e) {
			error = mensaje(e);
		}
	}

	// --- Añadir línea ---
	let tipo: 'mano_obra' | 'pieza' = $state('mano_obra');
	let concepto = $state('');
	let horas = $state('');
	let unidades: number | null = $state(1);
	let busqueda = $state('');
	let resultados: Pieza[] = $state([]);
	let pieza: Pieza | null = $state(null);
	let enviando = $state(false);
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

	async function anadir(evento: SubmitEvent) {
		evento.preventDefault();
		if (!presupuesto) return;
		error = '';
		if (tipo === 'pieza' && !pieza) {
			error = 'Busca y elige una referencia de la lista';
			return;
		}
		enviando = true;
		try {
			// Las horas se escriben con coma («1,5») y viajan con punto («1.5»), como dice el contrato.
			const linea =
				tipo === 'mano_obra'
					? { tipo, concepto, cantidad: horas.trim().replace(',', '.') }
					: { tipo, pieza_id: pieza!.id, cantidad: String(unidades ?? '') };
			presupuesto = await post<Presupuesto>(`/presupuestos/${presupuesto.id}/lineas`, linea);
			concepto = '';
			horas = '';
			unidades = 1;
			busqueda = '';
			pieza = null;
			onCambio();
		} catch (e) {
			error = mensaje(e);
		} finally {
			enviando = false;
		}
	}

	async function quitar(linea: Linea) {
		if (!presupuesto) return;
		error = '';
		try {
			presupuesto = await del<Presupuesto>(`/presupuestos/${presupuesto.id}/lineas/${linea.id}`);
			onCambio();
		} catch (e) {
			error = mensaje(e);
		}
	}

	async function emitir() {
		if (!presupuesto) return;
		if (!confirm('Una vez emitido no se podrá modificar. ¿Emitir el presupuesto?')) return;
		error = '';
		avisoCorreo = '';
		try {
			const emitido = await post<Presupuesto>(`/presupuestos/${presupuesto.id}/emitir`);
			presupuesto = emitido;
			if (emitido.aviso_enviado === false) avisoCorreo = 'No se pudo enviar el correo al cliente';
			onCambio();
		} catch (e) {
			error = mensaje(e);
		}
	}
</script>

<section class="space-y-4">
	<div class="flex flex-wrap items-center gap-4">
		<h2 class="h4">Presupuesto</h2>
		{#if presupuesto}
			<span class="badge preset-tonal">
				{presupuesto.tipo === 'ampliacion' ? 'Ampliación' : 'Inicial'} ·
				{NOMBRE_ESTADO[presupuesto.estado]}
			</span>
		{/if}
		{#if puedeCrear}
			<button type="button" class="btn preset-filled-primary-500 btn-sm ml-auto" onclick={crear}>
				Nuevo presupuesto
			</button>
		{/if}
	</div>

	{#if error}
		<p class="text-error-500" role="alert">{error}</p>
	{/if}

	{#if !presupuesto}
		<p>Esta orden todavía no tiene presupuesto.</p>
	{:else}
		{#if presupuesto.emitido_en}
			<p>
				Emitido el {fechaDe(presupuesto.emitido_en)} ·
				{#if presupuesto.caducado && presupuesto.caduca_el}
					<strong>Caducado el {fecha(presupuesto.caduca_el)}</strong>
				{:else if presupuesto.caduca_el}
					Válido hasta {fecha(presupuesto.caduca_el)}
				{/if}
			</p>
		{/if}
		{#if avisoCorreo}
			<p class="text-warning-700-300" role="alert">{avisoCorreo}</p>
		{/if}

		<TablaLineas lineas={presupuesto.lineas} onquitar={enBorrador ? quitar : undefined} />
		<Totales totales={presupuesto} />

		{#if enBorrador}
			<form class="card bg-surface-100-900 grid gap-4 p-4 sm:grid-cols-2" onsubmit={anadir}>
				<h3 class="h5 sm:col-span-2">Añadir línea</h3>
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
						{enviando ? 'Añadiendo…' : 'Añadir línea'}
					</button>
					{#if presupuesto.lineas.length > 0}
						<button type="button" class="btn preset-filled-primary-500 ml-auto" onclick={emitir}>
							Emitir presupuesto
						</button>
					{/if}
				</div>
			</form>
		{/if}
	{/if}
</section>

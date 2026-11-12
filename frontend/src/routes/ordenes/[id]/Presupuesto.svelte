<script lang="ts">
	// Sección «Presupuesto» de la ficha de la orden (spec-003 US2 y US3). La ficha (`+page.svelte`,
	// spec-002) le pasa la orden, sus presupuestos (`presupuestos_de_orden`) y si el usuario es su
	// mecánico o el jefe. Enseña también las respuestas del cliente y, mientras se puede responder,
	// el enlace (spec-004 FR-004 y FR-008).
	import { del, ErrorApi, get, post } from '$lib/api';
	import FormularioLinea from '$lib/componentes/FormularioLinea.svelte';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import Totales from '$lib/componentes/Totales.svelte';
	import { fecha, fechaDe, hora } from '$lib/formato';
	import type {
		EstadoOrden,
		EstadoPresupuesto,
		Linea,
		NuevaLinea,
		Presupuesto,
		PresupuestoResumen,
		TipoPresupuesto
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

	const NOMBRE_TIPO: Record<TipoPresupuesto, string> = {
		inicial: 'Inicial',
		ampliacion: 'Ampliación'
	};

	const mensaje = (e: unknown) =>
		e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';

	let presupuesto = $state<Presupuesto | null>(null);
	let error = $state('');
	let avisoCorreo = $state('');
	let copiado = $state(false);

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
	const respuestas = $derived(presupuestos.filter((p) => p.respondido_en));
	// Emitido y vigente: solo entonces se puede responder (spec-004 Q5). Es la condición del enlace y
	// de «Válido hasta». El enlace solo se oculta en pantalla: la API sigue enviándolo (D6).
	const vigente = $derived(presupuesto?.estado === 'emitido' && !presupuesto.caducado);
	const enlace = $derived(vigente ? presupuesto?.enlace : null);

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

	async function copiar() {
		if (!enlace) return;
		try {
			await navigator.clipboard.writeText(enlace);
			copiado = true;
			setTimeout(() => (copiado = false), 2000);
		} catch {
			// Sin HTTPS no hay portapapeles: el enlace sigue en el campo para copiarlo a mano (D6).
		}
	}

	async function crear() {
		error = '';
		try {
			presupuesto = await post<Presupuesto>(`/ordenes/${orden.id}/presupuestos`);
			onCambio();
		} catch (e) {
			error = mensaje(e);
		}
	}

	// Los errores los muestra el propio formulario (FormularioLinea).
	async function anadir(linea: NuevaLinea) {
		if (!presupuesto) return;
		presupuesto = await post<Presupuesto>(`/presupuestos/${presupuesto.id}/lineas`, linea);
		onCambio();
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
				Emitido el {fechaDe(presupuesto.emitido_en)}
				{#if presupuesto.caducado && presupuesto.caduca_el}
					· <strong>Caducado el {fecha(presupuesto.caduca_el)}</strong>
				{:else if vigente && presupuesto.caduca_el}
					· Válido hasta {fecha(presupuesto.caduca_el)}
				{/if}
			</p>
		{/if}
		{#if avisoCorreo}
			<p class="text-warning-700-300" role="alert">{avisoCorreo}</p>
		{/if}
		{#if respuestas.length > 0}
			<ul class="space-y-1">
				{#each respuestas as respuesta (respuesta.id)}
					{#if respuesta.respondido_en}
						<li>
							{NOMBRE_TIPO[respuesta.tipo]} · {NOMBRE_ESTADO[respuesta.estado]} el
							{fechaDe(respuesta.respondido_en)} a las {hora(respuesta.respondido_en)}
						</li>
					{/if}
				{/each}
			</ul>
		{/if}
		{#if enlace}
			<div class="flex flex-wrap gap-3">
				<input
					class="input min-w-0 flex-1 basis-64"
					type="text"
					readonly
					value={enlace}
					aria-label="Enlace del presupuesto"
				/>
				<button type="button" class="btn preset-tonal" onclick={copiar}>
					{copiado ? 'Copiado' : 'Copiar enlace'}
				</button>
			</div>
		{/if}

		<TablaLineas lineas={presupuesto.lineas} onquitar={enBorrador ? quitar : undefined} />
		<Totales totales={presupuesto} />

		{#if enBorrador}
			<FormularioLinea titulo="Añadir línea" textoBoton="Añadir línea" onenviar={anadir} />
			{#if presupuesto.lineas.length > 0}
				<div class="flex justify-end">
					<button type="button" class="btn preset-filled-primary-500" onclick={emitir}>
						Emitir presupuesto
					</button>
				</div>
			{/if}
		{/if}
	{/if}
</section>

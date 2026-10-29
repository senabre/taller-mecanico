<script lang="ts">
	// Sección «Presupuesto» de la ficha de la orden (spec-003 US2 y US3). La ficha (`+page.svelte`,
	// spec-002) le pasa la orden, sus presupuestos (`presupuestos_de_orden`) y si el usuario es su
	// mecánico o el jefe. El enlace y «Copiar enlace» los añade spec-004 (FR-008) en esta sección.
	import { del, ErrorApi, get, post } from '$lib/api';
	import FormularioLinea from '$lib/componentes/FormularioLinea.svelte';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import Totales from '$lib/componentes/Totales.svelte';
	import { fecha, fechaDe } from '$lib/formato';
	import type {
		EstadoOrden,
		EstadoPresupuesto,
		Linea,
		NuevaLinea,
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

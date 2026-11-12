<script lang="ts">
	// Sección «Diagnóstico» de la ficha de la orden (spec-003 US1, FR-001, FR-001a). La ficha
	// (`+page.svelte`, spec-002) le pasa la orden y si el usuario es su mecánico o el jefe.
	import { ErrorApi, put } from '$lib/api';
	import type { EstadoOrden, PresupuestoResumen } from '$lib/tipos';

	let {
		orden,
		puedeEditar,
		onguardado
	}: {
		orden: {
			id: number;
			estado: EstadoOrden;
			diagnostico: string | null;
			presupuestos: PresupuestoResumen[];
		};
		puedeEditar: boolean;
		onguardado: () => void;
	} = $props();

	// Con un presupuesto emitido (cualquiera que no esté en borrador, la misma regla que el
	// servidor, research D2), el diagnóstico queda fijo: se enseña sin formulario y con el motivo
	// (FR-001a, US1-3; converge-1 F2).
	const emitido = $derived(orden.presupuestos.some((p) => p.estado !== 'borrador'));
	// Se edita en recibida y en diagnóstico mientras no haya un presupuesto emitido.
	const editable = $derived(
		puedeEditar &&
			!emitido &&
			(orden.estado === 'recibida' || orden.estado === 'en_diagnostico')
	);

	let texto = $state('');
	let enviando = $state(false);
	let error = $state('');

	$effect(() => {
		texto = orden.diagnostico ?? '';
	});

	async function guardar(evento: SubmitEvent) {
		evento.preventDefault();
		enviando = true;
		error = '';
		try {
			await put(`/ordenes/${orden.id}/diagnostico`, { diagnostico: texto });
			onguardado();
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		} finally {
			enviando = false;
		}
	}
</script>

<section class="space-y-3">
	<h2 class="h4">Diagnóstico</h2>
	{#if editable}
		<!-- `novalidate`: el mensaje lo da el servidor, en español, no el navegador (principio II;
		     converge-1 F1). -->
		<form class="space-y-3" novalidate onsubmit={guardar}>
			<textarea
				class="textarea"
				rows="3"
				name="diagnostico"
				placeholder="Qué le pasa al vehículo"
				required
				bind:value={texto}
			></textarea>
			<div class="flex flex-wrap items-center gap-4">
				<button type="submit" class="btn preset-filled-primary-500" disabled={enviando}>
					{enviando ? 'Guardando…' : 'Guardar diagnóstico'}
				</button>
				{#if error}
					<p class="text-error-500" role="alert">{error}</p>
				{/if}
			</div>
		</form>
	{:else if orden.diagnostico}
		<p class="whitespace-pre-line">{orden.diagnostico}</p>
		{#if puedeEditar && emitido && orden.estado === 'en_diagnostico'}
			<p class="text-surface-600-400 text-sm">
				Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar.
			</p>
		{/if}
	{:else}
		<p>Todavía no hay diagnóstico.</p>
	{/if}
</section>

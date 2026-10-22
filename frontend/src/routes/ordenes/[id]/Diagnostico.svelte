<script lang="ts">
	// Sección «Diagnóstico» de la ficha de la orden (spec-003 US1, FR-001, FR-001a). La ficha
	// (`+page.svelte`, spec-002) le pasa la orden y si el usuario es su mecánico o el jefe.
	import { ErrorApi, put } from '$lib/api';
	import type { EstadoOrden } from '$lib/tipos';

	let {
		orden,
		puedeEditar,
		onguardado
	}: {
		orden: { id: number; estado: EstadoOrden; diagnostico: string | null };
		puedeEditar: boolean;
		onguardado: () => void;
	} = $props();

	// Se edita en recibida y en diagnóstico; el servidor rechaza el cambio si ya hay un
	// presupuesto emitido y el mensaje se muestra tal cual.
	const editable = $derived(
		puedeEditar && (orden.estado === 'recibida' || orden.estado === 'en_diagnostico')
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
		<form class="space-y-3" onsubmit={guardar}>
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
	{:else}
		<p>Todavía no hay diagnóstico.</p>
	{/if}
</section>

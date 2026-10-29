<script lang="ts">
	// Sección «Factura» de la ficha de la orden (spec-007 US1 y US2). La ficha (`+page.svelte`,
	// spec-002) le pasa la orden, su `factura_id` (`factura_de_orden`) y si el usuario es de
	// recepción o el jefe. En cualquier otro caso no muestra nada (US1/AC4).
	import { ErrorApi, get, post } from '$lib/api';
	import { euros, fecha } from '$lib/formato';
	import type { EstadoOrden, Factura } from '$lib/tipos';

	let {
		orden,
		facturaId,
		puedeFacturar,
		onCambio
	}: {
		orden: { id: number; estado: EstadoOrden };
		facturaId: number | null;
		puedeFacturar: boolean;
		onCambio: () => void;
	} = $props();

	let factura = $state<Factura | null>(null);
	let enviando = $state(false);
	let error = $state('');

	$effect(() => {
		if (!puedeFacturar || facturaId === null) {
			factura = null;
			return;
		}
		get<Factura>(`/facturas/${facturaId}`)
			.then((f) => (factura = f))
			.catch((e) => (error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado'));
	});

	async function facturar() {
		if (!confirm('¿Emitir la factura? Ya no se podrá modificar.')) return;
		enviando = true;
		error = '';
		try {
			factura = await post<Factura>(`/ordenes/${orden.id}/factura`);
			onCambio();
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		} finally {
			enviando = false;
		}
	}
</script>

{#if puedeFacturar && (factura || orden.estado === 'terminada')}
	<section class="space-y-3">
		<h2 class="h4">Factura</h2>
		{#if error}
			<p class="text-error-500" role="alert">{error}</p>
		{/if}

		{#if factura}
			<p>
				Factura nº {factura.numero} · {fecha(factura.fecha)} · <strong>{euros(factura.total)}</strong>
				· <a class="anchor" href="/facturas/{factura.id}">Ver e imprimir</a>
			</p>
			{#if !factura.aviso_enviado}
				<p class="text-warning-700-300" role="alert">
					No se pudo avisar al cliente por correo: llámale para decirle que puede recoger el coche.
				</p>
			{/if}
		{:else}
			<button
				type="button"
				class="btn preset-filled-primary-500"
				disabled={enviando}
				onclick={facturar}
			>
				{enviando ? 'Facturando…' : 'Facturar'}
			</button>
		{/if}
	</section>
{/if}

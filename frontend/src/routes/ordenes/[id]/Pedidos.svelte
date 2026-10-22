<script lang="ts">
	// Sección «Pedidos» de la ficha de la orden, solo de lectura (spec-006 US4, FR-009). La ficha
	// (`+page.svelte`, spec-002) le pasa el campo `pedidos` de GET /api/ordenes/{id}.
	import { fechaDe } from '$lib/formato';
	import type { EstadoPedido, Pedido } from '$lib/tipos';

	let { pedidos }: { pedidos: Pedido[] } = $props();

	const NOMBRE_ESTADO_PEDIDO: Record<EstadoPedido, string> = {
		pendiente: 'Pendiente',
		recibido: 'Recibido'
	};
</script>

<section class="space-y-4">
	<h2 class="h4">Pedidos</h2>
	{#if pedidos.length === 0}
		<p>Esta orden no tiene pedidos.</p>
	{:else}
		<div class="table-wrap">
			<table class="table">
				<thead>
					<tr>
						<th>Proveedor</th>
						<th>Referencia</th>
						<th class="text-right">Cantidad</th>
						<th>Pedido el</th>
						<th>Estado</th>
						<th>Recibido el</th>
					</tr>
				</thead>
				<tbody>
					{#each pedidos as p (p.id)}
						<tr>
							<td>{p.proveedor.nombre}</td>
							<td><span class="font-mono">{p.pieza.referencia}</span> · {p.pieza.descripcion}</td>
							<td class="text-right">{p.cantidad}</td>
							<td>{fechaDe(p.pedido_en)}</td>
							<td>{NOMBRE_ESTADO_PEDIDO[p.estado]}</td>
							<td>{p.recibido_en ? fechaDe(p.recibido_en) : '—'}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</section>

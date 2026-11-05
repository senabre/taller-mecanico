<script lang="ts">
	// Líneas de un presupuesto, de las imputaciones o de una factura (compartido: 003, 004, 005,
	// 007). Con `onquitar`, cada fila tiene el botón «Quitar». Las líneas de factura no tienen tipo
	// ni id (spec-007 D6): su cantidad sale sin «h» ni «uds».
	import { euros, numero } from '$lib/formato';
	import type { Linea, LineaFactura } from '$lib/tipos';

	let {
		lineas,
		onquitar
	}: { lineas: (Linea | LineaFactura)[]; onquitar?: (linea: Linea) => void } = $props();

	const cantidad = (l: Linea | LineaFactura) => {
		if (!('tipo' in l)) return numero(l.cantidad);
		return l.tipo === 'mano_obra' ? `${numero(l.cantidad)} h` : `${numero(l.cantidad)} uds`;
	};
</script>

{#if lineas.length === 0}
	<p>Todavía no hay líneas.</p>
{:else}
	<div class="table-wrap">
		<table class="table">
			<thead>
				<tr>
					<th>Concepto</th>
					<th class="text-right">Cantidad</th>
					<th class="text-right">Precio</th>
					<th class="text-right">Importe</th>
					{#if onquitar}<th></th>{/if}
				</tr>
			</thead>
			<tbody>
				{#each lineas as linea, i ('id' in linea ? linea.id : i)}
					<tr>
						<td>{linea.concepto}</td>
						<td class="text-right">{cantidad(linea)}</td>
						<td class="text-right">{euros(linea.precio_unitario)}</td>
						<td class="text-right">{euros(linea.importe)}</td>
						{#if onquitar}
							<td class="text-right">
								<button
									type="button"
									class="btn preset-tonal-error btn-sm"
									onclick={() => onquitar(linea as Linea)}
								>
									Quitar
								</button>
							</td>
						{/if}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

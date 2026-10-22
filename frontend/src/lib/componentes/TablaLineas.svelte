<script lang="ts">
	// Líneas de un presupuesto, de las imputaciones o de una factura (compartido: 003, 004, 005,
	// 007). Con `onquitar`, cada fila tiene el botón «Quitar».
	import { euros, numero } from '$lib/formato';
	import type { Linea } from '$lib/tipos';

	let {
		lineas,
		onquitar
	}: { lineas: Linea[]; onquitar?: (linea: Linea) => void } = $props();

	const cantidad = (l: Linea) =>
		l.tipo === 'mano_obra' ? `${numero(l.cantidad)} h` : `${numero(l.cantidad)} uds`;
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
				{#each lineas as linea (linea.id)}
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
									onclick={() => onquitar(linea)}
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

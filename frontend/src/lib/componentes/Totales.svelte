<script lang="ts">
	// Base imponible, IVA desglosado y total (compartido: 003, 004, 005, 007).
	import { euros, numero } from '$lib/formato';
	import type { Totales } from '$lib/tipos';

	let { totales }: { totales: Totales } = $props();
</script>

<dl class="ml-auto grid w-full max-w-xs grid-cols-2 gap-x-4 gap-y-1">
	<dt>Base imponible</dt>
	<dd class="text-right">{euros(totales.base)}</dd>
	<dt>IVA ({numero(totales.iva_porcentaje)} %)</dt>
	<dd class="text-right">{euros(totales.iva)}</dd>
	<dt class="font-bold">Total</dt>
	<dd class="text-right font-bold">{euros(totales.total)}</dd>
</dl>

<script lang="ts">
	// Sección «Datos» de la ficha de la orden (spec-002 US1-3, SC-004).
	import { fecha as fechaEs, NOMBRE_ESTADO, numero } from '$lib/formato';
	import type { Orden } from '$lib/tipos';

	let { orden }: { orden: Orden } = $props();
</script>

<section class="space-y-3">
	<h2 class="h4">Datos</h2>
	<dl class="grid gap-x-6 gap-y-2 sm:grid-cols-[auto_1fr]">
		<dt class="font-semibold">Estado</dt>
		<dd>{NOMBRE_ESTADO[orden.estado]}</dd>
		<dt class="font-semibold">Vehículo</dt>
		<dd><span class="font-mono">{orden.matricula}</span> · {orden.vehiculo.marca} {orden.vehiculo.modelo}</dd>
		<dt class="font-semibold">Cita</dt>
		<dd>{fechaEs(orden.cita.fecha)} · {orden.cita.motivo}</dd>
		<dt class="font-semibold">Kilómetros</dt>
		<dd>{numero(orden.km)} km</dd>
		<dt class="font-semibold">Daños visibles</dt>
		<dd class="whitespace-pre-line">{orden.danos}</dd>
		<dt class="font-semibold">Mecánico</dt>
		<dd>{orden.mecanico.nombre}</dd>
		<dt class="font-semibold">Entrega prometida</dt>
		<dd>
			{fechaEs(orden.fecha_entrega_prometida)}
			{#if orden.retrasada}
				<span class="text-error-500 font-semibold">Retrasada</span>
			{/if}
		</dd>
	</dl>
</section>

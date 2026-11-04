<script lang="ts">
	// Ficha de la orden (spec-002; contenedor compartido, research D6). Carga la orden una vez y
	// pasa a cada sección lo que espera. Una sección nueva es una línea en el marcado; la de
	// «Factura» (spec-007) se activa con `yo.rol` para recepción y el jefe (hallazgo I4).
	import { page } from '$app/state';
	import { ErrorApi, get } from '$lib/api';
	import type { Orden, ResumenImputaciones } from '$lib/tipos';
	import Datos from './Datos.svelte';
	import Diagnostico from './Diagnostico.svelte';
	import Imputaciones from './Imputaciones.svelte';
	import Pedidos from './Pedidos.svelte';
	import Presupuesto from './Presupuesto.svelte';

	const id = $derived(page.params.id);
	const yo = $derived(page.data.yo);

	let orden = $state<Orden | null>(null);
	let error = $state('');

	// Las secciones del mecánico: el asignado y el jefe.
	const puedeEditar = $derived(
		orden !== null && (yo?.rol === 'jefe' || yo?.id === orden.mecanico.id)
	);
	const resumen: ResumenImputaciones | null = $derived(
		orden && {
			orden_id: orden.id,
			estado: orden.estado,
			coste_acumulado: orden.coste_acumulado,
			total_presupuestado: orden.total_presupuestado,
			imputaciones: orden.imputaciones
		}
	);

	async function cargar(ordenId: string | undefined) {
		try {
			orden = await get<Orden>(`/ordenes/${ordenId}`);
			error = '';
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'No se pudo cargar la orden';
		}
	}

	const recargar = () => cargar(id);

	$effect(() => {
		cargar(id);
	});
</script>

<svelte:head>
	<title>{orden ? `Orden Nº ${orden.numero}` : 'Orden'} · Talleres Rueda</title>
</svelte:head>

{#if error}
	<p class="text-error-500" role="alert">{error}</p>
{:else if orden && resumen}
	<div class="space-y-8">
		<h1 class="h2">Orden Nº {orden.numero}</h1>
		<Datos {orden} />
		<Diagnostico {orden} {puedeEditar} onguardado={recargar} />
		<Presupuesto {orden} presupuestos={orden.presupuestos} {puedeEditar} onCambio={recargar} />
		<Imputaciones {resumen} {puedeEditar} onCambio={recargar} />
		<Pedidos pedidos={orden.pedidos} />
	</div>
{/if}

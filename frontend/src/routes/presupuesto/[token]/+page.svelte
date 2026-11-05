<script lang="ts">
	// Respuesta del cliente, página pública (spec-004 US1, US2 y US3). El token del enlace da acceso
	// a este presupuesto y a nada más; nunca se muestran el nombre, el teléfono ni el correo del
	// cliente (FR-002, principio V). La confirmación va en la página, y no con `confirm()`, cuyos
	// botones salen en el idioma del navegador (research D5).
	import { page } from '$app/state';
	import { ErrorApi, get, post } from '$lib/api';
	import TablaLineas from '$lib/componentes/TablaLineas.svelte';
	import Totales from '$lib/componentes/Totales.svelte';
	import { euros, fecha, fechaDe, hora } from '$lib/formato';
	import type { PresupuestoPublico } from '$lib/tipos';

	type Decision = 'aceptar' | 'rechazar';

	const CONFIRMACION: Record<Decision, { pregunta: string; boton: string }> = {
		aceptar: { pregunta: '¿Acepta este presupuesto por', boton: 'Sí, aceptar' },
		rechazar: { pregunta: '¿Rechaza este presupuesto por', boton: 'Sí, rechazar' }
	};
	const NOMBRE_DECISION = { aceptado: 'Aceptado', rechazado: 'Rechazado' };

	const ruta = $derived(`/publico/presupuestos/${encodeURIComponent(page.params.token ?? '')}`);

	let presupuesto = $state<PresupuestoPublico | null>(null);
	let errorCarga = $state('');
	let aviso = $state('');
	let confirmando = $state<Decision | null>(null);
	let enviando = $state(false);

	async function cargar() {
		try {
			presupuesto = await get<PresupuestoPublico>(ruta);
			errorCarga = '';
		} catch (e) {
			errorCarga =
				e instanceof ErrorApi && e.estado === 404 ? e.message : 'No se pudo cargar el presupuesto';
		}
	}

	$effect(() => {
		cargar();
	});

	async function responder() {
		if (!confirmando) return;
		enviando = true;
		aviso = '';
		try {
			presupuesto = await post<PresupuestoPublico>(`${ruta}/respuesta`, { decision: confirmando });
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 409) {
				await cargar();
				// El aviso de caducidad ya lo enseña la página: no se repite (US3-2).
				aviso = presupuesto?.caducado ? '' : e.message;
			} else {
				aviso = 'Ha ocurrido un error inesperado';
			}
		} finally {
			confirmando = null;
			enviando = false;
		}
	}
</script>

<svelte:head><title>Presupuesto · Talleres Rueda</title></svelte:head>

<main class="mx-auto max-w-2xl space-y-6 p-4 sm:p-8">
	{#if presupuesto}
		<header>
			<p class="text-sm">{presupuesto.taller.nombre}</p>
			<h1 class="h2">
				{presupuesto.tipo === 'ampliacion' ? 'Ampliación de presupuesto' : 'Presupuesto'}
			</h1>
		</header>

		<section class="card bg-surface-100-900 space-y-4 p-4 sm:p-6">
			<p>
				<strong>Vehículo:</strong>
				{presupuesto.marca}
				{presupuesto.modelo} · {presupuesto.matricula}
			</p>
			{#if presupuesto.diagnostico}
				<p><strong>Diagnóstico:</strong> {presupuesto.diagnostico}</p>
			{/if}
			<TablaLineas lineas={presupuesto.lineas} />
			<Totales totales={presupuesto} />
		</section>

		{#if aviso}
			<p class="text-error-500" role="alert">{aviso}</p>
		{/if}

		{#if presupuesto.decision && presupuesto.respondido_en}
			<p class="font-bold" role="status">
				{NOMBRE_DECISION[presupuesto.decision]} el {fechaDe(presupuesto.respondido_en)} a las
				{hora(presupuesto.respondido_en)}
			</p>
		{:else if presupuesto.caducado}
			<p class="font-bold" role="alert">
				Este presupuesto caducó el {fecha(presupuesto.caduca_el)}. Contacte con el taller
			</p>
		{:else}
			<p>Válido hasta {fecha(presupuesto.caduca_el)}</p>
			{#if confirmando}
				<section class="card bg-surface-100-900 space-y-4 p-4 sm:p-6">
					<p>
						{CONFIRMACION[confirmando].pregunta}
						{euros(presupuesto.total)}? Una vez confirmado, no se podrá cambiar.
					</p>
					<div class="flex flex-wrap gap-3">
						<button
							type="button"
							class="btn preset-filled-primary-500"
							disabled={enviando}
							onclick={responder}
						>
							{CONFIRMACION[confirmando].boton}
						</button>
						<button
							type="button"
							class="btn preset-tonal"
							disabled={enviando}
							onclick={() => (confirmando = null)}
						>
							Volver
						</button>
					</div>
				</section>
			{:else}
				<div class="flex flex-wrap gap-3">
					<button
						type="button"
						class="btn preset-filled-primary-500"
						onclick={() => (confirmando = 'aceptar')}
					>
						Aceptar
					</button>
					<button type="button" class="btn preset-tonal" onclick={() => (confirmando = 'rechazar')}>
						Rechazar
					</button>
				</div>
			{/if}
		{/if}
	{:else if errorCarga}
		<p role="alert">{errorCarga}</p>
	{/if}
</main>

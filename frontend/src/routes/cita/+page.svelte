<script lang="ts">
	// Pedir cita, página pública (spec-001 FR-001). Todos los mensajes de error de los campos
	// salen del servidor: el formulario usa `novalidate` (research D2).
	import { ErrorApi, get, post } from '$lib/api';
	import { fecha as fechaEs } from '$lib/formato';
	import type { CitaCreada, Fecha, VehiculoPublico } from '$lib/tipos';

	const CITA_PENDIENTE = 'Este vehículo ya tiene una cita pendiente.';

	let dias: Fecha[] = $state([]);
	let errorDias = $state('');

	let matricula = $state('');
	let fecha = $state<Fecha | ''>('');
	let motivo = $state('');

	// Resultado de buscar la matrícula: null mientras no se sepa (o si hay que volver a buscar).
	let vehiculo = $state<VehiculoPublico | null>(null);
	let buscando = $state(false);

	// Mensaje de error por campo (`detalle.campo` del 422) y error general.
	let errores: Record<string, string> = $state({});
	let error = $state('');
	let enviando = $state(false);
	let confirmada = $state<CitaCreada | null>(null);

	const puedeConfirmar = $derived(
		vehiculo !== null && !vehiculo.cita_pendiente && fecha !== '' && !enviando && !buscando
	);

	async function cargarDias() {
		try {
			dias = await get<Fecha[]>('/publico/dias-disponibles');
			errorDias = '';
		} catch (e) {
			errorDias = e instanceof ErrorApi ? e.message : 'No se pudieron cargar los días';
		}
	}

	$effect(() => {
		cargarDias();
	});

	function matriculaCambiada() {
		vehiculo = null;
		delete errores.matricula;
		error = '';
	}

	async function buscarVehiculo() {
		if (matricula.trim() === '') return;
		buscando = true;
		vehiculo = null;
		errores = {};
		error = '';
		try {
			vehiculo = await get<VehiculoPublico>(
				'/publico/vehiculos/' + encodeURIComponent(matricula.trim())
			);
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 422) {
				errores = { matricula: e.message };
			} else if (e instanceof ErrorApi && e.estado === 404) {
				// Vehículo nuevo: los campos de vehículo y contacto los añade T025 (US2).
			} else {
				error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
			}
		} finally {
			buscando = false;
		}
	}

	async function confirmar(evento: SubmitEvent) {
		evento.preventDefault();
		if (!puedeConfirmar) return;
		enviando = true;
		errores = {};
		error = '';
		try {
			confirmada = await post<CitaCreada>('/publico/citas', {
				matricula: matricula.trim(),
				fecha,
				motivo
			});
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 422 && typeof e.detalle?.campo === 'string') {
				errores = { [e.detalle.campo]: e.message };
			} else if (e instanceof ErrorApi && e.estado === 409 && e.message === CITA_PENDIENTE) {
				if (vehiculo) vehiculo = { ...vehiculo, cita_pendiente: true };
			} else {
				error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
			}
		} finally {
			enviando = false;
		}
	}

	function otraCita() {
		confirmada = null;
		matricula = '';
		fecha = '';
		motivo = '';
		vehiculo = null;
		errores = {};
		cargarDias();
	}
</script>

<svelte:head><title>Pedir cita · Talleres Rueda</title></svelte:head>

<main class="mx-auto max-w-lg space-y-6 p-4 sm:p-8">
	<h1 class="h2">Pedir cita</h1>

	{#if confirmada}
		<section class="card bg-surface-100-900 space-y-3 p-6" role="status">
			<h2 class="h4">Cita confirmada</h2>
			<p>Fecha: <strong>{fechaEs(confirmada.fecha)}</strong></p>
			<p>Motivo: <strong>{confirmada.motivo}</strong></p>
			<button type="button" class="btn preset-tonal" onclick={otraCita}>Pedir otra cita</button>
		</section>
	{:else}
		<form class="card bg-surface-100-900 space-y-4 p-6" novalidate onsubmit={confirmar}>
			<label class="label">
				<span class="label-text">Matrícula</span>
				<input
					class="input"
					name="matricula"
					autocomplete="off"
					autocapitalize="characters"
					placeholder="1234 BCD"
					bind:value={matricula}
					oninput={matriculaCambiada}
					onblur={buscarVehiculo}
				/>
				{#if errores.matricula}
					<span class="text-error-500" role="alert">{errores.matricula}</span>
				{/if}
			</label>

			{#if buscando}
				<p>Buscando el vehículo…</p>
			{:else if vehiculo}
				<p>Vehículo: <strong>{vehiculo.marca} {vehiculo.modelo}</strong></p>
				{#if vehiculo.cita_pendiente}
					<p class="text-warning-700-300" role="alert">{CITA_PENDIENTE}</p>
				{/if}
			{/if}

			<label class="label">
				<span class="label-text">Día</span>
				<select class="select" name="fecha" bind:value={fecha}>
					<option value="" disabled>Elige un día</option>
					{#each dias as dia (dia)}
						<option value={dia}>{fechaEs(dia)}</option>
					{/each}
				</select>
				{#if errores.fecha}
					<span class="text-error-500" role="alert">{errores.fecha}</span>
				{/if}
			</label>
			{#if errorDias}
				<p class="text-error-500" role="alert">{errorDias}</p>
			{/if}

			<label class="label">
				<span class="label-text">Motivo de la visita</span>
				<textarea class="textarea" name="motivo" rows="3" bind:value={motivo}></textarea>
				{#if errores.motivo}
					<span class="text-error-500" role="alert">{errores.motivo}</span>
				{/if}
			</label>

			<div class="flex flex-wrap items-center gap-4">
				<button type="submit" class="btn preset-filled-primary-500" disabled={!puedeConfirmar}>
					{enviando ? 'Confirmando…' : 'Confirmar'}
				</button>
				{#if error}
					<p class="text-error-500" role="alert">{error}</p>
				{/if}
			</div>
		</form>
	{/if}
</main>

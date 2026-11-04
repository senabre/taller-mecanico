<script lang="ts">
	// Abrir la orden de una cita de hoy (spec-002 US1), desde «Abrir orden» en /citas. Todos los
	// mensajes de error salen del servidor: el formulario usa `novalidate` (research D7).
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { ErrorApi, get, post } from '$lib/api';
	import type { CitaDelDia, Fecha, Mecanico, Orden, PeticionOrden } from '$lib/tipos';

	const citaId = Number(page.url.searchParams.get('cita'));

	let cita = $state<CitaDelDia | null>(null);
	let mecanicos: Mecanico[] = $state([]);
	let cargada = $state(false);
	let errorCarga = $state('');

	let km = $state('');
	let danos = $state('');
	let mecanico = $state<number | null>(null);
	let entrega = $state<Fecha | ''>('');

	// Mensaje de error por campo (`detalle.campo` del 422) y error general.
	let errores: Record<string, string> = $state({});
	let error = $state('');
	// Con el 409 de una cita que ya tiene orden, la orden que la tiene.
	let ordenExistente = $state<number | null>(null);
	let enviando = $state(false);

	async function cargar() {
		try {
			// Sin `fecha`, el servidor da las citas de hoy en Madrid (R12).
			const [citas, lista] = await Promise.all([
				get<CitaDelDia[]>('/citas'),
				get<Mecanico[]>('/mecanicos')
			]);
			cita = citas.find((c) => c.id === citaId) ?? null;
			mecanicos = lista;
		} catch (e) {
			errorCarga = e instanceof ErrorApi ? e.message : 'No se pudo cargar la cita';
		} finally {
			cargada = true;
		}
	}

	$effect(() => {
		cargar();
	});

	async function guardar(evento: SubmitEvent) {
		evento.preventDefault();
		enviando = true;
		errores = {};
		error = '';
		ordenExistente = null;
		try {
			const peticion = {
				cita_id: citaId,
				km,
				danos,
				mecanico_id: mecanico,
				fecha_entrega_prometida: entrega
			} as PeticionOrden;
			const orden = await post<Orden>('/ordenes', peticion);
			await goto(`/ordenes/${orden.id}`);
		} catch (e) {
			if (e instanceof ErrorApi && e.estado === 422 && typeof e.detalle?.campo === 'string') {
				errores = { [e.detalle.campo]: e.message };
			} else {
				error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
				if (e instanceof ErrorApi && typeof e.detalle?.orden_id === 'number') {
					ordenExistente = e.detalle.orden_id;
				}
			}
		} finally {
			enviando = false;
		}
	}
</script>

<svelte:head><title>Abrir orden · Talleres Rueda</title></svelte:head>

<div class="mx-auto max-w-lg space-y-6">
	<h1 class="h2">Abrir orden</h1>

	{#if errorCarga}
		<p class="text-error-500" role="alert">{errorCarga}</p>
	{:else if cargada && !cita}
		<p class="text-error-500" role="alert">Solo se puede abrir la orden el día de la cita</p>
	{:else if cita}
		<section class="card bg-surface-100-900 space-y-1 p-6">
			<p class="font-mono font-semibold">{cita.matricula}</p>
			<p>{cita.marca} {cita.modelo}</p>
			<p>Motivo: {cita.motivo}</p>
		</section>

		<form class="card bg-surface-100-900 space-y-4 p-6" novalidate onsubmit={guardar}>
			<label class="label">
				<span class="label-text">Kilómetros</span>
				<input class="input" name="km" inputmode="numeric" autocomplete="off" bind:value={km} />
				{#if errores.km}
					<span class="text-error-500" role="alert">{errores.km}</span>
				{/if}
			</label>

			<label class="label">
				<span class="label-text">Daños visibles</span>
				<textarea class="textarea" name="danos" rows="3" bind:value={danos}></textarea>
				{#if errores.danos}
					<span class="text-error-500" role="alert">{errores.danos}</span>
				{/if}
			</label>

			<label class="label">
				<span class="label-text">Mecánico</span>
				<select class="select" name="mecanico" bind:value={mecanico}>
					<option value={null} disabled>Elige un mecánico</option>
					{#each mecanicos as m (m.id)}
						<option value={m.id}>{m.nombre}</option>
					{/each}
				</select>
				{#if errores.mecanico_id}
					<span class="text-error-500" role="alert">{errores.mecanico_id}</span>
				{/if}
			</label>

			<label class="label">
				<span class="label-text">Fecha de entrega prometida</span>
				<input class="input" type="date" name="entrega" bind:value={entrega} />
				{#if errores.fecha_entrega_prometida}
					<span class="text-error-500" role="alert">{errores.fecha_entrega_prometida}</span>
				{/if}
			</label>

			<div class="flex flex-wrap items-center gap-4">
				<button type="submit" class="btn preset-filled-primary-500" disabled={enviando}>
					{enviando ? 'Guardando…' : 'Guardar'}
				</button>
				{#if error}
					<p class="text-error-500" role="alert">
						{error}
						{#if ordenExistente !== null}
							<a class="anchor" href={`/ordenes/${ordenExistente}`}>Ver la orden</a>
						{/if}
					</p>
				{/if}
			</div>
		</form>
	{/if}
</div>

<script lang="ts">
	// Citas de un día, para recepción y el jefe de taller (spec-001 FR-009). Abre con el día de
	// hoy en hora de Madrid (research R12), el mismo «hoy» que usa el servidor.
	import { ErrorApi, get } from '$lib/api';
	import { fecha as fechaEs } from '$lib/formato';
	import type { CitaDelDia, Fecha } from '$lib/tipos';

	/** Hoy en Madrid como "AAAA-MM-DD" (el formato `en-CA` ya es año-mes-día). */
	function hoyEnMadrid(): Fecha {
		return new Intl.DateTimeFormat('en-CA', { timeZone: 'Europe/Madrid' }).format(new Date());
	}

	const hoy = hoyEnMadrid();
	let dia = $state<Fecha>(hoy);
	let citas: CitaDelDia[] = $state([]);
	let cargada = $state(false);
	let error = $state('');
	// Sin permiso (403): solo se muestra el mensaje, sin título ni selector.
	let prohibido = $state(false);

	async function cargar(elegido: Fecha) {
		cargada = false;
		error = '';
		try {
			const lista = await get<CitaDelDia[]>(`/citas?fecha=${elegido}`);
			// Si mientras tanto se ha elegido otro día, esta respuesta ya no vale.
			if (elegido !== dia) return;
			citas = lista;
		} catch (e) {
			if (elegido !== dia) return;
			citas = [];
			prohibido = e instanceof ErrorApi && e.estado === 403;
			error = e instanceof ErrorApi ? e.message : 'No se pudo cargar la lista de citas';
		}
		cargada = true;
	}

	$effect(() => {
		// Si se borra la fecha del selector, se vuelve a hoy.
		if (!dia) {
			dia = hoy;
			return;
		}
		cargar(dia);
	});
</script>

<svelte:head><title>Citas · Talleres Rueda</title></svelte:head>

{#if prohibido}
	<p class="text-error-500" role="alert">{error}</p>
{:else}
	<div class="space-y-6">
		<h1 class="h2">{dia === hoy ? 'Citas de hoy' : `Citas del ${fechaEs(dia)}`}</h1>

		<div class="flex flex-wrap items-end gap-4">
			<label class="label w-auto">
				<span class="label-text">Día</span>
				<input class="input" type="date" name="dia" bind:value={dia} />
			</label>
			{#if dia !== hoy}
				<button type="button" class="btn preset-tonal" onclick={() => (dia = hoy)}>Hoy</button>
			{/if}
		</div>

		{#if error}
			<p class="text-error-500" role="alert">{error}</p>
		{:else if cargada && citas.length === 0}
			<p>No hay citas para este día</p>
		{:else if cargada}
			<div class="table-wrap">
				<table class="table">
					<thead>
						<tr>
							<th>Matrícula</th>
							<th>Marca</th>
							<th>Modelo</th>
							<th>Motivo</th>
							<th>Nombre</th>
							<th>Teléfono</th>
						</tr>
					</thead>
					<tbody>
						{#each citas as cita (cita.id)}
							<tr>
								<td class="font-mono">{cita.matricula}</td>
								<td>{cita.marca}</td>
								<td>{cita.modelo}</td>
								<td>{cita.motivo}</td>
								<td>{cita.cliente.nombre}</td>
								<td>{cita.cliente.telefono}</td>
								<!-- spec-002 añade aquí el botón «Abrir orden» (usa cita.orden_id). -->
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>
{/if}

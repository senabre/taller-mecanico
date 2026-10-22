<script lang="ts">
	// Pedidos a proveedor: registrar un pedido para una orden (US2) y recibir los pendientes de
	// todas las órdenes (US3). spec-006.
	import { ErrorApi, get, post } from '$lib/api';
	import { fechaDe } from '$lib/formato';
	import type { Pedido, Pieza, Proveedor } from '$lib/tipos';

	const mensaje = (e: unknown) =>
		e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';

	// --- Datos de apoyo y lista de pendientes ---
	let proveedores: Proveedor[] = $state([]);
	let pendientes: Pedido[] = $state([]);
	let errorCarga = $state('');

	async function cargarPendientes() {
		pendientes = await get<Pedido[]>('/pedidos?estado=pendiente');
	}

	$effect(() => {
		(async () => {
			try {
				proveedores = await get<Proveedor[]>('/proveedores');
				await cargarPendientes();
			} catch (e) {
				errorCarga = mensaje(e);
			}
		})();
	});

	// --- Nuevo pedido ---
	let proveedorId: number | null = $state(null);
	let busqueda = $state('');
	let resultados: Pieza[] = $state([]);
	let pieza: Pieza | null = $state(null);
	let cantidad: number | null = $state(1);
	let ordenNumero: number | null = $state(null);
	let enviando = $state(false);
	let error = $state('');
	let aviso = $state('');

	let temporizador: ReturnType<typeof setTimeout> | undefined;

	function buscar() {
		pieza = null;
		clearTimeout(temporizador);
		const texto = busqueda.trim();
		if (texto === '') {
			resultados = [];
			return;
		}
		temporizador = setTimeout(async () => {
			try {
				resultados = await get<Pieza[]>('/piezas?q=' + encodeURIComponent(texto));
			} catch (e) {
				error = mensaje(e);
			}
		}, 250);
	}

	function elegir(p: Pieza) {
		pieza = p;
		busqueda = `${p.referencia} · ${p.descripcion}`;
		resultados = [];
	}

	async function registrar(evento: SubmitEvent) {
		evento.preventDefault();
		error = '';
		aviso = '';
		if (!pieza) {
			error = 'Busca y elige una referencia de la lista';
			return;
		}
		enviando = true;
		try {
			const pedido = await post<Pedido>('/pedidos', {
				proveedor_id: proveedorId,
				pieza_id: pieza.id,
				cantidad,
				orden_numero: ordenNumero
			});
			aviso = `Pedido registrado: ${pedido.cantidad} × ${pedido.pieza.referencia} para la orden ${pedido.orden.numero}`;
			pieza = null;
			busqueda = '';
			cantidad = 1;
			ordenNumero = null;
			await cargarPendientes();
		} catch (e) {
			error = mensaje(e);
		} finally {
			enviando = false;
		}
	}

	// --- Recibir ---
	let recibiendo: number | null = $state(null);
	let errorRecibir: Record<number, string> = $state({});

	async function marcarRecibido(p: Pedido) {
		recibiendo = p.id;
		errorRecibir[p.id] = '';
		try {
			await post<Pedido>(`/pedidos/${p.id}/recibir`);
			pendientes = pendientes.filter((x) => x.id !== p.id);
		} catch (e) {
			errorRecibir[p.id] = mensaje(e);
		} finally {
			recibiendo = null;
		}
	}
</script>

<svelte:head><title>Pedidos · Talleres Rueda</title></svelte:head>

<div class="space-y-8">
	<h1 class="h2">Pedidos a proveedor</h1>

	{#if errorCarga}
		<p class="text-error-500" role="alert">{errorCarga}</p>
	{/if}

	<section class="card bg-surface-100-900 max-w-3xl space-y-4 p-6">
		<h2 class="h4">Nuevo pedido</h2>
		<form class="grid gap-4 sm:grid-cols-2" onsubmit={registrar}>
			<label class="label">
				<span class="label-text">Proveedor</span>
				<select class="select" name="proveedor" required bind:value={proveedorId}>
					<option value={null} disabled>Elige un proveedor</option>
					{#each proveedores as proveedor (proveedor.id)}
						<option value={proveedor.id}>{proveedor.nombre}</option>
					{/each}
				</select>
			</label>

			<div class="label relative">
				<label class="label-text" for="referencia">Referencia</label>
				<input
					id="referencia"
					class="input"
					autocomplete="off"
					placeholder="Busca por referencia o descripción"
					required
					bind:value={busqueda}
					oninput={buscar}
				/>
				{#if resultados.length > 0}
					<ul
						class="card bg-surface-50-950 border-surface-200-800 absolute z-10 mt-1 max-h-60 w-full overflow-auto border"
					>
						{#each resultados as r (r.id)}
							<li>
								<button
									type="button"
									class="hover:bg-surface-200-800 w-full px-3 py-2 text-left"
									onclick={() => elegir(r)}
								>
									<span class="font-mono">{r.referencia}</span> · {r.descripcion}
									<span class="text-surface-600-400 text-sm">(stock {r.stock})</span>
								</button>
							</li>
						{/each}
					</ul>
				{:else if busqueda.trim() !== '' && !pieza}
					<p class="text-surface-600-400 text-sm">
						Si la referencia no aparece, primero hay que darla de alta en Almacén.
					</p>
				{/if}
			</div>

			<label class="label">
				<span class="label-text">Cantidad</span>
				<input
					class="input"
					name="cantidad"
					type="number"
					min="1"
					step="1"
					required
					bind:value={cantidad}
				/>
			</label>

			<label class="label">
				<span class="label-text">Nº de orden</span>
				<input
					class="input"
					name="orden"
					type="number"
					min="1"
					step="1"
					required
					bind:value={ordenNumero}
				/>
			</label>

			<div class="flex flex-wrap items-center gap-4 sm:col-span-2">
				<button type="submit" class="btn preset-filled-primary-500" disabled={enviando}>
					{enviando ? 'Registrando…' : 'Registrar pedido'}
				</button>
				{#if error}
					<p class="text-error-500" role="alert">{error}</p>
				{:else if aviso}
					<p class="text-success-700-300" role="status">{aviso}</p>
				{/if}
			</div>
		</form>
	</section>

	<section class="space-y-4">
		<h2 class="h4">Pedidos pendientes</h2>
		{#if pendientes.length === 0}
			<p>No hay pedidos pendientes.</p>
		{:else}
			<div class="table-wrap">
				<table class="table">
					<thead>
						<tr>
							<th>Orden</th>
							<th>Proveedor</th>
							<th>Referencia</th>
							<th class="text-right">Cantidad</th>
							<th>Fecha</th>
							<th></th>
						</tr>
					</thead>
					<tbody>
						{#each pendientes as p (p.id)}
							<tr>
								<td>{p.orden.numero}</td>
								<td>{p.proveedor.nombre}</td>
								<td><span class="font-mono">{p.pieza.referencia}</span> · {p.pieza.descripcion}</td>
								<td class="text-right">{p.cantidad}</td>
								<td>{fechaDe(p.pedido_en)}</td>
								<td class="text-right">
									<button
										type="button"
										class="btn preset-tonal-primary btn-sm"
										disabled={recibiendo === p.id}
										onclick={() => marcarRecibido(p)}
									>
										{recibiendo === p.id ? 'Guardando…' : 'Marcar recibido'}
									</button>
									{#if errorRecibir[p.id]}
										<p class="text-error-500 text-sm" role="alert">{errorRecibir[p.id]}</p>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</div>

<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { ErrorApi, get, post } from '$lib/api';
	import { INICIO_ROL } from '$lib/formato';
	import type { Yo } from '$lib/tipos';

	let { data } = $props();

	let login = $state('');
	let contrasena = $state('');
	let enviando = $state(false);
	let error = $state('');
	let campoUsuario: HTMLInputElement | undefined = $state();

	// Con sesión abierta no hace falta entrar: a la página de inicio del rol.
	$effect(() => {
		if (data.yo) goto(INICIO_ROL[data.yo.rol], { replaceState: true });
	});

	$effect(() => {
		campoUsuario?.focus();
	});

	async function entrar(evento: SubmitEvent) {
		evento.preventDefault();
		enviando = true;
		error = '';
		try {
			await post('/sesion', { login, contrasena });
			const yo = await get<Yo>('/yo');
			await invalidateAll();
			await goto(INICIO_ROL[yo.rol], { replaceState: true });
		} catch (e) {
			error = e instanceof ErrorApi ? e.message : 'Ha ocurrido un error inesperado';
		} finally {
			enviando = false;
		}
	}
</script>

<main class="flex min-h-screen items-center justify-center px-4">
	<form class="card bg-surface-100-900 w-full max-w-sm space-y-4 p-6" onsubmit={entrar}>
		<h1 class="h3 text-center">Talleres Rueda</h1>

		<label class="label">
			<span class="label-text">Usuario</span>
			<input
				class="input"
				name="login"
				autocomplete="username"
				required
				bind:value={login}
				bind:this={campoUsuario}
			/>
		</label>

		<label class="label">
			<span class="label-text">Contraseña</span>
			<input
				class="input"
				type="password"
				name="contrasena"
				autocomplete="current-password"
				required
				bind:value={contrasena}
			/>
		</label>

		{#if error}
			<p class="text-error-500" role="alert">{error}</p>
		{/if}

		<button type="submit" class="btn preset-filled-primary-500 w-full" disabled={enviando}>
			{enviando ? 'Entrando…' : 'Entrar'}
		</button>
	</form>
</main>

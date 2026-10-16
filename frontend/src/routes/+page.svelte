<script lang="ts">
	import { goto } from '$app/navigation';
	import { INICIO_ROL } from '$lib/formato';

	let { data } = $props();

	// «/» no tiene contenido: lleva a la página de inicio del rol, o a /entrar sin sesión.
	$effect(() => {
		goto(data.yo ? INICIO_ROL[data.yo.rol] : '/entrar', { replaceState: true });
	});
</script>

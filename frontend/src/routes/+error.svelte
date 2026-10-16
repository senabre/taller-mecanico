<script lang="ts">
	import { page } from '$app/state';
</script>

<!-- Se pinta dentro del layout, así que el menú del rol sigue visible. Siempre texto fijo. -->
<section class="py-16 text-center">
	<h1 class="h3">{page.status === 404 ? 'Esta página no existe' : 'Ha ocurrido un error'}</h1>
</section>

import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		// En desarrollo, la API la sirve el backend en el puerto 3000 (research R2).
		proxy: { '/api': 'http://localhost:3000' }
	},
	test: {
		include: ['src/**/*.test.ts']
	}
});

// Una sola petición del tablero a la vez, más una si llegan avisos mientras tanto (spec-008
// research D5). Las peticiones son promesas que la prueba resuelve a mano.
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { crearRecarga } from './recarga';

/** Una petición simulada: cuenta las llamadas y las que hay en curso a la vez. */
function peticionSimulada() {
	const pendientes: { resolver: () => void; fallar: () => void }[] = [];
	let enCurso = 0;
	let maximo = 0;
	const fn = vi.fn(
		() =>
			new Promise<void>((resolve, reject) => {
				enCurso++;
				maximo = Math.max(maximo, enCurso);
				pendientes.push({
					resolver: () => {
						enCurso--;
						resolve();
					},
					fallar: () => {
						enCurso--;
						reject(new Error('fallo de la petición'));
					}
				});
			})
	);
	return { fn, pendientes, maximo: () => maximo };
}

describe('crearRecarga', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});
	afterEach(() => {
		vi.useRealTimers();
	});

	it('tres avisos durante una petición dan una sola petición más, nunca dos a la vez', async () => {
		const { fn, pendientes, maximo } = peticionSimulada();
		const pedir = crearRecarga(fn);

		pedir();
		pedir();
		pedir();
		pedir();
		expect(fn).toHaveBeenCalledTimes(1);

		pendientes[0].resolver();
		await vi.runAllTimersAsync();
		expect(fn).toHaveBeenCalledTimes(2);

		pendientes[1].resolver();
		await vi.runAllTimersAsync();
		expect(fn).toHaveBeenCalledTimes(2);
		expect(maximo()).toBe(1);
	});

	it('un aviso sin nada en curso lanza la petición enseguida', () => {
		const { fn } = peticionSimulada();
		const pedir = crearRecarga(fn);

		pedir();
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('si la petición falla, el siguiente aviso vuelve a pedir', async () => {
		const { fn, pendientes } = peticionSimulada();
		const pedir = crearRecarga(fn);

		pedir();
		pendientes[0].fallar();
		await vi.runAllTimersAsync();

		pedir();
		expect(fn).toHaveBeenCalledTimes(2);
	});
});

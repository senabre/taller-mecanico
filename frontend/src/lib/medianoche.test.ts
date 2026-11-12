// La medianoche de Madrid del tablero (spec-008 research D6, US4-4). El proceso se pone en otra
// zona horaria para comprobar que el cálculo no usa la del navegador. `vi.stubEnv` escribe en
// process.env sin necesitar los tipos de Node; beforeAll comprueba que ha surtido efecto.
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';
import { msHastaMedianocheMadrid, programarMedianoche } from './medianoche';

vi.stubEnv('TZ', 'America/New_York');

const MINUTO = 60_000;
const HORA = 60 * MINUTO;

beforeAll(() => {
	// Si TZ no surtiera efecto, el resto daría verde sin probar nada (A2 de analyze-1).
	const desfase = new Date('2026-07-01T12:00:00Z').getTimezoneOffset();
	if (desfase !== 240) {
		throw new Error(
			`El proceso no está en America/New_York (desfase ${desfase} min; Madrid sería -120): ` +
				'la prueba no estaría comprobando nada'
		);
	}
});

describe('msHastaMedianocheMadrid', () => {
	it('cuenta hasta la siguiente 00:00 de Madrid, no la del proceso', () => {
		// 14:00 en Madrid (verano, UTC+2) = 08:00 en Nueva York.
		expect(msHastaMedianocheMadrid(new Date('2026-07-01T12:00:00Z'))).toBe(10 * HORA);
		// 23:59:59,999 en Madrid.
		expect(msHastaMedianocheMadrid(new Date('2026-07-01T21:59:59.999Z'))).toBe(1);
		// 23:59 en Madrid (invierno, UTC+1).
		expect(msHastaMedianocheMadrid(new Date('2026-11-12T22:59:00Z'))).toBe(MINUTO);
		// Justo a las 00:00 de Madrid, la siguiente es la del día después.
		expect(msHastaMedianocheMadrid(new Date('2026-11-12T23:00:00Z'))).toBe(24 * HORA);
	});

	it('cuenta bien los días de cambio de hora', () => {
		// 00:01 del 28/03/2027 (día de 23 h) → 00:00 del 29/03 (UTC+2).
		expect(msHastaMedianocheMadrid(new Date('2027-03-27T23:01:00Z'))).toBe(23 * HORA - MINUTO);
		// 00:01 del 31/10/2027 (día de 25 h) → 00:00 del 01/11 (UTC+1).
		expect(msHastaMedianocheMadrid(new Date('2027-10-30T22:01:00Z'))).toBe(25 * HORA - MINUTO);
	});
});

describe('programarMedianoche', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});
	afterEach(() => {
		vi.useRealTimers();
	});

	it('a las 23:59 de Madrid llama a fn a las 00:01, una sola vez', () => {
		vi.setSystemTime(new Date('2026-11-12T22:59:00Z'));
		const fn = vi.fn();
		programarMedianoche(fn);

		vi.advanceTimersByTime(2 * MINUTO - 1);
		expect(fn).not.toHaveBeenCalled();
		vi.advanceTimersByTime(1);
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('después de saltar se vuelve a programar para la medianoche siguiente', () => {
		vi.setSystemTime(new Date('2026-11-12T22:59:00Z'));
		const fn = vi.fn();
		programarMedianoche(fn);
		vi.advanceTimersByTime(2 * MINUTO);
		expect(fn).toHaveBeenCalledTimes(1);

		vi.advanceTimersByTime(24 * HORA - 1);
		expect(fn).toHaveBeenCalledTimes(1);
		vi.advanceTimersByTime(1);
		expect(fn).toHaveBeenCalledTimes(2);
	});

	it.each([
		['28/03/2027, día de 23 h', '2027-03-27T23:01:00Z', 23 * HORA],
		['31/10/2027, día de 25 h', '2027-10-30T22:01:00Z', 25 * HORA]
	])('el %s salta a las 00:01 de Madrid del día siguiente', (_, inicio, duracion) => {
		vi.setSystemTime(new Date(inicio));
		const fn = vi.fn();
		programarMedianoche(fn);

		vi.advanceTimersByTime(duracion - 1);
		expect(fn).not.toHaveBeenCalled();
		vi.advanceTimersByTime(1);
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('la función que devuelve cancela el temporizador', () => {
		vi.setSystemTime(new Date('2026-11-12T22:59:00Z'));
		const fn = vi.fn();
		const cancelar = programarMedianoche(fn);
		cancelar();

		vi.advanceTimersByTime(2 * 24 * HORA);
		expect(fn).not.toHaveBeenCalled();
	});
});

// La conexión de eventos del tablero (spec-008 research D5; E1 de analyze-1 y N3 de analyze-2),
// con un EventSource simulado que la prueba controla y reloj simulado.
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { conectarEventos, type FuenteEventos } from './eventos';

const CONNECTING = 0;
const OPEN = 1;
const CLOSED = 2;

class EventSourceSimulado implements FuenteEventos {
	readyState = CONNECTING;
	onopen: ((ev: Event) => void) | null = null;
	onerror: ((ev: Event) => void) | null = null;
	cerrada = false;
	private oyentes: (() => void)[] = [];

	constructor(public url: string) {}

	addEventListener(tipo: 'orden', oyente: () => void) {
		if (tipo === 'orden') this.oyentes.push(oyente);
	}

	close() {
		this.cerrada = true;
		this.readyState = CLOSED;
	}

	// Lo que haría el navegador.
	abrir() {
		this.readyState = OPEN;
		this.onopen?.(new Event('open'));
	}
	emitirOrden() {
		for (const oyente of this.oyentes) oyente();
	}
	fallar(estado: number) {
		this.readyState = estado;
		this.onerror?.(new Event('error'));
	}
}

function preparar() {
	const creadas: EventSourceSimulado[] = [];
	const alAbrir = vi.fn();
	const alEvento = vi.fn();
	const alError = vi.fn();
	const cerrar = conectarEventos({
		alAbrir,
		alEvento,
		alError,
		crear: (url) => {
			const fuente = new EventSourceSimulado(url);
			creadas.push(fuente);
			return fuente;
		},
		espera: 5000
	});
	return { creadas, alAbrir, alEvento, alError, cerrar };
}

describe('conectarEventos', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});
	afterEach(() => {
		vi.useRealTimers();
	});

	it('se conecta a /api/eventos y onopen llama a alAbrir, la primera vez y tras reconectar', () => {
		const { creadas, alAbrir } = preparar();
		expect(creadas).toHaveLength(1);
		expect(creadas[0].url).toBe('/api/eventos');

		creadas[0].abrir();
		expect(alAbrir).toHaveBeenCalledTimes(1);

		// El navegador reconecta solo tras un error de red.
		creadas[0].fallar(CONNECTING);
		creadas[0].abrir();
		expect(alAbrir).toHaveBeenCalledTimes(2);

		// Conexión nueva tras quedar cerrada.
		creadas[0].fallar(CLOSED);
		vi.advanceTimersByTime(5000);
		creadas[1].abrir();
		expect(alAbrir).toHaveBeenCalledTimes(3);
	});

	it('un evento orden llama a alEvento', () => {
		const { creadas, alEvento } = preparar();
		creadas[0].abrir();
		creadas[0].emitirOrden();
		expect(alEvento).toHaveBeenCalledTimes(1);
	});

	it('con onerror y CONNECTING llama a alError(false) y no crea otra conexión', () => {
		const { creadas, alError } = preparar();
		creadas[0].abrir();
		creadas[0].fallar(CONNECTING);

		expect(alError).toHaveBeenCalledWith(false);
		vi.advanceTimersByTime(60_000);
		expect(creadas).toHaveLength(1);
		expect(creadas[0].cerrada).toBe(false);
	});

	it('con onerror y CLOSED llama a alError(true) y crea una sola conexión nueva pasada la espera', () => {
		const { creadas, alError } = preparar();
		creadas[0].fallar(CLOSED);
		creadas[0].fallar(CLOSED);

		expect(alError).toHaveBeenCalledWith(true);
		expect(creadas[0].cerrada).toBe(true);
		vi.advanceTimersByTime(4999);
		expect(creadas).toHaveLength(1);
		vi.advanceTimersByTime(1);
		expect(creadas).toHaveLength(2);
		vi.advanceTimersByTime(60_000);
		expect(creadas).toHaveLength(2);
	});

	it('la función de cierre cierra la conexión y cancela la espera pendiente', () => {
		const { creadas, cerrar } = preparar();
		creadas[0].fallar(CLOSED);
		cerrar();

		vi.advanceTimersByTime(60_000);
		expect(creadas).toHaveLength(1);
		expect(creadas[0].cerrada).toBe(true);

		const otra = preparar();
		otra.creadas[0].abrir();
		otra.cerrar();
		expect(otra.creadas[0].cerrada).toBe(true);
	});
});

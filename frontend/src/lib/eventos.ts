// La conexión de eventos del tablero, GET /api/eventos (spec-008 research D5; E1 de analyze-1 y
// N3 de analyze-2).
//
// - Tras un error de red, el navegador reintenta solo (readyState CONNECTING): no se hace nada más.
// - Si la respuesta no es un 200 de eventos (un 401 con la sesión caducada, por ejemplo), el
//   navegador cierra la conexión para siempre (readyState CLOSED): se crea otra pasada la espera.
// - Quien llama pide el tablero en cada apertura (alAbrir) y, solo con la conexión cerrada, una vez
//   en alError(true), para que un 401 lleve a /entrar.

/** Lo que se usa de un EventSource; la prueba lo sustituye por uno simulado. */
export interface FuenteEventos {
	readonly readyState: number;
	onopen: ((ev: Event) => void) | null;
	onerror: ((ev: Event) => void) | null;
	addEventListener(tipo: 'orden', oyente: () => void): void;
	close(): void;
}

export interface OpcionesEventos {
	alAbrir: () => void;
	alEvento: () => void;
	alError: (cerrada: boolean) => void;
	crear?: (url: string) => FuenteEventos;
	/** Milisegundos antes de crear otra conexión cuando la anterior quedó cerrada. */
	espera?: number;
}

/** `EventSource.CLOSED`; el número, porque en las pruebas no hay EventSource. */
const CERRADA = 2;

/** Abre la conexión y devuelve la función que la cierra y cancela la espera pendiente. */
export function conectarEventos({
	alAbrir,
	alEvento,
	alError,
	crear = (url) => new EventSource(url),
	espera = 5000
}: OpcionesEventos): () => void {
	let actual: FuenteEventos;
	let temporizador: ReturnType<typeof setTimeout> | undefined;
	let terminado = false;

	function abrir() {
		const fuente = crear('/api/eventos');
		actual = fuente;
		fuente.onopen = () => alAbrir();
		fuente.addEventListener('orden', () => alEvento());
		fuente.onerror = () => {
			const cerrada = fuente.readyState === CERRADA;
			alError(cerrada);
			if (cerrada && temporizador === undefined && !terminado) {
				fuente.close();
				temporizador = setTimeout(() => {
					temporizador = undefined;
					if (!terminado) abrir();
				}, espera);
			}
		};
	}

	abrir();
	return () => {
		terminado = true;
		clearTimeout(temporizador);
		actual.close();
	};
}

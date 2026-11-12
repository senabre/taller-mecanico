// Una sola petición del tablero a la vez (spec-008 research D5): si llegan avisos mientras hay una
// en curso, se hace una sola más al terminar. Así una respuesta antigua nunca pisa a una nueva y
// una ráfaga de eventos no lanza una petición por cada uno.

/** Envuelve `fn` y devuelve la función que la pide. */
export function crearRecarga(fn: () => Promise<void>): () => void {
	let enCurso = false;
	let pendiente = false;

	async function lanzar() {
		enCurso = true;
		try {
			await fn();
		} catch {
			// Quien llama muestra el error; el siguiente aviso vuelve a pedir.
		} finally {
			enCurso = false;
			if (pendiente) {
				pendiente = false;
				void lanzar();
			}
		}
	}

	return () => {
		if (enCurso) {
			pendiente = true;
		} else {
			void lanzar();
		}
	};
}

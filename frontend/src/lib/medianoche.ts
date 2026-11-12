// La medianoche de Madrid para el tablero (spec-008 research D6): a las 00:01 las órdenes con
// la entrega de ayer pasan a «Retrasada». Se calcula siempre en Europe/Madrid con Intl, nunca con
// la zona del navegador (getHours, setHours…), que puede ser otra.

/** Margen tras la medianoche por si el reloj del ordenador va algo adelantado respecto al servidor. */
export const MARGEN_MS = 60_000;

const PARTES_MADRID = new Intl.DateTimeFormat('es-ES', {
	timeZone: 'Europe/Madrid',
	year: 'numeric',
	month: '2-digit',
	day: '2-digit',
	hour: '2-digit',
	minute: '2-digit',
	second: '2-digit',
	hourCycle: 'h23'
});

/** Fecha y hora de Madrid de un instante, como números. */
function enMadrid(instante: Date) {
	const p = Object.fromEntries(
		PARTES_MADRID.formatToParts(instante).map((parte) => [parte.type, Number(parte.value)])
	);
	return { anio: p.year, mes: p.month, dia: p.day, hora: p.hour, minuto: p.minute, segundo: p.second };
}

/** Milisegundos que Madrid va por delante de UTC en ese instante (3 600 000 o 7 200 000). */
function desfaseMadrid(instante: number): number {
	const m = enMadrid(new Date(instante));
	const comoSiFueraUtc = Date.UTC(m.anio, m.mes - 1, m.dia, m.hora, m.minuto, m.segundo);
	return comoSiFueraUtc - Math.floor(instante / 1000) * 1000;
}

/** Milisegundos desde `ahora` hasta la siguiente 00:00 de Madrid. */
export function msHastaMedianocheMadrid(ahora: Date): number {
	const hoy = enMadrid(ahora);
	// 00:00 de mañana en Madrid, escrita como si fuera UTC; luego se le resta el desfase de ese
	// instante, en dos pasadas para acertar también los días de cambio de hora.
	const manana = Date.UTC(hoy.anio, hoy.mes - 1, hoy.dia + 1);
	let instante = manana - desfaseMadrid(manana);
	instante = manana - desfaseMadrid(instante);
	return instante - ahora.getTime();
}

/**
 * Llama a `fn` cada día a las 00:01 de Madrid. Se vuelve a programar en cada salto, así que los
 * días de 23 y 25 horas también aciertan. Devuelve la función que lo cancela.
 */
export function programarMedianoche(fn: () => void): () => void {
	let temporizador: ReturnType<typeof setTimeout>;
	const programar = () => {
		temporizador = setTimeout(
			() => {
				programar();
				fn();
			},
			msHastaMedianocheMadrid(new Date()) + MARGEN_MS
		);
	};
	programar();
	return () => clearTimeout(temporizador);
}

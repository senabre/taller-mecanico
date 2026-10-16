import { describe, expect, it } from 'vitest';
import {
	ESTADOS_ABIERTOS,
	NOMBRE_ESTADO,
	euros,
	fecha,
	fechaHora,
	hora,
	numero
} from './formato';

// Intl separa el número y el símbolo € con un espacio no separable.
const sinNbsp = (texto: string) => texto.replace(/ /g, ' ');

describe('formato', () => {
	it('euros con punto de miles y coma decimal', () => {
		expect(sinNbsp(euros('1234.56'))).toBe('1.234,56 €');
		expect(sinNbsp(euros('133.10'))).toBe('133,10 €');
	});

	it('fechas dd/mm/aaaa', () => {
		expect(fecha('2026-10-20')).toBe('20/10/2026');
	});

	it('horas en hora de Madrid', () => {
		expect(hora('2026-10-16T10:32:00+02:00')).toBe('10:32');
		expect(fechaHora('2026-10-15T22:30:00Z')).toBe('16/10/2026 00:30');
	});

	it('números con punto de miles y coma decimal', () => {
		expect(numero(85000)).toBe('85.000');
		expect(numero(1500)).toBe('1.500');
		expect(numero('1.25')).toBe('1,25');
	});

	it('nombres de estado y estados abiertos', () => {
		expect(NOMBRE_ESTADO.en_espera_piezas).toBe('En espera de piezas');
		expect(ESTADOS_ABIERTOS).toHaveLength(6);
		expect(ESTADOS_ABIERTOS).not.toContain('facturada');
	});
});

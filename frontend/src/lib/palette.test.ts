import { describe, expect, it } from 'vitest';
import { parsePalette } from './palette';

describe('parsePalette — loose prototype schemas', () => {
  it('parses { filaments: [...] }', () => {
    const { filaments } = parsePalette(
      JSON.stringify({ filaments: [{ name: 'Black', color: '#111' }] }),
    );
    expect(filaments).toEqual([{ name: 'Black', color: '#111', active: true }]);
  });

  it('parses { colors: [...] } with label/hex aliases', () => {
    const { filaments } = parsePalette(
      JSON.stringify({ colors: [{ label: 'Snow', hex: '#fff' }] }),
    );
    expect(filaments[0]).toEqual({ name: 'Snow', color: '#fff', active: true });
  });

  it('parses a top-level array', () => {
    const { filaments } = parsePalette(JSON.stringify([{ name: 'A', color: '#abcdef' }]));
    expect(filaments[0].name).toBe('A');
  });

  it('extracts plate thickness from calibration', () => {
    const { plateThickness } = parsePalette(
      JSON.stringify({ filaments: [{ name: 'X', color: '#000' }], plate_thickness: 0.24 }),
    );
    expect(plateThickness).toBe(0.24);
  });
});

describe('parsePalette — real PIXEstL keyed-by-hex schema', () => {
  const real = JSON.stringify({
    '#0086D6': { name: 'Cyan[PLA Basic]', active: true, layers: { '5': { H: 203.8, S: 99.1, L: 45.3 } } },
    '#8BD5EE': { name: 'Matte Ice Blue[PLA Matte]', active: true, layers: {} },
    '#000000': { name: 'Black', active: false, layers: {} },
  });

  it('maps hex keys to filament base colors', () => {
    const { filaments } = parsePalette(real);
    expect(filaments).toHaveLength(3);
    expect(filaments[0]).toEqual({ name: 'Cyan[PLA Basic]', color: '#0086D6', active: true });
  });

  it('respects active:false', () => {
    const { filaments } = parsePalette(real);
    expect(filaments.find((f) => f.name === 'Black')?.active).toBe(false);
  });
});

describe('parsePalette — errors', () => {
  it('throws on invalid JSON', () => {
    expect(() => parsePalette('{not json')).toThrow();
  });
  it('throws on empty object', () => {
    expect(() => parsePalette('{}')).toThrow();
  });
});

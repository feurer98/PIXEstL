import { describe, expect, it } from 'vitest';
import { hexToRgb, matchColor, rgbToLab } from './color';
import type { Filament } from './types';

describe('hexToRgb', () => {
  it('parses a 6-digit hex with hash', () => {
    expect(hexToRgb('#1a2b3c')).toEqual([26, 43, 60]);
  });
  it('parses without hash', () => {
    expect(hexToRgb('ffffff')).toEqual([255, 255, 255]);
  });
});

describe('rgbToLab', () => {
  it('maps black to L≈0', () => {
    const [L] = rgbToLab(0, 0, 0);
    expect(L).toBeCloseTo(0, 5);
  });
  it('maps white to L≈100', () => {
    const [L] = rgbToLab(255, 255, 255);
    expect(L).toBeCloseTo(100, 1);
  });
});

describe('matchColor', () => {
  const filaments: Filament[] = [
    { name: 'Black', color: '#000000', active: true },
    { name: 'White', color: '#ffffff', active: true },
    { name: 'Red', color: '#ff0000', active: false },
  ];

  it('returns null when no active filaments', () => {
    expect(matchColor(10, 10, 10, [], true)).toBeNull();
  });

  it('picks the nearest active color (RGB)', () => {
    expect(matchColor(20, 20, 20, filaments, false)?.name).toBe('Black');
    expect(matchColor(240, 240, 240, filaments, false)?.name).toBe('White');
  });

  it('ignores inactive filaments even if closest', () => {
    // Pure red is closest to the inactive Red filament, but must not be chosen.
    const result = matchColor(255, 0, 0, filaments, false);
    expect(result?.name).not.toBe('Red');
  });

  it('CIE-Lab and RGB can both resolve a clear case', () => {
    expect(matchColor(5, 5, 5, filaments, true)?.name).toBe('Black');
  });
});

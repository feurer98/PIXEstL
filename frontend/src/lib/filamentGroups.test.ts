import { describe, expect, it } from 'vitest';
import {
  materialOf,
  displayName,
  matchesQuery,
  withIndices,
  groupByMaterial,
} from './filamentGroups';
import type { Filament } from './types';

const f = (name: string, color = '#000000', active = true): Filament => ({ name, color, active });

describe('materialOf', () => {
  it('extracts the trailing bracket material', () => {
    expect(materialOf('Matte Sakura Pink[PLA Matte]')).toBe('PLA Matte');
    expect(materialOf('Black[PLA Tough]')).toBe('PLA Tough');
  });
  it('falls back to "Sonstige" without a bracket', () => {
    expect(materialOf('Cyan')).toBe('Sonstige');
  });
});

describe('displayName', () => {
  it('strips the material bracket', () => {
    expect(displayName('Matte Sakura Pink[PLA Matte]')).toBe('Matte Sakura Pink');
  });
  it('keeps a name without a bracket', () => {
    expect(displayName('Cyan')).toBe('Cyan');
  });
});

describe('matchesQuery', () => {
  const cyan = f('Cyan[PLA Basic]', '#0086D6');
  it('matches empty query', () => {
    expect(matchesQuery(cyan, '')).toBe(true);
  });
  it('matches by name, material and hex (case-insensitive)', () => {
    expect(matchesQuery(cyan, 'cyan')).toBe(true);
    expect(matchesQuery(cyan, 'pla basic')).toBe(true);
    expect(matchesQuery(cyan, '0086d6')).toBe(true);
  });
  it('rejects non-matches', () => {
    expect(matchesQuery(cyan, 'yellow')).toBe(false);
  });
});

describe('groupByMaterial', () => {
  it('groups by material, preserves indices and order', () => {
    const filaments = [
      f('Cyan[PLA Basic]'),
      f('Ice[PLA Matte]'),
      f('White[PLA Basic]'),
    ];
    const groups = groupByMaterial(withIndices(filaments));
    expect(groups.map((g) => g.material)).toEqual(['PLA Basic', 'PLA Matte']);
    const basic = groups.find((g) => g.material === 'PLA Basic')!;
    expect(basic.items.map((i) => i.index)).toEqual([0, 2]);
  });
});

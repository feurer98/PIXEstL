import { describe, expect, it } from 'vitest';
import { oklchToHex, withOklchFallback } from './oklch';

describe('oklchToHex', () => {
  it('maps black', () => {
    expect(oklchToHex('oklch(0 0 0)')).toBe('#000000');
  });

  it('maps white', () => {
    expect(oklchToHex('oklch(1 0 0)')).toBe('#ffffff');
  });

  it('accepts percentage lightness', () => {
    expect(oklchToHex('oklch(100% 0 0)')).toBe('#ffffff');
  });

  it('produces a valid hex for a chromatic accent', () => {
    const hex = oklchToHex('oklch(0.62 0.14 62)');
    expect(hex).toMatch(/^#[0-9a-f]{6}$/);
  });

  it('returns non-oklch input unchanged', () => {
    expect(oklchToHex('#c96c1a')).toBe('#c96c1a');
    expect(oklchToHex('rgb(1,2,3)')).toBe('rgb(1,2,3)');
  });
});

describe('withOklchFallback', () => {
  it('converts only oklch values, leaving hex untouched', () => {
    const out = withOklchFallback({
      '--c-accent': 'oklch(0.62 0.14 62)',
      '--c-text': '#1a1916',
    });
    expect(out['--c-accent']).toMatch(/^#[0-9a-f]{6}$/);
    expect(out['--c-text']).toBe('#1a1916');
  });
});

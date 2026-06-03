import { describe, expect, it } from 'vitest';
import { DEFAULT_PALETTE, DEFAULT_PALETTE_TEXT } from './defaultPalette';
import { parsePalette } from './palette';

describe('bundled default palette', () => {
  it('parses into filaments', () => {
    expect(DEFAULT_PALETTE.filaments.length).toBeGreaterThan(0);
  });

  it('is valid in the real PIXEstL palette shape', () => {
    expect(() => parsePalette(DEFAULT_PALETTE_TEXT)).not.toThrow();
  });

  it('includes an active white (#FFFFFF) so additive-mode export works', () => {
    const white = DEFAULT_PALETTE.filaments.find((f) => f.color.toLowerCase() === '#ffffff');
    expect(white?.active).toBe(true);
  });

  it('produces a File of the exact bytes for upload', () => {
    const file = DEFAULT_PALETTE.toFile();
    expect(file).toBeInstanceOf(File);
    expect(file?.size).toBe(new Blob([DEFAULT_PALETTE_TEXT]).size);
  });
});

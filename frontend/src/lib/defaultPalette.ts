import { parsePalette } from './palette';
import { DEFAULT_FILAMENTS, DEFAULT_PALETTE_NAME } from './constants';
import type { Filament } from './types';
// Bundled real, calibrated palette (hex-keyed PIXEstL format). Imported as raw
// text so we can both parse it for the UI and hand the exact bytes to the
// backend export — the parser alone discards the per-layer calibration the CLI
// needs (V-MODEL-01).
import defaultPaletteText from '../assets/default-palette.json?raw';

export const DEFAULT_PALETTE_TEXT = defaultPaletteText;

export interface DefaultPalette {
  filaments: Filament[];
  name: string;
  /** A fresh File of the bundled palette, or null if it couldn't be parsed. */
  toFile: () => File | null;
}

function build(): DefaultPalette {
  try {
    const { filaments } = parsePalette(defaultPaletteText);
    return {
      filaments,
      name: DEFAULT_PALETTE_NAME,
      toFile: () =>
        new File([defaultPaletteText], DEFAULT_PALETTE_NAME, { type: 'application/json' }),
    };
  } catch {
    // Fall back to the display-only default (export will then require an upload).
    return { filaments: DEFAULT_FILAMENTS, name: DEFAULT_PALETTE_NAME, toFile: () => null };
  }
}

export const DEFAULT_PALETTE = build();

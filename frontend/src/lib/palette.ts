import type { Filament, PaletteLoadResult } from './types';

// ─── Palette parsing ─────────────────────────────────────────────────────────
// The prototype expected a loose schema: { filaments | colors: [{name, color}] }.
// The REAL PIXEstL palette files (see /palette/*.json) use a different shape:
//
//   {
//     "#0086D6": { "name": "Cyan[PLA Basic]", "active": true,
//                  "layers": { "5": {H,S,L}, "4": {...}, ... } },
//     ...
//   }
//
// i.e. an object keyed by base hex color, where each entry carries the printed
// HSL value per stacked layer count (additive mixing calibration).
//
// This normalizer accepts BOTH. The per-layer HSL data is currently DISCARDED —
// the 2D preview only uses the base hex color. Preserving / using it is tracked
// as V-MODEL-01 in docs/frontend/02-offene-punkte-und-vereinfachungen.md.

const HEX_KEY = /^#?[0-9a-fA-F]{6}$/;

interface LooseFilament {
  name?: string;
  label?: string;
  color?: string;
  hex?: string;
  active?: boolean;
}

interface PixestlEntry {
  name?: string;
  active?: boolean;
  layers?: Record<string, { H: number; S: number; L: number }>;
}

function normalizeHex(s: string): string {
  return s.startsWith('#') ? s : `#${s}`;
}

/** Detects the real PIXEstL "keyed by hex color" palette shape. */
function isPixestlKeyedPalette(data: Record<string, unknown>): boolean {
  const keys = Object.keys(data);
  if (keys.length === 0) return false;
  // Heuristic: a majority of top-level keys look like hex colors and map to objects.
  const hexLike = keys.filter((k) => HEX_KEY.test(k) && typeof data[k] === 'object');
  return hexLike.length >= Math.ceil(keys.length / 2);
}

function fromPixestlKeyed(data: Record<string, PixestlEntry>): Filament[] {
  return Object.entries(data)
    .filter(([k, v]) => HEX_KEY.test(k) && v && typeof v === 'object')
    .map(([hex, entry]) => ({
      name: entry.name || hex,
      color: normalizeHex(hex),
      // Real palettes mark availability with `active`; default to enabled.
      active: entry.active !== false,
    }));
}

function fromLooseList(list: LooseFilament[]): Filament[] {
  return list.map((f) => ({
    name: f.name || f.label || 'Filament',
    color: normalizeHex(f.color || f.hex || '#888888'),
    active: f.active !== false,
  }));
}

function extractPlateThickness(data: Record<string, unknown>): number | null {
  const cal = data['calibration'] as Record<string, unknown> | undefined;
  const pt =
    (data['plate_thickness'] as number | undefined) ??
    (data['plateThickness'] as number | undefined) ??
    (cal?.['plate_thickness'] as number | undefined) ??
    null;
  return pt === null || pt === undefined ? null : Number(pt);
}

/**
 * Parse a palette file's raw text into filaments + optional calibration.
 * Throws on invalid JSON or when no filaments can be derived.
 */
export function parsePalette(text: string): PaletteLoadResult {
  const data = JSON.parse(text);

  let filaments: Filament[];

  if (Array.isArray(data)) {
    filaments = fromLooseList(data as LooseFilament[]);
  } else if (data && typeof data === 'object') {
    const obj = data as Record<string, unknown>;
    if (Array.isArray(obj['filaments'])) {
      filaments = fromLooseList(obj['filaments'] as LooseFilament[]);
    } else if (Array.isArray(obj['colors'])) {
      filaments = fromLooseList(obj['colors'] as LooseFilament[]);
    } else if (isPixestlKeyedPalette(obj)) {
      filaments = fromPixestlKeyed(obj as Record<string, PixestlEntry>);
    } else {
      throw new Error('Unbekanntes Palettenformat');
    }
  } else {
    throw new Error('Unbekanntes Palettenformat');
  }

  if (filaments.length === 0) {
    throw new Error('Palette enthält keine Filamente');
  }

  // Calibration only applies to the keyed/object shapes.
  const plateThickness =
    Array.isArray(data) ? null : extractPlateThickness(data as Record<string, unknown>);

  return { filaments, plateThickness };
}

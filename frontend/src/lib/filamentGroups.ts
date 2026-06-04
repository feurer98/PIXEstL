import type { Filament } from './types';

// ─── Filament grouping & filtering (for the filament manager) ────────────────
// Palette names encode the material in a trailing bracket, e.g.
// "Matte Sakura Pink[PLA Matte]" → material "PLA Matte", label "Matte Sakura Pink".

/** Material group parsed from the trailing "[...]" of a filament name. */
export function materialOf(name: string): string {
  const m = name.match(/\[([^\]]+)\]\s*$/);
  return m ? m[1].trim() : 'Sonstige';
}

/** Filament name without the trailing material bracket. */
export function displayName(name: string): string {
  const stripped = name.replace(/\s*\[[^\]]+\]\s*$/, '').trim();
  return stripped || name;
}

/** Case-insensitive match against name, material, or hex. */
export function matchesQuery(f: Filament, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  return (
    f.name.toLowerCase().includes(q) ||
    f.color.toLowerCase().includes(q) ||
    materialOf(f.name).toLowerCase().includes(q)
  );
}

/** A filament paired with its index in the original array (needed for toggling). */
export interface IndexedFilament {
  filament: Filament;
  index: number;
}

export function withIndices(filaments: Filament[]): IndexedFilament[] {
  return filaments.map((filament, index) => ({ filament, index }));
}

export interface FilamentGroup {
  material: string;
  items: IndexedFilament[];
}

/** Group indexed filaments by material, preserving first-seen order. */
export function groupByMaterial(items: IndexedFilament[]): FilamentGroup[] {
  const map = new Map<string, IndexedFilament[]>();
  for (const it of items) {
    const mat = materialOf(it.filament.name);
    let bucket = map.get(mat);
    if (!bucket) {
      bucket = [];
      map.set(mat, bucket);
    }
    bucket.push(it);
  }
  return [...map.entries()].map(([material, groupItems]) => ({ material, items: groupItems }));
}

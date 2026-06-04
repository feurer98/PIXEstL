import type { Filament } from './types';

// ─── Color matching ──────────────────────────────────────────────────────────
// Ported verbatim (behaviour-preserving) from the prototype. Kept as pure
// functions so they can be unit-tested independently of React.

export type Rgb = [number, number, number];
export type Lab = [number, number, number];

export function hexToRgb(hex: string): Rgb {
  const n = parseInt(hex.replace('#', ''), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

export function srgbToLinear(c: number): number {
  const v = c / 255;
  return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
}

export function rgbToLab(r: number, g: number, b: number): Lab {
  const rl = srgbToLinear(r);
  const gl = srgbToLinear(g);
  const bl = srgbToLinear(b);
  let X = rl * 0.4124564 + gl * 0.3575761 + bl * 0.1804375;
  let Y = rl * 0.2126729 + gl * 0.7151522 + bl * 0.072175;
  let Z = rl * 0.0193339 + gl * 0.119192 + bl * 0.9503041;
  X /= 0.95047;
  Z /= 1.08883;
  const f = (v: number) => (v > 0.008856 ? Math.cbrt(v) : 7.787 * v + 16 / 116);
  const fx = f(X);
  const fy = f(Y);
  const fz = f(Z);
  return [116 * fy - 16, 500 * (fx - fy), 200 * (fy - fz)];
}

/**
 * Find the closest active filament to an RGB color.
 * Uses CIE-Lab (perceptual) or raw RGB (squared) distance.
 * Returns null when there are no active filaments.
 */
export function matchColor(
  r: number,
  g: number,
  b: number,
  filaments: Filament[],
  useLab: boolean,
): Filament | null {
  let best: Filament | null = null;
  let bestD = Infinity;
  for (const f of filaments) {
    if (!f.active) continue;
    const [fr, fg, fb] = hexToRgb(f.color);
    let d: number;
    if (useLab) {
      const [L1, a1, b1] = rgbToLab(r, g, b);
      const [L2, a2, b2] = rgbToLab(fr, fg, fb);
      d = (L1 - L2) ** 2 + (a1 - a2) ** 2 + (b1 - b2) ** 2;
    } else {
      d = (r - fr) ** 2 + (g - fg) ** 2 + (b - fb) ** 2;
    }
    if (d < bestD) {
      bestD = d;
      best = f;
    }
  }
  return best;
}

// ─── oklch() → hex fallback ──────────────────────────────────────────────────
// The themes use oklch() colors (supported in current browsers, ~2023+). For
// older browsers we compute an accurate sRGB hex equivalent rather than guess.
// Conversion: oklch → oklab → linear sRGB → gamma sRGB → hex (Björn Ottosson).
// See docs/frontend/02-offene-punkte-und-vereinfachungen.md (V-MODEL-12).

const OKLCH_RE = /^oklch\(\s*([\d.]+%?)\s+([\d.]+)\s+([\d.]+)\s*\)$/i;

function clamp01(n: number): number {
  return n < 0 ? 0 : n > 1 ? 1 : n;
}

function linearToGamma(c: number): number {
  const v = clamp01(c);
  return v <= 0.0031308 ? 12.92 * v : 1.055 * Math.pow(v, 1 / 2.4) - 0.055;
}

function toHexByte(c: number): string {
  return Math.round(clamp01(c) * 255)
    .toString(16)
    .padStart(2, '0');
}

/**
 * Convert an `oklch(L C H)` string to a `#rrggbb` hex color.
 * L may be given as a unit fraction (0..1) or a percentage. Returns the input
 * unchanged if it is not an oklch() string.
 */
export function oklchToHex(value: string): string {
  const m = value.trim().match(OKLCH_RE);
  if (!m) return value;

  const L = m[1].endsWith('%') ? parseFloat(m[1]) / 100 : parseFloat(m[1]);
  const C = parseFloat(m[2]);
  const Hdeg = parseFloat(m[3]);
  const h = (Hdeg * Math.PI) / 180;
  const a = C * Math.cos(h);
  const b = C * Math.sin(h);

  const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
  const s_ = L - 0.0894841775 * a - 1.291485548 * b;

  const l = l_ * l_ * l_;
  const mm = m_ * m_ * m_;
  const s = s_ * s_ * s_;

  const r = +4.0767416621 * l - 3.3077115913 * mm + 0.2309699292 * s;
  const g = -1.2684380046 * l + 2.6097574011 * mm - 0.3413193965 * s;
  const bl = -0.0041960863 * l - 0.7034186147 * mm + 1.707614701 * s;

  return `#${toHexByte(linearToGamma(r))}${toHexByte(linearToGamma(g))}${toHexByte(linearToGamma(bl))}`;
}

/** Returns true if the current environment supports CSS oklch() colors. */
export function supportsOklch(): boolean {
  return (
    typeof CSS !== 'undefined' &&
    typeof CSS.supports === 'function' &&
    CSS.supports('color', 'oklch(0 0 0)')
  );
}

/** Replace any oklch() values in a CSS-vars map with computed hex fallbacks. */
export function withOklchFallback(vars: Record<string, string>): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(vars)) {
    out[k] = v.startsWith('oklch(') ? oklchToHex(v) : v;
  }
  return out;
}

// ─── Domain types ────────────────────────────────────────────────────────────
// Ported from the single-file prototype (`PIXEstL Lithophane Converter.html`).
// Field names are kept identical to the prototype so the mapping to the Rust
// CLI flags (see docs/frontend/02-offene-punkte-und-vereinfachungen.md) stays
// traceable.

export type PixelMethod = 'additive' | 'full';
export type ColorMatching = 'cie-lab' | 'rgb';
export type PreviewMode = 'color' | 'backlit' | 'depth' | 'white';
export type ExportFormat = '3mf' | 'zip';

/** A single printable filament / color in the palette. */
export interface Filament {
  name: string;
  /** Base color as a hex string, e.g. `#1a1a1a`. */
  color: string;
  active: boolean;
}

/** All physical / processing parameters of the lithophane. */
export interface Settings {
  // Dimensions
  width: number;
  height: number;
  plateThickness: number;
  curve: number;
  // Color layer
  colorPixelWidth: number;
  colorLayerThickness: number;
  colorLayers: number;
  pixelMethod: PixelMethod;
  colorMatching: ColorMatching;
  amsColors: number;
  // Texture layer
  texturePixelWidth: number;
  textureMin: number;
  textureMax: number;
  textureColor: string;
  // Toggles
  enableColor: boolean;
  enableTexture: boolean;
}

/** Loaded source image metadata. The decoded `HTMLImageElement` is held in a ref. */
export interface ImageInfo {
  name: string;
  w: number;
  h: number;
}

/** Result of rasterizing the image into lithophane blocks. */
export interface LithoStats {
  blocksX: number;
  blocksY: number;
  /** filament name → number of blocks assigned to it (color mode only). */
  filamentUsage: Record<string, number>;
}

/** The result of loading a palette file, including optional calibration data. */
export interface PaletteLoadResult {
  filaments: Filament[];
  /** Plate thickness in mm if the palette carried calibration data. */
  plateThickness: number | null;
}

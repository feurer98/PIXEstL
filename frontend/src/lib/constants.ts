import type { Filament, Settings } from './types';

// ─── Default palette ─────────────────────────────────────────────────────────
// Mirrors DEFAULT_FILAMENTS from the prototype. Used until the user loads a
// real palette file (which uses a different, richer schema — see palette.ts).
export const DEFAULT_FILAMENTS: Filament[] = [
  { name: 'Black', color: '#1a1a1a', active: true },
  { name: 'White', color: '#f2f0eb', active: true },
  { name: 'Cyan', color: '#00b4d8', active: true },
  { name: 'Magenta', color: '#d62876', active: true },
  { name: 'Yellow', color: '#f4d03f', active: true },
  { name: 'Orange', color: '#e8622a', active: true },
  { name: 'Red', color: '#c0392b', active: true },
  { name: 'Blue', color: '#2471a3', active: true },
  { name: 'Green', color: '#1e8449', active: true },
  { name: 'Light Gray', color: '#bdc3c7', active: true },
  { name: 'Dark Gray', color: '#555250', active: true },
  { name: 'Skin', color: '#e8c8a0', active: true },
];

export const DEFAULT_PALETTE_NAME = 'default-palette.json';

export const DEFAULT_SETTINGS: Settings = {
  width: 100,
  height: 80,
  plateThickness: 0.2,
  curve: 0,
  colorPixelWidth: 0.8,
  colorLayerThickness: 0.1,
  colorLayers: 5,
  pixelMethod: 'additive',
  colorMatching: 'cie-lab',
  amsColors: 0,
  texturePixelWidth: 0.25,
  textureMin: 0.3,
  textureMax: 1.8,
  textureColor: '#ffffff',
  enableColor: true,
  enableTexture: true,
};

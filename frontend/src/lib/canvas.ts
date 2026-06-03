import { matchColor } from './color';
import type { Filament, LithoStats, PreviewMode, Settings } from './types';

// ─── Canvas processing ───────────────────────────────────────────────────────
// Renders a 2D rasterized preview of the lithophane into `canvas` and returns
// block statistics.
//
// The canvas bitmap is sized to the block grid's own aspect ratio (one integer
// cell per block), so paired with CSS `object-fit: contain` the preview shows
// the true lithophane proportions — fully visible, never cropped or distorted.
//
// NOTE: This is still a 2D approximation. It does NOT model curvature, stacked
// color layers, additive transparent-filament mixing, or AMS color limits.
// See docs/frontend/02-offene-punkte-und-vereinfachungen.md (V-MODEL-02..05).

export interface ProcessOptions {
  mode: PreviewMode;
  showGrid: boolean;
  filaments: Filament[];
}

const EMPTY_STATS: LithoStats = { blocksX: 0, blocksY: 0, filamentUsage: {} };

// Longest side of the rendered bitmap, in px. Cells are sized so the grid fills
// roughly this, keeping the preview crisp without an oversized canvas.
const TARGET_PX = 720;

export function processCanvas(
  imgEl: CanvasImageSource | null,
  settings: Settings,
  canvas: HTMLCanvasElement | null,
  { mode, showGrid, filaments }: ProcessOptions,
): LithoStats {
  if (!imgEl || !canvas) return EMPTY_STATS;
  const ctx = canvas.getContext('2d');
  if (!ctx) return EMPTY_STATS;

  // Block count derives from physical size / pixel width (mm). The height was
  // derived from the image aspect on load, so blocksX:blocksY ≈ image aspect.
  const blocksX = Math.max(4, Math.round(settings.width / settings.colorPixelWidth));
  const blocksY = Math.max(4, Math.round(settings.height / settings.colorPixelWidth));

  // One integer cell per block; size the bitmap to the grid's aspect ratio.
  const cell = Math.max(1, Math.min(24, Math.round(TARGET_PX / Math.max(blocksX, blocksY))));
  canvas.width = blocksX * cell;
  canvas.height = blocksY * cell;

  // Downscale the image to one representative sample per block.
  const off = document.createElement('canvas');
  off.width = blocksX;
  off.height = blocksY;
  const octx = off.getContext('2d');
  if (!octx) return EMPTY_STATS;
  octx.drawImage(imgEl, 0, 0, blocksX, blocksY);
  const pix = octx.getImageData(0, 0, blocksX, blocksY).data;

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  const useLab = settings.colorMatching === 'cie-lab';
  const usage: Record<string, number> = {};
  const activeFilaments = filaments.filter((f) => f.active);

  for (let y = 0; y < blocksY; y++) {
    for (let x = 0; x < blocksX; x++) {
      const i = (y * blocksX + x) * 4;
      const r = pix[i];
      const g = pix[i + 1];
      const b = pix[i + 2];
      const gray = 0.299 * r + 0.587 * g + 0.114 * b;
      const depth = 1 - gray / 255; // 0 = thin, 1 = thick

      let color: string;
      if (mode === 'color' && activeFilaments.length > 0) {
        const match = matchColor(r, g, b, activeFilaments, useLab);
        if (match) {
          color = match.color;
          usage[match.name] = (usage[match.name] || 0) + 1;
        } else {
          color = '#888';
        }
      } else if (mode === 'backlit') {
        const t = 1 - depth;
        color = `rgb(${Math.round(255 * Math.pow(t, 0.6))},${Math.round(
          180 * Math.pow(t, 0.8),
        )},${Math.round(60 * t)})`;
      } else {
        // 'depth' and 'white' both render as grayscale by brightness.
        const v = Math.round((1 - depth) * 255);
        color = `rgb(${v},${v},${v})`;
      }

      ctx.fillStyle = color;
      ctx.fillRect(x * cell, y * cell, cell, cell);
      if (showGrid && cell > 4) {
        ctx.strokeStyle = 'rgba(0,0,0,0.08)';
        ctx.lineWidth = 0.5;
        ctx.strokeRect(x * cell, y * cell, cell, cell);
      }
    }
  }

  return { blocksX, blocksY, filamentUsage: usage };
}

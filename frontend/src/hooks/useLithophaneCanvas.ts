import { useEffect, useState, type RefObject } from 'react';
import { processCanvas, type ProcessOptions } from '../lib/canvas';
import type { LithoStats, Settings } from '../lib/types';

/**
 * Redraws the lithophane preview canvas whenever the inputs change and returns
 * the resulting block statistics. The draw is debounced because processCanvas
 * runs synchronously over every block and fine pixel widths produce many blocks
 * (see V-MODEL-06 — moving this to a Web Worker is a follow-up).
 */
export function useLithophaneCanvas(
  canvasRef: RefObject<HTMLCanvasElement>,
  imgElRef: RefObject<HTMLImageElement | null>,
  settings: Settings,
  opts: ProcessOptions,
  /** Bumps when a new image is decoded so we redraw even though imgElRef is a ref. */
  imageKey: unknown,
  debounceMs = 80,
): LithoStats | null {
  const [stats, setStats] = useState<LithoStats | null>(null);

  useEffect(() => {
    if (!imgElRef.current || !canvasRef.current) return;
    const id = window.setTimeout(() => {
      const s = processCanvas(imgElRef.current, settings, canvasRef.current, opts);
      setStats(s);
    }, debounceMs);
    return () => window.clearTimeout(id);
    // opts is recreated each render; depend on its primitive fields instead.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [settings, opts.mode, opts.showGrid, opts.filaments, imageKey, debounceMs]);

  return stats;
}

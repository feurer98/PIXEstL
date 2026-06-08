import { type RefObject } from 'react';
import type { PreviewMode } from '../../lib/types';
import s from './LithoPreview.module.css';

const MODES: [PreviewMode, string][] = [
  ['color', 'Farbe'],
  ['backlit', 'Hinterleuchtet'],
  ['depth', 'Tiefenkarte'],
  ['white', 'Weiß'],
];

interface LithoPreviewProps {
  hasImage: boolean;
  canvasRef: RefObject<HTMLCanvasElement>;
  previewMode: PreviewMode;
  onModeChange: (mode: PreviewMode) => void;
}

export function LithoPreview({ hasImage, canvasRef, previewMode, onModeChange }: LithoPreviewProps) {
  return (
    <div className={s.wrap}>
      <div className={s.badge}>
        Lithophan Vorschau
      </div>
      <div className={s.modes} role="tablist" aria-label="Vorschaumodus">
        {MODES.map(([v, lbl]) => (
          <button
            key={v}
            role="tab"
            onClick={() => onModeChange(v)}
            data-selected={previewMode === v}
            aria-selected={previewMode === v}
            className={s.modeBtn}
          >
            {lbl}
          </button>
        ))}
      </div>
      {!hasImage ? (
        <div className={s.empty}>
          <span className={s.emptyText}>— Bild laden —</span>
        </div>
      ) : (
        <canvas
          ref={canvasRef}
          width={800}
          height={600}
          className={s.canvas}
        />
      )}
    </div>
  );
}

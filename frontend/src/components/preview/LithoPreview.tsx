import { type RefObject } from 'react';
import { useTheme } from '../../theme/ThemeContext';
import type { PreviewMode } from '../../lib/types';

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
  const { theme } = useTheme();
  return (
    <div style={{ position: 'relative', overflow: 'hidden', background: 'var(--c-preview2-bg)' }}>
      <div
        style={{
          position: 'absolute',
          top: 8,
          left: 10,
          zIndex: 2,
          fontSize: 9,
          fontWeight: 700,
          letterSpacing: '0.1em',
          textTransform: 'uppercase',
          color: 'rgba(255,255,255,0.35)',
          pointerEvents: 'none',
        }}
      >
        Lithophan Vorschau
      </div>
      <div style={{ position: 'absolute', top: 6, right: 8, zIndex: 3, display: 'flex', gap: 2 }}>
        {MODES.map(([v, lbl]) => {
          const selected = previewMode === v;
          return (
            <button
              key={v}
              onClick={() => onModeChange(v)}
              style={{
                fontSize: 9,
                fontWeight: 600,
                letterSpacing: '0.06em',
                textTransform: 'uppercase',
                padding: '3px 7px',
                border: 'none',
                borderRadius: 3,
                cursor: 'pointer',
                background: selected ? theme.accent : 'rgba(255,255,255,0.07)',
                color: selected ? '#fff' : 'rgba(255,255,255,0.4)',
                transition: 'all 0.15s',
              }}
            >
              {lbl}
            </button>
          );
        })}
      </div>
      {!hasImage ? (
        <div style={{ width: '100%', height: '100%', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
          <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.15)', fontFamily: 'DM Mono' }}>— Bild laden —</span>
        </div>
      ) : (
        <canvas
          ref={canvasRef}
          width={800}
          height={600}
          style={{ width: '100%', height: '100%', objectFit: 'cover', display: 'block', imageRendering: 'pixelated' }}
        />
      )}
    </div>
  );
}

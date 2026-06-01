import { useEffect, useRef, useState, type RefObject } from 'react';
import { useTheme } from '../../theme/ThemeContext';
import type { ImageInfo } from '../../lib/types';

interface SourcePreviewProps {
  image: ImageInfo | null;
  imgElRef: RefObject<HTMLImageElement | null>;
  onFile: (file: File | null | undefined) => void;
  /** Bumps when a new image is decoded. */
  imageKey: unknown;
}

export function SourcePreview({ image, imgElRef, onFile, imageKey }: SourcePreviewProps) {
  const { theme } = useTheme();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [dragging, setDragging] = useState(false);

  // Draw the source image "contain"-fit onto a dark backdrop.
  useEffect(() => {
    const canvas = canvasRef.current;
    const imgEl = imgElRef.current;
    if (!canvas || !imgEl) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.fillStyle = '#111';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    const iAR = imgEl.naturalWidth / imgEl.naturalHeight;
    const cAR = canvas.width / canvas.height;
    let dw: number, dh: number, dx: number, dy: number;
    if (iAR > cAR) {
      dw = canvas.width;
      dh = dw / iAR;
      dx = 0;
      dy = (canvas.height - dh) / 2;
    } else {
      dh = canvas.height;
      dw = dh * iAR;
      dy = 0;
      dx = (canvas.width - dw) / 2;
    }
    ctx.drawImage(imgEl, dx, dy, dw, dh);
  }, [imageKey, imgElRef]);

  return (
    <div
      style={{
        position: 'relative',
        borderRight: '1px solid var(--c-border)',
        overflow: 'hidden',
        background: '#111',
      }}
    >
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
          color: 'rgba(255,255,255,0.4)',
          pointerEvents: 'none',
        }}
      >
        Quellbild
      </div>
      {!image ? (
        <div
          onDragOver={(e) => {
            e.preventDefault();
            setDragging(true);
          }}
          onDragLeave={() => setDragging(false)}
          onDrop={(e) => {
            e.preventDefault();
            setDragging(false);
            onFile(e.dataTransfer.files[0]);
          }}
          onClick={() => fileInputRef.current?.click()}
          style={{
            width: '100%',
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            cursor: 'pointer',
            gap: 10,
            background: dragging ? 'rgba(255,255,255,0.04)' : 'transparent',
            border: dragging ? '2px dashed rgba(255,255,255,0.25)' : '2px dashed transparent',
          }}
        >
          <svg width="36" height="36" viewBox="0 0 36 36" fill="none">
            <rect x="4" y="6" width="28" height="20" rx="3" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" />
            <circle cx="12" cy="13" r="2.5" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" />
            <path d="M4 20L11 14L17 20L23 16L32 23" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" strokeLinejoin="round" />
            <path d="M18 30V26M15 28L18 26L21 28" stroke={theme.accentRgb} strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
          <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: 12, color: 'rgba(255,255,255,0.55)', fontWeight: 500 }}>Bild hier ablegen</div>
            <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.25)', marginTop: 3 }}>
              oder klicken zum Auswählen · PNG, JPG, WEBP
            </div>
          </div>
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            style={{ display: 'none' }}
            onChange={(e) => onFile(e.target.files?.[0])}
          />
        </div>
      ) : (
        <canvas
          ref={canvasRef}
          width={800}
          height={600}
          style={{ width: '100%', height: '100%', objectFit: 'cover', display: 'block' }}
        />
      )}
    </div>
  );
}

import { useRef, useState } from 'react';
import type { ImageInfo } from '../../lib/types';
import s from './SourcePreview.module.css';

interface SourcePreviewProps {
  image: ImageInfo | null;
  /** Data URL of the source image for display. */
  previewUrl: string | null;
  onFile: (file: File | null | undefined) => void;
}

export function SourcePreview({ image, previewUrl, onFile }: SourcePreviewProps) {
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [dragging, setDragging] = useState(false);

  return (
    <div className={s.wrap}>
      <div className={s.badge}>Quellbild</div>
      {!image || !previewUrl ? (
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
          data-dragging={dragging}
          className={s.dropzone}
        >
          <svg width="36" height="36" viewBox="0 0 36 36" fill="none">
            <rect x="4" y="6" width="28" height="20" rx="3" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" />
            <circle cx="12" cy="13" r="2.5" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" />
            <path d="M4 20L11 14L17 20L23 16L32 23" stroke="rgba(255,255,255,0.2)" strokeWidth="1.5" strokeLinejoin="round" />
            <path d="M18 30V26M15 28L18 26L21 28" className={s.uploadAccent} strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
          <div className={s.dropText}>
            <div className={s.dropTitle}>Bild hier ablegen</div>
            <div className={s.dropHint}>oder klicken zum Auswählen · PNG, JPG, WEBP</div>
          </div>
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            className={s.hiddenInput}
            onChange={(e) => onFile(e.target.files?.[0])}
          />
        </div>
      ) : (
        // object-fit: contain → whole image always visible, aspect kept, never cropped.
        <img src={previewUrl} alt={image.name} className={s.image} />
      )}
    </div>
  );
}

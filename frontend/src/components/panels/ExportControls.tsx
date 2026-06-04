import { SectionLabel } from '../ui/SectionLabel';
import { Spinner } from '../ui/Spinner';
import type { ExportFormat } from '../../lib/types';
import type { ExportState } from '../../hooks/useExport';
import s from './ExportControls.module.css';

interface ExportControlsProps {
  hasImage: boolean;
  exportFormat: ExportFormat;
  onFormatChange: (format: ExportFormat) => void;
  exportState: ExportState;
  exportError?: string | null;
  onExport: () => void;
}

const FORMATS: [ExportFormat, string][] = [
  ['3mf', '3MF'],
  ['zip', 'ZIP'],
];

export function ExportControls({
  hasImage,
  exportFormat,
  onFormatChange,
  exportState,
  exportError,
  onExport,
}: ExportControlsProps) {
  return (
    <>
      <SectionLabel>Export</SectionLabel>
      <div className={s.formats}>
        {FORMATS.map(([v, lbl]) => {
          const selected = exportFormat === v;
          return (
            <button
              key={v}
              onClick={() => onFormatChange(v)}
              data-selected={selected}
              className={s.formatBtn}
            >
              {lbl}
            </button>
          );
        })}
      </div>

      <button
        onClick={onExport}
        disabled={!hasImage || exportState === 'exporting'}
        data-has-image={hasImage}
        data-state={exportState}
        className={s.exportBtn}
      >
        {exportState === 'idle' && (
          <>
            <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
              <path
                d="M5.5 1v6M3 4.5l2.5 2.5 2.5-2.5M1 9h9"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
            {exportFormat.toUpperCase()} exportieren
          </>
        )}
        {exportState === 'exporting' && (
          <>
            <Spinner />
            Wird generiert…
          </>
        )}
        {exportState === 'done' && <>✓ Bereit zum Download</>}
        {exportState === 'error' && <>Fehler — erneut versuchen</>}
      </button>
      {!hasImage && (
        <p className={s.hint}>
          Zuerst ein Bild laden
        </p>
      )}
      {exportState === 'error' && exportError && <p className={s.error}>{exportError}</p>}
    </>
  );
}

import { useTheme } from '../../theme/ThemeContext';
import { SectionLabel } from '../ui/SectionLabel';
import { Spinner } from '../ui/Spinner';
import type { ExportFormat } from '../../lib/types';
import type { ExportState } from '../../hooks/useExport';

interface ExportControlsProps {
  hasImage: boolean;
  exportFormat: ExportFormat;
  onFormatChange: (format: ExportFormat) => void;
  exportState: ExportState;
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
  onExport,
}: ExportControlsProps) {
  const { theme } = useTheme();
  const buttonBg =
    exportState === 'done' ? theme.ok : hasImage ? theme.accent : theme.border;

  return (
    <>
      <SectionLabel>Export</SectionLabel>
      <div style={{ display: 'flex', gap: 4, marginBottom: 10 }}>
        {FORMATS.map(([v, lbl]) => {
          const selected = exportFormat === v;
          return (
            <button
              key={v}
              onClick={() => onFormatChange(v)}
              style={{
                flex: 1,
                padding: '5px 0',
                border: `1px solid ${selected ? 'var(--c-border-strong)' : 'transparent'}`,
                borderRadius: 5,
                cursor: 'pointer',
                fontSize: 10,
                fontWeight: selected ? 600 : 400,
                background: selected ? 'var(--c-surface)' : 'var(--c-panel2)',
                color: selected ? 'var(--c-text)' : 'var(--c-text-mid)',
              }}
            >
              {lbl}
            </button>
          );
        })}
      </div>

      <button
        onClick={onExport}
        disabled={!hasImage || exportState !== 'idle'}
        style={{
          width: '100%',
          padding: '10px 0',
          borderRadius: 7,
          border: 'none',
          cursor: hasImage ? 'pointer' : 'not-allowed',
          background: buttonBg,
          color: hasImage ? '#fff' : 'var(--c-text-faint)',
          fontFamily: 'DM Sans',
          fontWeight: 600,
          fontSize: 12,
          letterSpacing: '-0.01em',
          transition: 'all 0.2s',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          gap: 7,
        }}
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
        <p style={{ fontSize: 9, color: 'var(--c-text-faint)', textAlign: 'center', marginTop: 6 }}>
          Zuerst ein Bild laden
        </p>
      )}
    </>
  );
}

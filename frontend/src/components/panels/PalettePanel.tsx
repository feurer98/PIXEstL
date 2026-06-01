import { useRef } from 'react';
import { SectionLabel } from '../ui/SectionLabel';
import { FilamentList } from './FilamentList';
import { UsageBreakdown } from './UsageBreakdown';
import { ExportControls } from './ExportControls';
import type { ExportFormat, Filament, LithoStats, PreviewMode } from '../../lib/types';
import type { ExportState } from '../../hooks/useExport';

interface PalettePanelProps {
  paletteName: string;
  filaments: Filament[];
  paletteError: string | null;
  onLoadPalette: (file: File | null | undefined) => void;
  onToggleFilament: (index: number) => void;
  stats: LithoStats | null;
  previewMode: PreviewMode;
  hasImage: boolean;
  exportFormat: ExportFormat;
  onFormatChange: (format: ExportFormat) => void;
  exportState: ExportState;
  onExport: () => void;
}

export function PalettePanel(props: PalettePanelProps) {
  const paletteInputRef = useRef<HTMLInputElement | null>(null);
  const activeCount = props.filaments.filter((f) => f.active).length;
  const showUsage =
    props.stats &&
    Object.keys(props.stats.filamentUsage).length > 0 &&
    props.previewMode === 'color';

  return (
    <div
      style={{
        padding: '14px 16px',
        background: 'var(--c-panel)',
        overflowY: 'auto',
        display: 'flex',
        flexDirection: 'column',
        gap: 0,
      }}
    >
      <SectionLabel>Palette</SectionLabel>
      <div style={{ display: 'flex', gap: 5, marginBottom: 10 }}>
        <div
          style={{
            flex: 1,
            padding: '5px 8px',
            background: 'var(--c-surface)',
            border: '1px solid var(--c-border)',
            borderRadius: 5,
            fontSize: 10,
            fontFamily: 'DM Mono',
            color: 'var(--c-text-mid)',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}
        >
          {props.paletteName}
        </div>
        <button
          onClick={() => paletteInputRef.current?.click()}
          style={{
            padding: '5px 9px',
            border: '1px solid var(--c-border)',
            borderRadius: 5,
            cursor: 'pointer',
            background: 'var(--c-surface)',
            color: 'var(--c-text)',
            fontSize: 10,
            fontWeight: 500,
            whiteSpace: 'nowrap',
          }}
        >
          Öffnen
        </button>
        <input
          ref={paletteInputRef}
          type="file"
          accept=".json"
          style={{ display: 'none' }}
          onChange={(e) => props.onLoadPalette(e.target.files?.[0])}
        />
      </div>

      {props.paletteError && (
        <div style={{ fontSize: 9, color: '#c0392b', marginBottom: 8 }}>{props.paletteError}</div>
      )}

      <FilamentList filaments={props.filaments} onToggle={props.onToggleFilament} />

      <div style={{ fontSize: 9, color: 'var(--c-text-faint)', textAlign: 'center', marginBottom: 10 }}>
        {activeCount}/{props.filaments.length} Filamente aktiv · Klicken zum Umschalten
      </div>

      {showUsage && props.stats && <UsageBreakdown stats={props.stats} filaments={props.filaments} />}

      <ExportControls
        hasImage={props.hasImage}
        exportFormat={props.exportFormat}
        onFormatChange={props.onFormatChange}
        exportState={props.exportState}
        onExport={props.onExport}
      />
    </div>
  );
}

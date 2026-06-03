import { useRef } from 'react';
import { SectionLabel } from '../ui/SectionLabel';
import { FilamentList } from './FilamentList';
import { UsageBreakdown } from './UsageBreakdown';
import { ExportControls } from './ExportControls';
import type { ExportFormat, Filament, LithoStats, PreviewMode } from '../../lib/types';
import type { ExportState } from '../../hooks/useExport';
import s from './PalettePanel.module.css';

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
  exportError: string | null;
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
    <div className={s.column}>
      <SectionLabel>Palette</SectionLabel>
      <div className={s.loadRow}>
        <div className={s.name}>
          {props.paletteName}
        </div>
        <button
          onClick={() => paletteInputRef.current?.click()}
          className={s.openBtn}
        >
          Öffnen
        </button>
        <input
          ref={paletteInputRef}
          type="file"
          accept=".json"
          className={s.hiddenInput}
          onChange={(e) => props.onLoadPalette(e.target.files?.[0])}
        />
      </div>

      {props.paletteError && (
        <div className={s.error}>{props.paletteError}</div>
      )}

      <FilamentList filaments={props.filaments} onToggle={props.onToggleFilament} />

      <div className={s.activeCount}>
        {activeCount}/{props.filaments.length} Filamente aktiv · Klicken zum Umschalten
      </div>

      {showUsage && props.stats && <UsageBreakdown stats={props.stats} filaments={props.filaments} />}

      <ExportControls
        hasImage={props.hasImage}
        exportFormat={props.exportFormat}
        onFormatChange={props.onFormatChange}
        exportState={props.exportState}
        exportError={props.exportError}
        onExport={props.onExport}
      />
    </div>
  );
}

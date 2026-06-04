import { useRef } from 'react';
import { SectionLabel } from '../ui/SectionLabel';
import { UsageBreakdown } from './UsageBreakdown';
import { ExportControls } from './ExportControls';
import type { ExportFormat, Filament, LithoStats, PreviewMode } from '../../lib/types';
import type { ExportState } from '../../hooks/useExport';
import s from './PalettePanel.module.css';

const MAX_CHIPS = 18;

interface PalettePanelProps {
  paletteName: string;
  filaments: Filament[];
  paletteError: string | null;
  onLoadPalette: (file: File | null | undefined) => void;
  /** Open the filament manager slide-over. */
  onManage: () => void;
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
  const active = props.filaments.filter((f) => f.active);
  const showUsage =
    props.stats &&
    Object.keys(props.stats.filamentUsage).length > 0 &&
    props.previewMode === 'color';

  return (
    <div className={s.column}>
      <SectionLabel>Palette</SectionLabel>
      <div className={s.loadRow}>
        <div className={s.name}>{props.paletteName}</div>
        <button onClick={() => paletteInputRef.current?.click()} className={s.openBtn}>
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

      {props.paletteError && <div className={s.error}>{props.paletteError}</div>}

      {/* Compact summary; the full list + tools live in the manager slide-over. */}
      <div className={s.summary}>
        <div className={s.chips}>
          {active.length === 0 && <span className={s.emptyChips}>Keine Filamente aktiv</span>}
          {active.slice(0, MAX_CHIPS).map((f, i) => (
            <div key={`${f.name}-${i}`} className={s.chip} style={{ background: f.color }} title={f.name} />
          ))}
          {active.length > MAX_CHIPS && <span className={s.more}>+{active.length - MAX_CHIPS}</span>}
        </div>
        <button className={s.manageBtn} onClick={props.onManage}>
          Filamente verwalten
          <span className={s.manageCount}>
            {active.length}/{props.filaments.length}
          </span>
        </button>
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

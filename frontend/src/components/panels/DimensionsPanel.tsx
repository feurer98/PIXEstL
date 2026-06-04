import type { Dispatch } from 'react';
import { Slider } from '../ui/Slider';
import { SectionLabel } from '../ui/SectionLabel';
import { PanelColumn } from './PanelColumn';
import type { Settings } from '../../lib/types';
import type { SettingsAction } from '../../state/settingsReducer';
import s from './DimensionsPanel.module.css';

interface DimensionsPanelProps {
  settings: Settings;
  dispatch: Dispatch<SettingsAction>;
  lockAspect: boolean;
  onToggleLock: () => void;
  calibPlateThickness: number | null;
}

export function DimensionsPanel({
  settings,
  dispatch,
  lockAspect,
  onToggleLock,
  calibPlateThickness,
}: DimensionsPanelProps) {
  return (
    <PanelColumn>
      <SectionLabel>Abmessungen</SectionLabel>

      <Slider
        label="Breite"
        value={settings.width}
        min={20}
        max={300}
        step={1}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_WIDTH', value: v, lockAspect })}
      />

      <div className={s.lockRow}>
        <div className={s.rule} />
        <button
          title={lockAspect ? 'Seitenverhältnis gesperrt' : 'Seitenverhältnis frei'}
          aria-pressed={lockAspect}
          onClick={onToggleLock}
          data-locked={lockAspect}
          className={s.lockBtn}
        >
          {lockAspect ? (
            <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
              <rect x="2" y="5" width="7" height="5" rx="1" className={s.lockIcon} strokeWidth="1.3" />
              <path d="M3.5 5V3.5a2 2 0 0 1 4 0V5" className={s.lockIcon} strokeWidth="1.3" strokeLinecap="round" />
            </svg>
          ) : (
            <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
              <rect x="2" y="5" width="7" height="5" rx="1" className={s.unlockIcon} strokeWidth="1.3" />
              <path d="M3.5 5V3.5a2 2 0 0 1 4 0" className={s.unlockIcon} strokeWidth="1.3" strokeLinecap="round" />
            </svg>
          )}
        </button>
        <div className={s.rule} />
      </div>

      <Slider
        label="Höhe"
        value={settings.height}
        min={20}
        max={250}
        step={1}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_HEIGHT', value: v, lockAspect })}
      />

      {calibPlateThickness !== null && (
        <div className={s.calib}>
          <span className={s.calibLabel}>
            JSON Kalibrierung
          </span>
          <span className={s.calibValue}>
            {calibPlateThickness} mm
          </span>
        </div>
      )}

      <Slider
        label="Plattendicke"
        value={settings.plateThickness}
        min={0.1}
        max={2}
        step={0.05}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'plateThickness', value: v })}
      />
      <Slider
        label="Wölbung"
        value={settings.curve}
        min={0}
        max={180}
        step={5}
        unit="°"
        hint="0 = flach  →  180 = Vollzylinder"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'curve', value: v })}
      />

      <div className={s.summary}>
        <span className={s.summaryText}>
          {settings.width} × {settings.height} mm
          {settings.curve > 0 && <span className={s.summaryArc}> · {settings.curve}° Bogen</span>}
        </span>
      </div>
    </PanelColumn>
  );
}

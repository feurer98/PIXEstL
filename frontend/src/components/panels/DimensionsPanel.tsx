import type { Dispatch } from 'react';
import { useTheme } from '../../theme/ThemeContext';
import { Slider } from '../ui/Slider';
import { SectionLabel } from '../ui/SectionLabel';
import type { Settings } from '../../lib/types';
import type { SettingsAction } from '../../state/settingsReducer';

interface DimensionsPanelProps {
  settings: Settings;
  dispatch: Dispatch<SettingsAction>;
  lockAspect: boolean;
  onToggleLock: () => void;
  calibPlateThickness: number | null;
}

const COL_STYLE: React.CSSProperties = {
  borderRight: '1px solid var(--c-border)',
  padding: '14px 16px',
  overflowY: 'auto',
  background: 'var(--c-surface)',
};

export function DimensionsPanel({
  settings,
  dispatch,
  lockAspect,
  onToggleLock,
  calibPlateThickness,
}: DimensionsPanelProps) {
  const { theme } = useTheme();
  return (
    <div style={COL_STYLE}>
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

      <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8, marginTop: -4 }}>
        <div style={{ flex: 1, height: 1, background: 'var(--c-border)' }} />
        <button
          title={lockAspect ? 'Seitenverhältnis gesperrt' : 'Seitenverhältnis frei'}
          aria-pressed={lockAspect}
          onClick={onToggleLock}
          style={{
            width: 24,
            height: 24,
            borderRadius: 6,
            border: `1.5px solid ${lockAspect ? 'var(--c-accent)' : 'var(--c-border)'}`,
            background: lockAspect ? 'var(--c-accent-light)' : 'var(--c-surface)',
            cursor: 'pointer',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
            transition: 'all 0.15s',
          }}
        >
          {lockAspect ? (
            <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
              <rect x="2" y="5" width="7" height="5" rx="1" stroke={theme.accentText} strokeWidth="1.3" />
              <path d="M3.5 5V3.5a2 2 0 0 1 4 0V5" stroke={theme.accentText} strokeWidth="1.3" strokeLinecap="round" />
            </svg>
          ) : (
            <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
              <rect x="2" y="5" width="7" height="5" rx="1" stroke={theme.textFaint} strokeWidth="1.3" />
              <path d="M3.5 5V3.5a2 2 0 0 1 4 0" stroke={theme.textFaint} strokeWidth="1.3" strokeLinecap="round" />
            </svg>
          )}
        </button>
        <div style={{ flex: 1, height: 1, background: 'var(--c-border)' }} />
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
        <div
          style={{
            marginBottom: 10,
            padding: '6px 9px',
            background: 'var(--c-accent-light)',
            borderRadius: 6,
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
        >
          <span style={{ fontSize: 9, color: 'var(--c-accent-text)', letterSpacing: '0.04em', textTransform: 'uppercase', fontWeight: 600 }}>
            JSON Kalibrierung
          </span>
          <span style={{ fontFamily: 'DM Mono', fontSize: 11, color: 'var(--c-accent-text)', fontWeight: 600 }}>
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

      <div style={{ marginTop: 8, padding: '7px 9px', background: 'var(--c-accent-light)', borderRadius: 6 }}>
        <span style={{ fontSize: 10, color: 'var(--c-accent-text)', fontFamily: 'DM Mono' }}>
          {settings.width} × {settings.height} mm
          {settings.curve > 0 && <span style={{ color: 'var(--c-text-faint)' }}> · {settings.curve}° Bogen</span>}
        </span>
      </div>
    </div>
  );
}

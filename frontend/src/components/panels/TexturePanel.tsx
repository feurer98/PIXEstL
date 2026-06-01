import type { Dispatch } from 'react';
import { Slider } from '../ui/Slider';
import { SwitchRow } from '../ui/SwitchRow';
import { SectionLabel } from '../ui/SectionLabel';
import type { Filament, Settings } from '../../lib/types';
import type { SettingsAction } from '../../state/settingsReducer';

interface TexturePanelProps {
  settings: Settings;
  dispatch: Dispatch<SettingsAction>;
  filaments: Filament[];
}

const COL_STYLE: React.CSSProperties = {
  borderRight: '1px solid var(--c-border)',
  padding: '14px 16px',
  overflowY: 'auto',
  background: 'var(--c-surface)',
};

export function TexturePanel({ settings, dispatch, filaments }: TexturePanelProps) {
  const activeFilaments = filaments.filter((f) => f.active);
  const setColor = (color: string) => dispatch({ type: 'SET_FIELD', key: 'textureColor', value: color });

  return (
    <div style={COL_STYLE}>
      <SectionLabel>Texturschicht</SectionLabel>
      <Slider
        label="Pixelbreite"
        value={settings.texturePixelWidth}
        min={0.1}
        max={1}
        step={0.05}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'texturePixelWidth', value: v })}
      />
      <Slider
        label="Min. Dicke"
        value={settings.textureMin}
        min={0.1}
        max={2}
        step={0.05}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'textureMin', value: v })}
      />
      <Slider
        label="Max. Dicke"
        value={settings.textureMax}
        min={0.5}
        max={5}
        step={0.1}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'textureMax', value: v })}
      />

      <div style={{ marginBottom: 13 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'baseline', marginBottom: 6 }}>
          <div style={{ fontSize: 10, fontWeight: 500, letterSpacing: '0.05em', textTransform: 'uppercase', color: 'var(--c-text-mid)' }}>
            Texturfarbe
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 5 }}>
            <div style={{ width: 13, height: 13, borderRadius: 3, background: settings.textureColor, border: '1px solid var(--c-border)' }} />
            <span style={{ fontFamily: 'DM Mono', fontSize: 9, color: 'var(--c-text-faint)' }}>{settings.textureColor}</span>
          </div>
        </div>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
          {activeFilaments.map((f, i) => (
            <div
              key={`${f.name}-${i}`}
              title={f.name}
              onClick={() => setColor(f.color)}
              style={{
                width: 20,
                height: 20,
                borderRadius: 4,
                background: f.color,
                cursor: 'pointer',
                flexShrink: 0,
                outline: `2px solid ${settings.textureColor === f.color ? 'var(--c-text)' : 'transparent'}`,
                outlineOffset: 1,
                transition: 'outline 0.1s',
              }}
            />
          ))}
          {activeFilaments.length === 0 && (
            <span style={{ fontSize: 9, color: 'var(--c-text-faint)' }}>Palette laden um Farben zu wählen</span>
          )}
        </div>
      </div>

      <div style={{ borderTop: '1px solid var(--c-border)', paddingTop: 12, marginTop: 4 }}>
        <SwitchRow
          label="Farbschicht"
          checked={settings.enableColor}
          onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'enableColor', value: v })}
        />
        <SwitchRow
          label="Texturschicht"
          checked={settings.enableTexture}
          onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'enableTexture', value: v })}
        />
      </div>

      <div style={{ marginTop: 8, padding: '7px 9px', background: 'var(--c-panel2)', borderRadius: 6 }}>
        <span style={{ fontSize: 10, color: 'var(--c-text-mid)', fontFamily: 'DM Mono' }}>
          Gesamt: {settings.plateThickness} + {settings.textureMax} ={' '}
          {(settings.plateThickness + settings.textureMax).toFixed(1)} mm
        </span>
      </div>
    </div>
  );
}

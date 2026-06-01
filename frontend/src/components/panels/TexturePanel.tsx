import type { Dispatch } from 'react';
import { Slider } from '../ui/Slider';
import { SwitchRow } from '../ui/SwitchRow';
import { SectionLabel } from '../ui/SectionLabel';
import { PanelColumn } from './PanelColumn';
import type { Filament, Settings } from '../../lib/types';
import type { SettingsAction } from '../../state/settingsReducer';
import s from './TexturePanel.module.css';

interface TexturePanelProps {
  settings: Settings;
  dispatch: Dispatch<SettingsAction>;
  filaments: Filament[];
}

export function TexturePanel({ settings, dispatch, filaments }: TexturePanelProps) {
  const activeFilaments = filaments.filter((f) => f.active);
  const setColor = (color: string) => dispatch({ type: 'SET_FIELD', key: 'textureColor', value: color });

  return (
    <PanelColumn>
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

      <div className={s.colorSection}>
        <div className={s.head}>
          <div className={s.label}>
            Texturfarbe
          </div>
          <div className={s.current}>
            <div className={s.currentSwatch} style={{ background: settings.textureColor }} />
            <span className={s.currentHex}>{settings.textureColor}</span>
          </div>
        </div>
        <div className={s.swatches}>
          {activeFilaments.map((f, i) => (
            <div
              key={`${f.name}-${i}`}
              title={f.name}
              onClick={() => setColor(f.color)}
              data-selected={settings.textureColor === f.color}
              className={s.swatch}
              style={{ background: f.color }}
            />
          ))}
          {activeFilaments.length === 0 && (
            <span className={s.emptyHint}>Palette laden um Farben zu wählen</span>
          )}
        </div>
      </div>

      <div className={s.toggles}>
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

      <div className={s.summary}>
        <span className={s.summaryText}>
          Gesamt: {settings.plateThickness} + {settings.textureMax} ={' '}
          {(settings.plateThickness + settings.textureMax).toFixed(1)} mm
        </span>
      </div>
    </PanelColumn>
  );
}

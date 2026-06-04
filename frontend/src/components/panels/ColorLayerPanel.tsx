import type { Dispatch } from 'react';
import { Slider } from '../ui/Slider';
import { ToggleGroup } from '../ui/ToggleGroup';
import { SectionLabel } from '../ui/SectionLabel';
import { StatGrid } from '../ui/StatGrid';
import { PanelColumn } from './PanelColumn';
import type { ColorMatching, LithoStats, PixelMethod, Settings } from '../../lib/types';
import type { SettingsAction } from '../../state/settingsReducer';

interface ColorLayerPanelProps {
  settings: Settings;
  dispatch: Dispatch<SettingsAction>;
  stats: LithoStats | null;
}

export function ColorLayerPanel({ settings, dispatch, stats }: ColorLayerPanelProps) {
  return (
    <PanelColumn>
      <SectionLabel>Farbschicht</SectionLabel>
      <Slider
        label="Pixelbreite"
        value={settings.colorPixelWidth}
        min={0.2}
        max={2}
        step={0.05}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'colorPixelWidth', value: v })}
      />
      <Slider
        label="Schichtdicke"
        value={settings.colorLayerThickness}
        min={0.05}
        max={0.4}
        step={0.05}
        unit="mm"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'colorLayerThickness', value: v })}
      />
      <Slider
        label="Schichtanzahl"
        value={settings.colorLayers}
        min={1}
        max={10}
        step={1}
        unit=""
        hint="gestapelte Filamentschichten pro Pixel"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'colorLayers', value: v })}
      />
      <ToggleGroup<PixelMethod>
        label="Pixelmethode"
        options={[['additive', 'Additiv'], ['full', 'Voll']]}
        value={settings.pixelMethod}
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'pixelMethod', value: v })}
      />
      <ToggleGroup<ColorMatching>
        label="Farbabgleich"
        options={[['cie-lab', 'CIE-Lab'], ['rgb', 'RGB']]}
        value={settings.colorMatching}
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'colorMatching', value: v })}
      />
      <Slider
        label="AMS Farben"
        value={settings.amsColors}
        min={0}
        max={4}
        step={1}
        unit=""
        hint="0 = unbegrenzt"
        onChange={(v) => dispatch({ type: 'SET_FIELD', key: 'amsColors', value: v })}
      />
      {stats && (
        <StatGrid
          items={[
            ['Pixel', `${stats.blocksX}×${stats.blocksY}`],
            ['Gesamt', `${(stats.blocksX * stats.blocksY).toLocaleString()}`],
            ['Farbschichten', String(settings.colorLayers)],
            ['Druckschichten', String(settings.colorLayers * stats.blocksX)],
          ]}
        />
      )}
    </PanelColumn>
  );
}

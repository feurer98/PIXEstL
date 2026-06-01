import { useCallback, useReducer, useRef, useState } from 'react';
import { TopBar } from '../components/layout/TopBar';
import { ThemePanel } from '../components/layout/ThemePanel';
import { PreviewPane } from '../components/preview/PreviewPane';
import { SourcePreview } from '../components/preview/SourcePreview';
import { LithoPreview } from '../components/preview/LithoPreview';
import { DimensionsPanel } from '../components/panels/DimensionsPanel';
import { ColorLayerPanel } from '../components/panels/ColorLayerPanel';
import { TexturePanel } from '../components/panels/TexturePanel';
import { PalettePanel } from '../components/panels/PalettePanel';
import { settingsReducer } from '../state/settingsReducer';
import { DEFAULT_SETTINGS } from '../lib/constants';
import { useImageLoader } from '../hooks/useImageLoader';
import { usePaletteLoader } from '../hooks/usePaletteLoader';
import { useLithophaneCanvas } from '../hooks/useLithophaneCanvas';
import { useExport } from '../hooks/useExport';
import type { PreviewMode } from '../lib/types';
import s from './ConverterPage.module.css';

export function ConverterPage() {
  const [settings, dispatch] = useReducer(settingsReducer, DEFAULT_SETTINGS);
  const [previewMode, setPreviewMode] = useState<PreviewMode>('color');
  const [showGrid, setShowGrid] = useState(false);
  const [lockAspect, setLockAspect] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  const lithoRef = useRef<HTMLCanvasElement>(null);

  const onAspect = useCallback(
    (aspect: number) => dispatch({ type: 'APPLY_IMAGE_ASPECT', aspect }),
    [],
  );
  const { image, imgElRef, loadFile } = useImageLoader(onAspect);

  const onCalibration = useCallback(
    (cal: { plateThickness: number | null; firstColor: string | null }) =>
      dispatch({ type: 'APPLY_CALIBRATION', plateThickness: cal.plateThickness, firstColor: cal.firstColor }),
    [],
  );
  const { filaments, toggleFilament, paletteName, calibPlateThickness, error, loadPalette } =
    usePaletteLoader(onCalibration);

  const stats = useLithophaneCanvas(
    lithoRef,
    imgElRef,
    settings,
    { mode: previewMode, showGrid, filaments },
    image,
  );

  const { exportState, exportFormat, setExportFormat, runExport } = useExport();
  const activeCount = filaments.filter((f) => f.active).length;

  return (
    <div className={s.page}>
      <TopBar image={image} paletteName={paletteName} activeCount={activeCount} stats={stats} />

      <PreviewPane>
        <SourcePreview image={image} imgElRef={imgElRef} onFile={loadFile} imageKey={image} />
        <LithoPreview
          hasImage={!!image}
          canvasRef={lithoRef}
          previewMode={previewMode}
          onModeChange={setPreviewMode}
        />
      </PreviewPane>

      <div className={s.panels}>
        <DimensionsPanel
          settings={settings}
          dispatch={dispatch}
          lockAspect={lockAspect}
          onToggleLock={() => setLockAspect((l) => !l)}
          calibPlateThickness={calibPlateThickness}
        />
        <ColorLayerPanel settings={settings} dispatch={dispatch} stats={stats} />
        <TexturePanel settings={settings} dispatch={dispatch} filaments={filaments} />
        <PalettePanel
          paletteName={paletteName}
          filaments={filaments}
          paletteError={error}
          onLoadPalette={loadPalette}
          onToggleFilament={toggleFilament}
          stats={stats}
          previewMode={previewMode}
          hasImage={!!image}
          exportFormat={exportFormat}
          onFormatChange={setExportFormat}
          exportState={exportState}
          onExport={() => runExport({ settings, filaments, format: exportFormat, image })}
        />
      </div>

      {/* Settings drawer toggle (replaces the prototype's postMessage edit-mode). */}
      <button aria-label="Anpassungen" onClick={() => setShowSettings((v) => !v)} className={s.settingsBtn}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="2.2" stroke="currentColor" strokeWidth="1.3" />
          <path
            d="M8 1.5v1.6M8 12.9v1.6M14.5 8h-1.6M3.1 8H1.5M12.6 3.4l-1.1 1.1M4.5 11.5l-1.1 1.1M12.6 12.6l-1.1-1.1M4.5 4.5L3.4 3.4"
            stroke="currentColor"
            strokeWidth="1.3"
            strokeLinecap="round"
          />
        </svg>
      </button>

      <ThemePanel open={showSettings} showGrid={showGrid} onShowGridChange={setShowGrid} />
    </div>
  );
}

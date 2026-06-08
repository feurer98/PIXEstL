import { useCallback, useEffect, useReducer, useRef, useState } from 'react';
import { TopBar } from '../components/layout/TopBar';
import { ThemePanel } from '../components/layout/ThemePanel';
import { PreviewPane } from '../components/preview/PreviewPane';
import { SourcePreview } from '../components/preview/SourcePreview';
import { LithoPreview } from '../components/preview/LithoPreview';
import { DimensionsPanel } from '../components/panels/DimensionsPanel';
import { ColorLayerPanel } from '../components/panels/ColorLayerPanel';
import { TexturePanel } from '../components/panels/TexturePanel';
import { PalettePanel } from '../components/panels/PalettePanel';
import { FilamentManager } from '../components/panels/FilamentManager';
import { settingsReducer } from '../state/settingsReducer';
import { DEFAULT_SETTINGS } from '../lib/constants';
import type { Settings } from '../lib/types';

const SETTINGS_STORAGE_KEY = 'pixestl-settings';

function loadStoredSettings(): Settings {
  try {
    const raw = localStorage.getItem(SETTINGS_STORAGE_KEY);
    if (raw) return { ...DEFAULT_SETTINGS, ...JSON.parse(raw) };
  } catch { /* ignore */ }
  return DEFAULT_SETTINGS;
}
import { useImageLoader } from '../hooks/useImageLoader';
import { usePaletteLoader } from '../hooks/usePaletteLoader';
import { useLithophaneCanvas } from '../hooks/useLithophaneCanvas';
import { useExport } from '../hooks/useExport';
import type { PreviewMode } from '../lib/types';
import s from './ConverterPage.module.css';

export function ConverterPage() {
  const [settings, dispatch] = useReducer(settingsReducer, undefined, loadStoredSettings);
  const [previewMode, setPreviewMode] = useState<PreviewMode>('color');
  const [showGrid, setShowGrid] = useState(false);
  const [lockAspect, setLockAspect] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [focusMode, setFocusMode] = useState(false);
  const [filamentManagerOpen, setFilamentManagerOpen] = useState(false);

  // Escape key exits focus mode
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && focusMode) setFocusMode(false);
    };
    document.addEventListener('keydown', handler);
    return () => document.removeEventListener('keydown', handler);
  }, [focusMode]);

  const lithoRef = useRef<HTMLCanvasElement>(null);

  const onAspect = useCallback(
    (aspect: number) => dispatch({ type: 'APPLY_IMAGE_ASPECT', aspect }),
    [],
  );
  const { image, imageFile, previewUrl, imgElRef, loadFile } = useImageLoader(onAspect);

  const onCalibration = useCallback(
    (cal: { plateThickness: number | null; firstColor: string | null }) =>
      dispatch({ type: 'APPLY_CALIBRATION', plateThickness: cal.plateThickness, firstColor: cal.firstColor }),
    [],
  );
  const {
    filaments,
    setFilaments,
    toggleFilament,
    paletteName,
    paletteFile,
    calibPlateThickness,
    error,
    loadPalette,
  } = usePaletteLoader(onCalibration);

  const stats = useLithophaneCanvas(
    lithoRef,
    imgElRef,
    settings,
    { mode: previewMode, showGrid, filaments },
    image,
  );

  // F-003: persist settings to localStorage on change
  useEffect(() => {
    try { localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(settings)); } catch { /* ignore */ }
  }, [settings]);

  const { exportState, exportFormat, setExportFormat, exportError, runExport } = useExport();
  const activeCount = filaments.filter((f) => f.active).length;

  return (
    <div className={s.page}>
      <TopBar image={image} paletteName={paletteName} activeCount={activeCount} stats={stats} />

      <PreviewPane focusMode={focusMode}>
        <SourcePreview image={image} previewUrl={previewUrl} onFile={loadFile} />
        <LithoPreview
          hasImage={!!image}
          canvasRef={lithoRef}
          previewMode={previewMode}
          onModeChange={setPreviewMode}
          focusMode={focusMode}
          onFocusToggle={() => setFocusMode((v) => !v)}
        />
      </PreviewPane>

      <div className={focusMode ? `${s.panelsRegion} ${s.panelsRegionHidden}` : s.panelsRegion}>
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
            onManage={() => setFilamentManagerOpen(true)}
            stats={stats}
            previewMode={previewMode}
            hasImage={!!image}
            exportFormat={exportFormat}
            onFormatChange={setExportFormat}
            exportState={exportState}
            exportError={exportError}
            onExport={() =>
              runExport({
                settings,
                format: exportFormat,
                imageFile,
                paletteFile,
                activeColors: filaments.filter((f) => f.active).map((f) => f.color),
              })
            }
          />
        </div>

        <FilamentManager
          open={filamentManagerOpen}
          onClose={() => setFilamentManagerOpen(false)}
          filaments={filaments}
          onToggleIndex={toggleFilament}
          onSetFilaments={setFilaments}
        />
      </div>

      {/* F-010: visually hidden h1 for document structure / screen readers */}
      <h1 style={{ position: 'absolute', width: 1, height: 1, overflow: 'hidden', clip: 'rect(0,0,0,0)', whiteSpace: 'nowrap' }}>
        PIXEstL — Farb-Lithophan Konverter
      </h1>

      {/* Settings drawer toggle */}
      <button aria-label="Design & Anpassungen" title="Design & Anpassungen" onClick={() => setShowSettings((v) => !v)} className={s.settingsBtn}>
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

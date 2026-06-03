import { useCallback, useState } from 'react';
import { parsePalette } from '../lib/palette';
import { DEFAULT_FILAMENTS, DEFAULT_PALETTE_NAME } from '../lib/constants';
import type { Filament } from '../lib/types';

interface Calibration {
  plateThickness: number | null;
  firstColor: string | null;
}

/**
 * Owns the active filament palette plus its display name and calibration.
 * `onCalibration` lets the caller push calibration values into settings when a
 * palette carries them.
 */
export function usePaletteLoader(onCalibration?: (cal: Calibration) => void) {
  const [filaments, setFilaments] = useState<Filament[]>(DEFAULT_FILAMENTS);
  const [paletteName, setPaletteName] = useState(DEFAULT_PALETTE_NAME);
  const [calibPlateThickness, setCalibPlateThickness] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  // The original file is kept so the backend export can use the full palette
  // (incl. per-layer calibration the parser discards — see V-MODEL-01).
  const [paletteFile, setPaletteFile] = useState<File | null>(null);

  const loadPalette = useCallback(
    (file: File | null | undefined) => {
      if (!file) return;
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const { filaments: fs, plateThickness } = parsePalette(e.target?.result as string);
          setFilaments(fs);
          setPaletteName(file.name);
          setError(null);
          setPaletteFile(file);
          if (plateThickness !== null) setCalibPlateThickness(plateThickness);
          else setCalibPlateThickness(null);
          onCalibration?.({
            plateThickness,
            firstColor: fs[0]?.color ?? null,
          });
        } catch (err) {
          setError(err instanceof Error ? err.message : 'Palette konnte nicht gelesen werden');
        }
      };
      reader.readAsText(file);
    },
    [onCalibration],
  );

  /** Toggle a single filament's active flag by index. */
  const toggleFilament = useCallback((index: number) => {
    setFilaments((fs) => fs.map((f, i) => (i === index ? { ...f, active: !f.active } : f)));
  }, []);

  return {
    filaments,
    setFilaments,
    toggleFilament,
    paletteName,
    paletteFile,
    calibPlateThickness,
    error,
    loadPalette,
  };
}

import { useCallback, useState } from 'react';
import { parsePalette } from '../lib/palette';
import { DEFAULT_PALETTE } from '../lib/defaultPalette';
import type { Filament } from '../lib/types';

interface Calibration {
  plateThickness: number | null;
  firstColor: string | null;
}

/**
 * Owns the active filament palette plus its display name and calibration.
 * Starts from the bundled default palette (real, calibrated) so the app is
 * usable — including export — without an upload. `onCalibration` lets the caller
 * push calibration values into settings when a palette carries them.
 */
export function usePaletteLoader(onCalibration?: (cal: Calibration) => void) {
  const [filaments, setFilaments] = useState<Filament[]>(DEFAULT_PALETTE.filaments);
  const [paletteName, setPaletteName] = useState(DEFAULT_PALETTE.name);
  const [calibPlateThickness, setCalibPlateThickness] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  // Holds the exact palette bytes for the backend export. Seeded with the
  // bundled default so export works out of the box (V-MODEL-01).
  const [paletteFile, setPaletteFile] = useState<File | null>(DEFAULT_PALETTE.toFile());

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

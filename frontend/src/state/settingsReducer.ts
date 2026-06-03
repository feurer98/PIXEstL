import type { Settings } from '../lib/types';

// ─── Settings reducer ────────────────────────────────────────────────────────
// The prototype mutated `settings` through ad-hoc setSetting closures, which
// hid the coupled updates (aspect-lock changing the other dimension; a loaded
// palette pushing plate thickness + texture color). Modeling those as explicit
// actions keeps the coupling testable.

export type SettingsAction =
  | { type: 'SET_FIELD'; key: keyof Settings; value: Settings[keyof Settings] }
  | { type: 'SET_WIDTH'; value: number; lockAspect: boolean }
  | { type: 'SET_HEIGHT'; value: number; lockAspect: boolean }
  | { type: 'APPLY_IMAGE_ASPECT'; aspect: number } // height = round(width * aspect)
  | { type: 'APPLY_CALIBRATION'; plateThickness: number | null; firstColor: string | null };

export function settingsReducer(state: Settings, action: SettingsAction): Settings {
  switch (action.type) {
    case 'SET_FIELD':
      return { ...state, [action.key]: action.value };

    case 'SET_WIDTH': {
      const next: Settings = { ...state, width: action.value };
      if (action.lockAspect && state.width > 0) {
        next.height = Math.round(action.value * (state.height / state.width));
      }
      return next;
    }

    case 'SET_HEIGHT': {
      const next: Settings = { ...state, height: action.value };
      if (action.lockAspect && state.height > 0) {
        next.width = Math.round(action.value * (state.width / state.height));
      }
      return next;
    }

    case 'APPLY_IMAGE_ASPECT':
      // Called on image load: keep width, derive height from image aspect ratio.
      return { ...state, height: Math.round(state.width * action.aspect) };

    case 'APPLY_CALIBRATION': {
      const next = { ...state };
      if (action.plateThickness !== null) next.plateThickness = action.plateThickness;
      if (action.firstColor !== null) next.textureColor = action.firstColor;
      return next;
    }

    default:
      return state;
  }
}

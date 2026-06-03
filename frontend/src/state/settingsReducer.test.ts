import { describe, expect, it } from 'vitest';
import { settingsReducer } from './settingsReducer';
import { DEFAULT_SETTINGS } from '../lib/constants';

describe('settingsReducer', () => {
  it('SET_FIELD updates a single field', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, { type: 'SET_FIELD', key: 'curve', value: 90 });
    expect(next.curve).toBe(90);
    expect(next.width).toBe(DEFAULT_SETTINGS.width);
  });

  it('SET_WIDTH without lock leaves height untouched', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, { type: 'SET_WIDTH', value: 200, lockAspect: false });
    expect(next.width).toBe(200);
    expect(next.height).toBe(DEFAULT_SETTINGS.height);
  });

  it('SET_WIDTH with lock scales height proportionally', () => {
    // start 100x80 → width 200 keeps 0.8 ratio → height 160
    const next = settingsReducer(DEFAULT_SETTINGS, { type: 'SET_WIDTH', value: 200, lockAspect: true });
    expect(next.height).toBe(160);
  });

  it('SET_HEIGHT with lock scales width proportionally', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, { type: 'SET_HEIGHT', value: 160, lockAspect: true });
    expect(next.width).toBe(200);
  });

  it('APPLY_IMAGE_ASPECT derives height from width', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, { type: 'APPLY_IMAGE_ASPECT', aspect: 1.5 });
    expect(next.height).toBe(150);
  });

  it('APPLY_CALIBRATION sets plate thickness and texture color when present', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, {
      type: 'APPLY_CALIBRATION',
      plateThickness: 0.24,
      firstColor: '#0086D6',
    });
    expect(next.plateThickness).toBe(0.24);
    expect(next.textureColor).toBe('#0086D6');
  });

  it('APPLY_CALIBRATION ignores null fields', () => {
    const next = settingsReducer(DEFAULT_SETTINGS, {
      type: 'APPLY_CALIBRATION',
      plateThickness: null,
      firstColor: null,
    });
    expect(next.plateThickness).toBe(DEFAULT_SETTINGS.plateThickness);
    expect(next.textureColor).toBe(DEFAULT_SETTINGS.textureColor);
  });
});

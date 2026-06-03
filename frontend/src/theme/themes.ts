// ─── Themes ──────────────────────────────────────────────────────────────────
// Single source of truth, ported from the prototype's THEMES object. Each theme
// is projected onto CSS custom properties at runtime by ThemeProvider, so
// components reference `var(--c-*)` instead of receiving a `T` prop everywhere.

export interface Theme {
  name: string;
  bg: string;
  surface: string;
  panel: string;
  panel2: string;
  border: string;
  borderStrong: string;
  text: string;
  textMid: string;
  textFaint: string;
  accent: string;
  accentRgb: string;
  accentLight: string;
  accentText: string;
  preview2bg: string;
  sliderAccent: string;
  tag: string;
  tagText: string;
  toggleOn: string;
  ok: string;
  okText: string;
}

export type ThemeId = 'studio' | 'blueprint' | 'dark';

export const THEMES: Record<ThemeId, Theme> = {
  studio: {
    name: 'Studio', bg: '#faf9f7', surface: '#ffffff', panel: '#f4f3f0',
    panel2: '#edecea', border: '#e8e6e1', borderStrong: '#d0cdc6',
    text: '#1a1916', textMid: '#6b6860', textFaint: '#a8a49c',
    accent: 'oklch(0.62 0.14 62)', accentRgb: '#c96c1a', accentLight: 'oklch(0.96 0.04 65)',
    accentText: '#914e0a', preview2bg: '#110a02', sliderAccent: 'oklch(0.62 0.14 62)',
    tag: '#f0ebe0', tagText: '#7a5520', toggleOn: 'oklch(0.62 0.14 62)',
    ok: 'oklch(0.55 0.15 142)', okText: '#fff',
  },
  blueprint: {
    name: 'Entwurf', bg: '#f0f3f8', surface: '#ffffff', panel: '#e8edf5',
    panel2: '#dde4f0', border: '#c8d4e8', borderStrong: '#a0b4d0',
    text: '#0d1a2e', textMid: '#4a6080', textFaint: '#8aa0bc',
    accent: 'oklch(0.50 0.18 240)', accentRgb: '#1040a0', accentLight: 'oklch(0.94 0.06 240)',
    accentText: '#1040a0', preview2bg: '#050d1a', sliderAccent: 'oklch(0.50 0.18 240)',
    tag: '#dce8f8', tagText: '#1040a0', toggleOn: 'oklch(0.50 0.18 240)',
    ok: 'oklch(0.55 0.15 142)', okText: '#fff',
  },
  dark: {
    name: 'Dunkel', bg: '#111110', surface: '#1c1c1a', panel: '#252523',
    panel2: '#2e2e2b', border: '#2e2e2b', borderStrong: '#3d3d39',
    text: '#e8e6e0', textMid: '#8a8880', textFaint: '#504e48',
    accent: 'oklch(0.72 0.15 62)', accentRgb: '#e07830', accentLight: 'oklch(0.18 0.06 62)',
    accentText: 'oklch(0.82 0.15 62)', preview2bg: '#000000', sliderAccent: 'oklch(0.72 0.15 62)',
    tag: '#2a2010', tagText: 'oklch(0.75 0.14 65)', toggleOn: 'oklch(0.72 0.15 62)',
    ok: 'oklch(0.55 0.15 142)', okText: '#fff',
  },
};

/** Maps a Theme's fields onto the CSS custom properties consumed by components. */
export function themeToCssVars(theme: Theme): Record<string, string> {
  return {
    '--c-bg': theme.bg,
    '--c-surface': theme.surface,
    '--c-panel': theme.panel,
    '--c-panel2': theme.panel2,
    '--c-border': theme.border,
    '--c-border-strong': theme.borderStrong,
    '--c-text': theme.text,
    '--c-text-mid': theme.textMid,
    '--c-text-faint': theme.textFaint,
    '--c-accent': theme.accent,
    '--c-accent-rgb': theme.accentRgb,
    '--c-accent-light': theme.accentLight,
    '--c-accent-text': theme.accentText,
    '--c-preview2-bg': theme.preview2bg,
    '--c-slider-accent': theme.sliderAccent,
    '--c-tag': theme.tag,
    '--c-tag-text': theme.tagText,
    '--c-toggle-on': theme.toggleOn,
    '--c-ok': theme.ok,
    '--c-ok-text': theme.okText,
  };
}

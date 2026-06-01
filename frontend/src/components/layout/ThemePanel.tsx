import { useTheme } from '../../theme/ThemeContext';
import { THEMES, type ThemeId } from '../../theme/themes';
import { SwitchRow } from '../ui/SwitchRow';
import s from './ThemePanel.module.css';

interface ThemePanelProps {
  open: boolean;
  showGrid: boolean;
  onShowGridChange: (value: boolean) => void;
}

/**
 * Settings drawer. Replaces the prototype's postMessage "edit mode" bridge
 * (a bundler-only artifact) with a real in-app panel.
 */
export function ThemePanel({ open, showGrid, onShowGridChange }: ThemePanelProps) {
  const { themeId, setThemeId } = useTheme();
  if (!open) return null;

  return (
    <div className={s.drawer}>
      <div className={s.title}>
        Anpassungen
      </div>
      <div className={s.section}>
        <div className={s.sectionLabel}>
          Design
        </div>
        <div className={s.themes}>
          {(Object.entries(THEMES) as [ThemeId, (typeof THEMES)[ThemeId]][]).map(([k, t]) => {
            const selected = themeId === k;
            return (
              <button
                key={k}
                onClick={() => setThemeId(k)}
                data-selected={selected}
                className={s.themeBtn}
                // Each button previews ANOTHER theme's surface color, so this
                // intentionally stays inline (not var(--c-surface)).
                style={{ background: t.surface }}
              >
                {t.name}
              </button>
            );
          })}
        </div>
      </div>
      <SwitchRow label="Pixelraster" checked={showGrid} onChange={onShowGridChange} />
    </div>
  );
}

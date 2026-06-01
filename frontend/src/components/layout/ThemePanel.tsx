import { useTheme } from '../../theme/ThemeContext';
import { THEMES, type ThemeId } from '../../theme/themes';
import { SwitchRow } from '../ui/SwitchRow';

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
    <div
      style={{
        position: 'fixed',
        bottom: 20,
        right: 20,
        width: 210,
        background: 'var(--c-surface)',
        border: '1px solid var(--c-border-strong)',
        borderRadius: 12,
        padding: 16,
        boxShadow: '0 8px 32px rgba(0,0,0,0.18)',
        zIndex: 1000,
      }}
    >
      <div
        style={{
          fontSize: 11,
          fontWeight: 700,
          letterSpacing: '0.08em',
          textTransform: 'uppercase',
          color: 'var(--c-text-mid)',
          marginBottom: 14,
        }}
      >
        Anpassungen
      </div>
      <div style={{ marginBottom: 12 }}>
        <div
          style={{
            fontSize: 9,
            color: 'var(--c-text-faint)',
            marginBottom: 7,
            textTransform: 'uppercase',
            letterSpacing: '0.08em',
          }}
        >
          Design
        </div>
        <div style={{ display: 'flex', gap: 5 }}>
          {(Object.entries(THEMES) as [ThemeId, (typeof THEMES)[ThemeId]][]).map(([k, t]) => {
            const selected = themeId === k;
            return (
              <button
                key={k}
                onClick={() => setThemeId(k)}
                style={{
                  flex: 1,
                  padding: '6px 4px',
                  border: `1.5px solid ${selected ? 'var(--c-accent)' : 'var(--c-border)'}`,
                  borderRadius: 6,
                  background: t.surface,
                  cursor: 'pointer',
                  fontSize: 9,
                  fontWeight: selected ? 700 : 400,
                  color: selected ? 'var(--c-accent-text)' : 'var(--c-text-mid)',
                }}
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

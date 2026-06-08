import { createContext, useContext, useMemo, useState, useCallback, type ReactNode } from 'react';
import { THEMES, themeToCssVars, type Theme, type ThemeId } from './themes';
import { supportsOklch, withOklchFallback } from '../lib/oklch';

interface ThemeContextValue {
  themeId: ThemeId;
  theme: Theme;
  setThemeId: (id: ThemeId) => void;
}

const THEME_STORAGE_KEY = 'pixestl-theme';

function getInitialTheme(): ThemeId {
  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY) as ThemeId | null;
    if (stored && stored in THEMES) return stored;
  } catch {
    // localStorage unavailable
  }
  if (window.matchMedia('(prefers-color-scheme: dark)').matches) return 'dark';
  return 'studio';
}

const ThemeContext = createContext<ThemeContextValue | null>(null);

export function ThemeProvider({
  children,
  initial,
}: {
  children: ReactNode;
  initial?: ThemeId;
}) {
  const [themeId, setThemeIdRaw] = useState<ThemeId>(() => initial ?? getInitialTheme());

  const setThemeId = useCallback((id: ThemeId) => {
    setThemeIdRaw(id);
    try { localStorage.setItem(THEME_STORAGE_KEY, id); } catch { /* ignore */ }
  }, []);
  const theme = THEMES[themeId];

  // Detect oklch() support once; older browsers get computed hex fallbacks.
  const oklchOk = useMemo(() => supportsOklch(), []);

  const value = useMemo<ThemeContextValue>(
    () => ({ themeId, theme, setThemeId }),
    [themeId, theme],
  );

  // Project the active theme onto CSS custom properties on the wrapper element.
  const cssVars = useMemo(() => {
    const vars = themeToCssVars(theme);
    return (oklchOk ? vars : withOklchFallback(vars)) as React.CSSProperties;
  }, [theme, oklchOk]);

  return (
    <ThemeContext.Provider value={value}>
      <div
        data-theme={themeId}
        style={{
          ...cssVars,
          height: '100%',
          background: 'var(--c-bg)',
          color: 'var(--c-text)',
          transition: 'background 0.3s, color 0.3s',
        }}
      >
        {children}
      </div>
    </ThemeContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useTheme(): ThemeContextValue {
  const ctx = useContext(ThemeContext);
  if (!ctx) throw new Error('useTheme must be used within a ThemeProvider');
  return ctx;
}

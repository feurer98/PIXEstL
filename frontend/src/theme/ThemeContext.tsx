import { createContext, useContext, useMemo, useState, type ReactNode } from 'react';
import { THEMES, themeToCssVars, type Theme, type ThemeId } from './themes';

interface ThemeContextValue {
  themeId: ThemeId;
  theme: Theme;
  setThemeId: (id: ThemeId) => void;
}

const ThemeContext = createContext<ThemeContextValue | null>(null);

export function ThemeProvider({
  children,
  initial = 'studio',
}: {
  children: ReactNode;
  initial?: ThemeId;
}) {
  const [themeId, setThemeId] = useState<ThemeId>(initial);
  const theme = THEMES[themeId];

  const value = useMemo<ThemeContextValue>(
    () => ({ themeId, theme, setThemeId }),
    [themeId, theme],
  );

  // Project the active theme onto CSS custom properties on the wrapper element.
  const cssVars = themeToCssVars(theme) as React.CSSProperties;

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

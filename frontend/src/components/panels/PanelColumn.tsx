import type { ReactNode } from 'react';

/**
 * Shared shell for the three left settings columns (Dimensions, Color, Texture).
 * Replaces the COL_STYLE constant that was duplicated across those panels.
 * The fourth column (PalettePanel) has a distinct layout and does not use this.
 */
export function PanelColumn({ children }: { children: ReactNode }) {
  return (
    <div
      style={{
        borderRight: '1px solid var(--c-border)',
        padding: '14px 16px',
        overflowY: 'auto',
        background: 'var(--c-surface)',
      }}
    >
      {children}
    </div>
  );
}

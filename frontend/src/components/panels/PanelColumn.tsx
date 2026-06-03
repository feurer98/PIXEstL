import type { ReactNode } from 'react';
import s from './PanelColumn.module.css';

/**
 * Shared shell for the three left settings columns (Dimensions, Color, Texture).
 * Replaces the COL_STYLE constant that was duplicated across those panels.
 * The fourth column (PalettePanel) has a distinct layout and does not use this.
 */
export function PanelColumn({ children }: { children: ReactNode }) {
  return <div className={s.column}>{children}</div>;
}

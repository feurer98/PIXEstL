import { type ReactNode } from 'react';
import s from './PreviewPane.module.css';

interface PreviewPaneProps {
  children: ReactNode;
  focusMode: boolean;
}

/** Two-up preview grid (source | lithophane). Expands to fill the screen in focus mode. */
export function PreviewPane({ children, focusMode }: PreviewPaneProps) {
  return (
    <div className={focusMode ? `${s.grid} ${s.gridFocused}` : s.grid}>
      {children}
    </div>
  );
}

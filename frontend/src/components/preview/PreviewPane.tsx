import { type ReactNode } from 'react';
import s from './PreviewPane.module.css';

/** Two-up preview grid (source | lithophane), fixed at ~38% of viewport height. */
export function PreviewPane({ children }: { children: ReactNode }) {
  return <div className={s.grid}>{children}</div>;
}

import type { ReactNode } from 'react';
import s from './SectionLabel.module.css';

export function SectionLabel({ children }: { children: ReactNode }) {
  return <div className={s.label}>{children}</div>;
}

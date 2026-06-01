import { type ReactNode } from 'react';

/** Two-up preview grid (source | lithophane), fixed at ~38% of viewport height. */
export function PreviewPane({ children }: { children: ReactNode }) {
  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr 1fr',
        flex: '0 0 38%',
        borderBottom: '1px solid var(--c-border)',
        minHeight: 0,
      }}
    >
      {children}
    </div>
  );
}

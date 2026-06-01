import type { ReactNode } from 'react';

export function SectionLabel({ children }: { children: ReactNode }) {
  return (
    <div
      style={{
        fontSize: 9,
        fontWeight: 700,
        letterSpacing: '0.12em',
        textTransform: 'uppercase',
        color: 'var(--c-text-faint)',
        marginBottom: 12,
        paddingBottom: 6,
        borderBottom: '1px solid var(--c-border)',
      }}
    >
      {children}
    </div>
  );
}

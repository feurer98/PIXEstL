interface StatGridProps {
  items: [key: string, value: string][];
}

export function StatGrid({ items }: StatGridProps) {
  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr 1fr',
        gap: '6px 4px',
        padding: '8px 10px',
        background: 'var(--c-panel2)',
        borderRadius: 7,
        marginTop: 10,
      }}
    >
      {items.map(([k, v]) => (
        <div key={k}>
          <div
            style={{
              fontSize: 8,
              textTransform: 'uppercase',
              letterSpacing: '0.08em',
              color: 'var(--c-text-faint)',
              marginBottom: 1,
            }}
          >
            {k}
          </div>
          <div style={{ fontFamily: 'DM Mono', fontSize: 11, fontWeight: 500, color: 'var(--c-text)' }}>
            {v}
          </div>
        </div>
      ))}
    </div>
  );
}

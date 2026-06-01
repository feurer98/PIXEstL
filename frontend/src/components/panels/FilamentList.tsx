import type { Filament } from '../../lib/types';

interface FilamentListProps {
  filaments: Filament[];
  onToggle: (index: number) => void;
}

export function FilamentList({ filaments, onToggle }: FilamentListProps) {
  return (
    <div style={{ flex: 1, overflowY: 'auto', minHeight: 0, maxHeight: 240, marginBottom: 10 }}>
      {filaments.map((f, i) => (
        <div
          key={`${f.name}-${i}`}
          onClick={() => onToggle(i)}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 9,
            padding: '6px 8px',
            borderRadius: 6,
            cursor: 'pointer',
            marginBottom: 2,
            background: f.active ? 'var(--c-surface)' : 'transparent',
            opacity: f.active ? 1 : 0.35,
            border: `1px solid ${f.active ? 'var(--c-border)' : 'transparent'}`,
            transition: 'all 0.15s',
          }}
        >
          <div
            style={{
              width: 18,
              height: 18,
              borderRadius: 4,
              background: f.color,
              flexShrink: 0,
              border: '1px solid rgba(0,0,0,0.12)',
              boxShadow: '0 1px 2px rgba(0,0,0,0.1)',
            }}
          />
          <span
            style={{
              fontSize: 12,
              fontWeight: f.active ? 500 : 400,
              color: 'var(--c-text)',
              flex: 1,
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }}
          >
            {f.name}
          </span>
          <span style={{ fontFamily: 'DM Mono', fontSize: 9, color: 'var(--c-text-faint)' }}>{f.color}</span>
        </div>
      ))}
    </div>
  );
}

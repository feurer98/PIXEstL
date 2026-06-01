import type { Filament, LithoStats } from '../../lib/types';

interface UsageBreakdownProps {
  stats: LithoStats;
  filaments: Filament[];
}

/** Top-5 filament usage bars (color mode only). */
export function UsageBreakdown({ stats, filaments }: UsageBreakdownProps) {
  const total = stats.blocksX * stats.blocksY;
  const top = Object.entries(stats.filamentUsage)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 5);

  return (
    <div style={{ marginBottom: 12, padding: '8px 10px', background: 'var(--c-panel2)', borderRadius: 7 }}>
      <div
        style={{
          fontSize: 9,
          fontWeight: 700,
          letterSpacing: '0.1em',
          textTransform: 'uppercase',
          color: 'var(--c-text-faint)',
          marginBottom: 6,
        }}
      >
        Verwendung
      </div>
      {top.map(([name, count]) => {
        const pct = ((count / total) * 100).toFixed(0);
        const f = filaments.find((x) => x.name === name);
        const color = f?.color ?? '#888';
        return (
          <div key={name} style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 3 }}>
            <div style={{ width: 8, height: 8, borderRadius: 1, background: color, flexShrink: 0 }} />
            <span style={{ fontSize: 9, color: 'var(--c-text-mid)', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
              {name}
            </span>
            <span style={{ fontFamily: 'DM Mono', fontSize: 9, color: 'var(--c-text-faint)' }}>{pct}%</span>
            <div style={{ width: 32, height: 3, background: 'var(--c-border)', borderRadius: 2, overflow: 'hidden' }}>
              <div style={{ width: pct + '%', height: '100%', background: color }} />
            </div>
          </div>
        );
      })}
    </div>
  );
}

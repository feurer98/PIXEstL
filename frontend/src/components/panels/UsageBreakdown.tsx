import type { CSSProperties } from 'react';
import type { Filament, LithoStats } from '../../lib/types';
import s from './UsageBreakdown.module.css';

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
    <div className={s.wrap}>
      <div className={s.title}>
        Verwendung
      </div>
      {top.map(([name, count]) => {
        const pct = ((count / total) * 100).toFixed(0);
        const f = filaments.find((x) => x.name === name);
        const color = f?.color ?? '#888';
        return (
          <div key={name} className={s.row}>
            <div className={s.dot} style={{ background: color }} />
            <span className={s.name}>
              {name}
            </span>
            <span className={s.pct}>{pct}%</span>
            <div className={s.bar}>
              <div className={s.barFill} style={{ '--bar-width': pct + '%', background: color } as CSSProperties} />
            </div>
          </div>
        );
      })}
    </div>
  );
}

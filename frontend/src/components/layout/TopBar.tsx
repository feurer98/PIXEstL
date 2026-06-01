import type { ImageInfo, LithoStats } from '../../lib/types';
import s from './TopBar.module.css';

const LOGO_DOTS: [number, number, number][] = [
  [0, 0, 0.9], [3, 0, 0.7], [6, 0, 0.5],
  [0, 3, 0.6], [3, 3, 1], [6, 3, 0.4],
  [0, 6, 0.3], [3, 6, 0.8], [6, 6, 0.6],
];

interface TopBarProps {
  image: ImageInfo | null;
  paletteName: string;
  activeCount: number;
  stats: LithoStats | null;
}

export function TopBar({ image, paletteName, activeCount, stats }: TopBarProps) {
  return (
    <div className={s.bar}>
      <div className={s.left}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          {LOGO_DOTS.map(([x, y, o], i) => (
            <rect key={i} x={x} y={y} width="2.5" height="2.5" rx="0.4" className={s.logo} opacity={o} />
          ))}
        </svg>
        <span className={s.name}>
          PIXEstL
        </span>
        <span className={s.subtitle}>Farb-Lithophan</span>
        {image && (
          <>
            <div className={s.divider} />
            <span className={s.fileTag}>
              {image.name}
            </span>
          </>
        )}
      </div>
      <div className={s.right}>
        <span className={s.meta}>
          {paletteName} · {activeCount} aktiv
        </span>
        {stats && (
          <span className={s.meta}>
            {stats.blocksX}×{stats.blocksY} px
          </span>
        )}
      </div>
    </div>
  );
}

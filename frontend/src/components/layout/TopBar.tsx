import { useTheme } from '../../theme/ThemeContext';
import type { ImageInfo, LithoStats } from '../../lib/types';

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
  const { theme } = useTheme();
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '0 16px',
        height: 46,
        borderBottom: '1px solid var(--c-border)',
        background: 'var(--c-surface)',
        flexShrink: 0,
        gap: 8,
      }}
    >
      <div style={{ display: 'flex', alignItems: 'center', gap: 10, minWidth: 0 }}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          {LOGO_DOTS.map(([x, y, o], i) => (
            <rect key={i} x={x} y={y} width="2.5" height="2.5" rx="0.4" fill={theme.accentRgb} opacity={o} />
          ))}
        </svg>
        <span style={{ fontWeight: 600, fontSize: 12, letterSpacing: '-0.01em', whiteSpace: 'nowrap' }}>
          PIXEstL
        </span>
        <span style={{ color: 'var(--c-text-faint)', fontSize: 11, fontWeight: 300 }}>Farb-Lithophan</span>
        {image && (
          <>
            <div style={{ width: 1, height: 14, background: 'var(--c-border)' }} />
            <span
              style={{
                fontSize: 10,
                fontFamily: 'DM Mono',
                background: 'var(--c-tag)',
                color: 'var(--c-tag-text)',
                padding: '2px 7px',
                borderRadius: 3,
                maxWidth: 200,
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                whiteSpace: 'nowrap',
              }}
            >
              {image.name}
            </span>
          </>
        )}
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
        <span style={{ fontSize: 10, fontFamily: 'DM Mono', color: 'var(--c-text-faint)', whiteSpace: 'nowrap' }}>
          {paletteName} · {activeCount} aktiv
        </span>
        {stats && (
          <span style={{ fontSize: 10, fontFamily: 'DM Mono', color: 'var(--c-text-faint)', whiteSpace: 'nowrap' }}>
            {stats.blocksX}×{stats.blocksY} px
          </span>
        )}
      </div>
    </div>
  );
}

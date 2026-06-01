import s from './StatGrid.module.css';

interface StatGridProps {
  items: [key: string, value: string][];
}

export function StatGrid({ items }: StatGridProps) {
  return (
    <div className={s.grid}>
      {items.map(([k, v]) => (
        <div key={k}>
          <div className={s.key}>{k}</div>
          <div className={s.value}>{v}</div>
        </div>
      ))}
    </div>
  );
}

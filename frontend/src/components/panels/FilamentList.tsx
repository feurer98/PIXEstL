import type { Filament } from '../../lib/types';
import s from './FilamentList.module.css';

interface FilamentListProps {
  filaments: Filament[];
  onToggle: (index: number) => void;
}

export function FilamentList({ filaments, onToggle }: FilamentListProps) {
  return (
    <div className={s.list}>
      {filaments.map((f, i) => (
        <div
          key={`${f.name}-${i}`}
          onClick={() => onToggle(i)}
          data-active={f.active}
          className={s.item}
        >
          <div className={s.swatch} style={{ background: f.color }} />
          <span className={s.name}>
            {f.name}
          </span>
          <span className={s.hex}>{f.color}</span>
        </div>
      ))}
    </div>
  );
}

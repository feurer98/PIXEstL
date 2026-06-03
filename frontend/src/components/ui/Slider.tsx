import type { CSSProperties } from 'react';
import s from './Slider.module.css';

interface SliderProps {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  unit?: string;
  hint?: string;
  onChange: (value: number) => void;
}

export function Slider({ label, value, min, max, step, unit, hint, onChange }: SliderProps) {
  const pct = ((value - min) / (max - min)) * 100;
  const fmt = (v: number) => (step < 0.1 ? v.toFixed(2) : step < 1 ? v.toFixed(1) : String(v));

  return (
    <div className={s.wrap}>
      <div className={s.head}>
        <span className={s.label}>{label}</span>
        <span className={s.value}>
          {fmt(value)}
          <span className={s.unit}>{unit}</span>
        </span>
      </div>
      <div className={s.rail}>
        <div className={s.track}>
          <div className={s.fill} style={{ '--fill': pct + '%' } as CSSProperties} />
        </div>
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange(Number(e.target.value))}
          className={s.input}
        />
      </div>
      {hint && <div className={s.hint}>{hint}</div>}
    </div>
  );
}

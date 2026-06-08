import { useState, type CSSProperties } from 'react';
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
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState('');

  const pct = ((value - min) / (max - min)) * 100;
  const fmt = (v: number) => (step < 0.1 ? v.toFixed(2) : step < 1 ? v.toFixed(1) : String(v));

  function commitDraft() {
    const parsed = parseFloat(draft.replace(',', '.'));
    if (!isNaN(parsed)) {
      onChange(Math.min(max, Math.max(min, parsed)));
    }
    setEditing(false);
  }

  return (
    <div className={s.wrap}>
      <div className={s.head}>
        <span className={s.label}>{label}</span>
        {/* F-013: click the value to edit it directly */}
        {editing ? (
          <input
            type="number"
            className={s.valueInput}
            value={draft}
            min={min}
            max={max}
            step={step}
            autoFocus
            onChange={(e) => setDraft(e.target.value)}
            onBlur={commitDraft}
            onKeyDown={(e) => { if (e.key === 'Enter') commitDraft(); if (e.key === 'Escape') setEditing(false); }}
            aria-label={label}
          />
        ) : (
          <button
            type="button"
            className={s.value}
            title="Klicken zum direkten Eingeben"
            onClick={() => { setDraft(fmt(value)); setEditing(true); }}
            aria-label={`${label}: ${fmt(value)} ${unit ?? ''}, zum Bearbeiten klicken`}
          >
            {fmt(value)}
            <span className={s.unit}>{unit}</span>
          </button>
        )}
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
          aria-label={label}
        />
      </div>
      {hint && <div className={s.hint}>{hint}</div>}
    </div>
  );
}

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
  const fmt = (v: number) =>
    step < 0.1 ? v.toFixed(2) : step < 1 ? v.toFixed(1) : String(v);

  return (
    <div style={{ marginBottom: 13 }}>
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'baseline',
          marginBottom: 5,
        }}
      >
        <span
          style={{
            fontSize: 10,
            fontWeight: 500,
            letterSpacing: '0.05em',
            textTransform: 'uppercase',
            color: 'var(--c-text-mid)',
          }}
        >
          {label}
        </span>
        <span style={{ fontFamily: 'DM Mono', fontSize: 11, color: 'var(--c-text)', fontWeight: 500 }}>
          {fmt(value)}
          <span style={{ color: 'var(--c-text-faint)', marginLeft: 1 }}>{unit}</span>
        </span>
      </div>
      <div style={{ position: 'relative', height: 18, display: 'flex', alignItems: 'center' }}>
        <div
          style={{
            position: 'absolute',
            left: 0,
            right: 0,
            height: 2,
            background: 'var(--c-border)',
            borderRadius: 1,
            overflow: 'hidden',
          }}
        >
          <div
            style={{
              width: pct + '%',
              height: '100%',
              background: 'var(--c-slider-accent)',
              borderRadius: 1,
              transition: 'width 0.04s',
            }}
          />
        </div>
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange(Number(e.target.value))}
          style={{
            position: 'absolute',
            left: 0,
            right: 0,
            width: '100%',
            cursor: 'pointer',
            height: 18,
            margin: 0,
          }}
        />
      </div>
      {hint && <div style={{ fontSize: 9, color: 'var(--c-text-faint)', marginTop: 2 }}>{hint}</div>}
    </div>
  );
}

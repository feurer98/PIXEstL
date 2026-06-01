interface ToggleGroupProps<T extends string> {
  label: string;
  options: [value: T, label: string][];
  value: T;
  onChange: (value: T) => void;
}

export function ToggleGroup<T extends string>({
  label,
  options,
  value,
  onChange,
}: ToggleGroupProps<T>) {
  return (
    <div style={{ marginBottom: 13 }}>
      <div
        style={{
          fontSize: 10,
          fontWeight: 500,
          letterSpacing: '0.05em',
          textTransform: 'uppercase',
          color: 'var(--c-text-mid)',
          marginBottom: 6,
        }}
      >
        {label}
      </div>
      <div style={{ display: 'flex', gap: 3 }}>
        {options.map(([v, lbl]) => {
          const selected = value === v;
          return (
            <button
              key={v}
              onClick={() => onChange(v)}
              style={{
                flex: 1,
                padding: '5px 0',
                border: `1px solid ${selected ? 'var(--c-border-strong)' : 'transparent'}`,
                borderRadius: 5,
                cursor: 'pointer',
                fontSize: 10,
                fontWeight: selected ? 600 : 400,
                background: selected ? 'var(--c-surface)' : 'var(--c-panel2)',
                color: selected ? 'var(--c-text)' : 'var(--c-text-mid)',
                transition: 'all 0.15s',
                boxShadow: selected ? '0 1px 3px rgba(0,0,0,0.08)' : 'none',
              }}
            >
              {lbl}
            </button>
          );
        })}
      </div>
    </div>
  );
}

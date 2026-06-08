import s from './ToggleGroup.module.css';

interface ToggleGroupProps<T extends string> {
  label: string;
  tooltip?: string;
  options: [value: T, label: string][];
  value: T;
  onChange: (value: T) => void;
}

export function ToggleGroup<T extends string>({
  label,
  tooltip,
  options,
  value,
  onChange,
}: ToggleGroupProps<T>) {
  return (
    <div className={s.group}>
      <div className={s.label} title={tooltip}>{label}{tooltip && <span style={{ marginLeft: 3, opacity: 0.5, cursor: 'help' }}>?</span>}</div>
      <div className={s.options}>
        {options.map(([v, lbl]) => (
          <button key={v} onClick={() => onChange(v)} data-selected={value === v} className={s.btn}>
            {lbl}
          </button>
        ))}
      </div>
    </div>
  );
}

import s from './ToggleGroup.module.css';

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
    <div className={s.group}>
      <div className={s.label}>{label}</div>
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

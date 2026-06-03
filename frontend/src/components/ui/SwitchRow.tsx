import s from './SwitchRow.module.css';

interface SwitchRowProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}

export function SwitchRow({ label, checked, onChange }: SwitchRowProps) {
  return (
    <div className={s.row}>
      <span className={s.label}>{label}</span>
      <div
        role="switch"
        aria-checked={checked}
        aria-label={label}
        data-checked={checked}
        onClick={() => onChange(!checked)}
        className={s.track}
      >
        <div className={s.knob} />
      </div>
    </div>
  );
}

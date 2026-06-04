import { useEffect, useMemo, useState } from 'react';
import type { Filament } from '../../lib/types';
import {
  displayName,
  groupByMaterial,
  matchesQuery,
  withIndices,
} from '../../lib/filamentGroups';
import s from './FilamentManager.module.css';

interface FilamentManagerProps {
  open: boolean;
  onClose: () => void;
  filaments: Filament[];
  /** Toggle a single filament by its index in the original array. */
  onToggleIndex: (index: number) => void;
  /** Apply a bulk update to the whole filament array. */
  onSetFilaments: (updater: (fs: Filament[]) => Filament[]) => void;
}

/**
 * Slide-over panel for managing a large palette: search, bulk actions,
 * "active only" filter and per-material groups. Overlays the settings columns
 * while the preview above stays visible (live feedback when toggling).
 */
export function FilamentManager({
  open,
  onClose,
  filaments,
  onToggleIndex,
  onSetFilaments,
}: FilamentManagerProps) {
  const [query, setQuery] = useState('');
  const [onlyActive, setOnlyActive] = useState(false);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [open, onClose]);

  const activeCount = useMemo(() => filaments.filter((f) => f.active).length, [filaments]);

  const groups = useMemo(() => {
    const visible = withIndices(filaments).filter(
      ({ filament }) => matchesQuery(filament, query) && (!onlyActive || filament.active),
    );
    return groupByMaterial(visible);
  }, [filaments, query, onlyActive]);

  if (!open) return null;

  // The visible (filtered) indices — bulk actions respect the current filter.
  const visibleIndices = new Set(groups.flatMap((g) => g.items.map((i) => i.index)));
  const setVisible = (active: boolean) =>
    onSetFilaments((fs) => fs.map((f, i) => (visibleIndices.has(i) ? { ...f, active } : f)));
  const invertVisible = () =>
    onSetFilaments((fs) => fs.map((f, i) => (visibleIndices.has(i) ? { ...f, active: !f.active } : f)));
  const setGroup = (indices: number[], active: boolean) => {
    const set = new Set(indices);
    onSetFilaments((fs) => fs.map((f, i) => (set.has(i) ? { ...f, active } : f)));
  };

  return (
    <>
      <div className={s.backdrop} onClick={onClose} />
      <div className={s.panel} role="dialog" aria-label="Filamente verwalten">
        <header className={s.header}>
          <div className={s.title}>Filamente</div>
          <div className={s.count}>
            {activeCount}/{filaments.length} aktiv
          </div>
          <button className={s.close} onClick={onClose} aria-label="Schließen">
            ✕
          </button>
        </header>

        <div className={s.toolbar}>
          <input
            className={s.search}
            placeholder="Suchen (Name, Material, Hex)…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <div className={s.bulk}>
            <button className={s.bulkBtn} onClick={() => setVisible(true)}>
              Alle
            </button>
            <button className={s.bulkBtn} onClick={() => setVisible(false)}>
              Keine
            </button>
            <button className={s.bulkBtn} onClick={invertVisible}>
              Invertieren
            </button>
          </div>
          <label className={s.onlyActive}>
            <input
              type="checkbox"
              checked={onlyActive}
              onChange={(e) => setOnlyActive(e.target.checked)}
            />
            Nur aktive
          </label>
        </div>

        <div className={s.scroll}>
          {groups.length === 0 && <div className={s.emptyHint}>Keine Treffer</div>}
          {groups.map((g) => {
            const inGroup = g.items.length;
            const activeInGroup = g.items.filter((i) => i.filament.active).length;
            const indices = g.items.map((i) => i.index);
            return (
              <section key={g.material} className={s.group}>
                <div className={s.groupHead}>
                  <span className={s.groupName}>{g.material}</span>
                  <span className={s.groupCount}>
                    {activeInGroup}/{inGroup}
                  </span>
                  <button className={s.groupBtn} onClick={() => setGroup(indices, true)}>
                    Alle
                  </button>
                  <button className={s.groupBtn} onClick={() => setGroup(indices, false)}>
                    Keine
                  </button>
                </div>
                <div className={s.grid}>
                  {g.items.map(({ filament, index }) => (
                    <button
                      key={index}
                      type="button"
                      className={s.card}
                      data-active={filament.active}
                      onClick={() => onToggleIndex(index)}
                      title={`${filament.name} · ${filament.color}`}
                    >
                      <span className={s.check} data-active={filament.active} />
                      <span className={s.swatch} style={{ background: filament.color }} />
                      <span className={s.cardName}>{displayName(filament.name)}</span>
                      <span className={s.cardHex}>{filament.color}</span>
                    </button>
                  ))}
                </div>
              </section>
            );
          })}
        </div>
      </div>
    </>
  );
}

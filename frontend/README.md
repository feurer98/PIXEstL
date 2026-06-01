# PIXEstL Frontend (React)

React-/TypeScript-Portierung des Single-File-Prototyps
`PIXEstL Lithophane Converter.html` (Branch `feurer98-frontend`).

Das UI konfiguriert die Parameter eines Farb-Lithophans und zeigt eine
2D-Live-Vorschau. Die eigentliche 3D-/STL-/3MF-Geometrie erzeugt das Rust-CLI
unter [`/rust`](../rust) — siehe offener Punkt **V-MODEL-07**.

## Entwicklung

```bash
cd frontend
npm install
npm run dev        # Vite Dev-Server
npm run typecheck  # tsc -b --noEmit
npm test           # Vitest (Unit-Tests der reinen Logik)
npm run build      # Produktionsbuild nach dist/
```

## Struktur

```
src/
  lib/        reine, testbare Logik (color, canvas, palette, types, constants)
  theme/      Themes + ThemeProvider (CSS-Custom-Properties)
  state/      settingsReducer
  hooks/      useImageLoader, usePaletteLoader, useLithophaneCanvas, useExport
  components/ ui/ (generisch) · preview/ · panels/ · layout/
  pages/      ConverterPage (komponiert das Layout)
```

## Dokumentation / Weiterarbeit (V-Modell)

- `docs/frontend/00-react-analyse.md` — Ausgangsanalyse des Prototyps
- `docs/frontend/01-architektur-entscheidungen.md` — Architekturentscheidungen (ADR)
- `docs/frontend/02-offene-punkte-und-vereinfachungen.md` — **offene Punkte,
  bewusste Vereinfachungen und Unstimmigkeiten mit Anforderungs-/Test-Traceability**

> Status: Phasen 0–5 des Migrationsplans umgesetzt (Setup, reine Logik, Theming,
> State, Vorschau, Panels). Phase 6 (echter Export) ist ein dokumentierter
> Platzhalter.
>
> Styling: vollständig auf **CSS Modules** umgestellt (Theme weiterhin über
> `--c-*`-Variablen), inkl. responsiver Breakpoints (V-MODEL-08) und eines
> berechneten oklch→Hex-Fallbacks für ältere Browser (V-MODEL-12).

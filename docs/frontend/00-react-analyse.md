# React-Implementierungsanalyse — PIXEstL Lithophan Konverter

Ausgangsbasis: `PIXEstL Lithophane Converter.html` (Branch `feurer98-frontend`).
Die Datei ist ein selbst-entpackender Bundle-Artefakt (~1,4 MB); die eigentliche
Anwendung ist ein **einzelner React-Prototyp** (React + ReactDOM + Babel
Standalone, JSX inline, ~630 Zeilen) mit ausschließlich Inline-Styles.

## 1. Komponentenstruktur

**Generische Primitive (im Prototyp bereits vorhanden):** `Slider`,
`ToggleGroup`, `SwitchRow`, `SectionLabel`, `StatGrid`, `Spinner`.

**Seitenspezifisch (neu herausgelöst):** `TopBar`, `PreviewPane`,
`SourcePreview`, `LithoPreview`, `DimensionsPanel`, `ColorLayerPanel`,
`TexturePanel`, `PalettePanel` (+ `FilamentList`, `UsageBreakdown`,
`ExportControls`), `ThemePanel`.

**Aus dem JSX gelöste Logik:** Color-Math, Canvas-Processing, Themes, Defaults
→ `src/lib/*`, `src/theme/*`.

Ordnerstruktur: siehe `frontend/README.md`.

## 2. State Management

- **`useReducer`** für das gekoppelte `settings`-Objekt (Aspect-Lock,
  Kalibrierungs-Übernahme) → `state/settingsReducer.ts`.
- **Lokaler `useState`** für `previewMode`, `lockAspect`, `showGrid`,
  `dragging`, `exportFormat`/`exportState`.
- **Context API** für das Theme (`theme/ThemeContext.tsx`) — ersetzt das
  Durchreichen des `T`-Objekts an jede Komponente.
- **Kein Redux** — bewusst, Begründung in `01-architektur-entscheidungen.md`.

## 3. Abhängigkeiten

`react`, `react-dom`, Vite + `@vitejs/plugin-react`, TypeScript, Vitest + jsdom.
Fonts (DM Sans / DM Mono) via Google-Fonts-Link in `index.html`. Keine
UI-Library, keine Icon-Library (Inline-SVGs), kein Router.

Später nötig (abhängig von Phase 6): `jszip` / `lib3mf` (Client-Export) **oder**
`@tanstack/react-query` (Backend-Export). Optional `three` für 3D-Vorschau.

## 4. API / Datenschnittstellen

- Bild-Upload und Paletten-JSON werden **clientseitig** gelesen (FileReader).
- Palette: zwei Formate werden unterstützt — siehe **V-MODEL-01**.
- Export ist im Prototyp ein Mock; echter Endpoint offen — siehe **V-MODEL-07**.

## 5. Styling

CSS-Custom-Properties aus einer Theme-Single-Source (`theme/themes.ts`), gesetzt
vom `ThemeProvider`; Komponenten nutzen `var(--c-*)`. Globale Resets/Keyframes in
`styles/global.css`. Responsive ist **noch nicht** umgesetzt — siehe
**V-MODEL-08**.

## 6./7. Offene Punkte und Plan

Offene Punkte, Vereinfachungen und Risiken sind vollständig in
`02-offene-punkte-und-vereinfachungen.md` als nachverfolgbare Einträge geführt.
Der priorisierte Plan (Phasen 0–7) ist dort den Einträgen zugeordnet.

# Architekturentscheidungen (ADR) — PIXEstL Frontend

Kurzformat. Jede Entscheidung ist umkehrbar dokumentiert, damit im V-Modell auf
der Entwurfsebene nachvollzogen werden kann, *warum* etwas so gebaut ist.

## ADR-01 — Build: Vite + TypeScript statt Babel-Standalone-im-Browser
**Status:** angenommen.
Der Prototyp transpiliert JSX zur Laufzeit (`<script type="text/babel">`). Für
ein wartbares Projekt wird zur Build-Zeit kompiliert. Vorteile: Typsicherheit,
Tree-Shaking, Tests. Kosten: Build-Toolchain nötig.

## ADR-02 — Styling: CSS-Custom-Properties statt Inline-`T`-Prop
**Status:** angenommen.
Der Prototyp reicht ein Theme-Objekt `T` an *jede* Komponente. Stattdessen
projiziert der `ThemeProvider` die aktive Theme-Definition (`theme/themes.ts`,
weiterhin Single Source of Truth) auf `--c-*`-Variablen am Wurzelelement.
Komponenten lesen `var(--c-*)`. Themewechsel = ein Variablen-Swap, kein
Prop-Drilling. Pseudo-Elemente (Slider-Thumb) liegen in `global.css`.
Alternative (Tailwind) verworfen: erfordert für drei oklch-Themes ohnehin
Custom-Properties.

## ADR-03 — State: useReducer (settings) + Context (theme) + lokaler useState
**Status:** angenommen.
Redux/Zustand sind für eine Einzelseite Overkill. Das gekoppelte `settings`-
Objekt rechtfertigt einen Reducer mit expliziten Actions; das selten wechselnde
Theme passt zu Context. Bei Wachstum (mehrere Seiten, Persistenz) ist **Zustand**
der empfohlene nächste Schritt (kein Redux).

## ADR-04 — Canvas-Refs in der Seite, Logik in reinen Modulen
**Status:** angenommen.
`processCanvas`/Color-Math sind framework-frei in `lib/` und unit-getestet. Die
Seite hält die Canvas-Refs und reicht sie an die Preview-Komponenten; die
Statistik fließt über den `useLithophaneCanvas`-Hook nach oben. Das entspricht
der Refs+Effects-Struktur des Prototyps, aber testbar zerlegt.

## ADR-05 — Export bleibt vorerst Platzhalter
**Status:** offen (siehe V-MODEL-07).
Die Mesh-Erzeugung leistet das Rust-CLI. Bis die Schnittstelle (Backend-API vs.
WASM) entschieden ist, bildet `useExport` nur die Zustandsmaschine ab. Die
Signatur ist real, damit später nur der Funktionsrumpf getauscht wird.

## ADR-06 — Paletten-Parser akzeptiert beide Formate
**Status:** angenommen (siehe V-MODEL-01).
`lib/palette.ts` normalisiert sowohl das lose Prototyp-Schema als auch das echte
hex-keyed PIXEstL-Format. Per-Layer-HSL-Daten werden derzeit verworfen.

## ADR-07 — Edit-Mode-Bridge entfernt
**Status:** angenommen.
Die `postMessage`-„edit mode"-Brücke des Prototyps ist ein reines Artefakt der
Bundler-Umgebung. Ersetzt durch ein echtes In-App-Einstellungs-Panel
(`ThemePanel` + Zahnrad-Button).

# Offene Punkte, Vereinfachungen & Unstimmigkeiten

Dieses Dokument hält alle bewussten Vereinfachungen, strukturellen
Unstimmigkeiten zwischen Prototyp und realem Backend, Risiken und offenen
Entscheidungen fest, die während der Portierung aufgefallen sind.

Ziel: Weiterarbeit nach dem **V-Modell**. Jeder Eintrag ist mit einer (zunächst
abgeleiteten) Anforderung und einer **Verifikationsebene** versehen, damit aus
diesem Backlog Spezifikation und Testfälle der jeweiligen Ebene entstehen
können.

## V-Modell-Zuordnung (Legende)

| Ebene (Definition) | Gegenstück (Verifikation) |
|---|---|
| Anforderungen | Abnahmetest |
| Systemarchitektur | Systemtest |
| Komponentendesign | Integrationstest |
| Modulimplementierung | Unit-Test |

Status: `offen` · `umgesetzt` · `entscheidung-nötig`.

## Übersicht

| ID | Typ | Kurzbeschreibung | V-Ebene | Status |
|----|-----|------------------|---------|--------|
| V-MODEL-01 | Unstimmigkeit | Reales Palettenformat ≠ Prototyp-Schema | Komponentendesign / Integrationstest | umgesetzt (Daten verworfen) |
| V-MODEL-02 | Vereinfachung | Vorschau ignoriert Wölbung (`curve`) | Systemarchitektur / Systemtest | offen |
| V-MODEL-03 | Vereinfachung | Vorschau ignoriert additive Schichtmischung / `colorLayers` | Systemarchitektur / Systemtest | offen |
| V-MODEL-04 | Vereinfachung | AMS-Limit (`amsColors`) ohne Wirkung | Anforderungen / Abnahmetest | offen |
| V-MODEL-05 | Vereinfachung | `pixelMethod` (Additiv/Voll) ohne Wirkung | Anforderungen / Abnahmetest | offen |
| V-MODEL-06 | Risiko | `processCanvas` synchron im Main-Thread | Komponentendesign / Integrationstest | teilweise (Debounce) |
| V-MODEL-07 | Offene Entscheidung | Echter Export (Backend vs. WASM) fehlt | Systemarchitektur / Systemtest | umgesetzt (Backend) |
| V-MODEL-13 | Unstimmigkeit | UI-Filament-Toggles wirken nicht auf Backend-Export | Komponentendesign / Integrationstest | umgesetzt |
| V-MODEL-08 | Lücke | Kein Responsive-Layout | Anforderungen / Abnahmetest | umgesetzt (Breakpoints) |
| V-MODEL-09 | Lücke | Texturschicht-Toggles ohne Vorschau-Wirkung | Komponentendesign / Integrationstest | offen |
| V-MODEL-10 | Lücke | Keine Persistenz (Settings/Palette/Projekt) | Anforderungen / Abnahmetest | offen |
| V-MODEL-11 | Vereinfachung | i18n: UI hart deutsch | Anforderungen / Abnahmetest | offen |
| V-MODEL-12 | Risiko | `oklch()`/Browser-Support, kein Fallback-Konzept | Systemarchitektur / Systemtest | umgesetzt (Hex-Fallback) |

---

## Details

### V-MODEL-01 — Reales Palettenformat weicht vom Prototyp ab
**Befund.** Der Prototyp-Parser erwartet `{ filaments|colors: [{name, color}] }`.
Die echten Dateien in [`/palette`](../../palette) sind dagegen **nach Hex-Farbe
verschlüsselte Objekte** mit Per-Layer-HSL-Kalibrierung:

```jsonc
{ "#0086D6": { "name": "Cyan[PLA Basic]", "active": true,
               "layers": { "5": {H,S,L}, "4": {…}, … } } }
```

**Umsetzung.** `lib/palette.ts` erkennt und normalisiert **beide** Formate
(Unit-Tests in `palette.test.ts`). Die Hex-Schlüssel werden zur Basisfarbe.
**Vereinfachung:** die `layers`-HSL-Daten (additive Mischkalibrierung) werden
aktuell **verworfen** — die 2D-Vorschau nutzt nur die Basisfarbe.
**Offen für später.** Die Layer-Daten erlauben eine realistischere Vorschau und
gehören in das Color-Datenmodell (Verknüpfung mit V-MODEL-03). Außerdem kodiert
der Dateiname (`*-0.10mm.json`) die Schichtdicke — bislang nicht ausgewertet.
**Verifikation.** Integrationstest „realer Palette-Import setzt aktive
Filamente + Kalibrierung korrekt".

**Default-Fall gelöst.** Eine echte, kalibrierte Default-Palette ist ins Frontend
gebündelt (`src/assets/default-palette.json`, geladen über
`lib/defaultPalette.ts`). Sie dient Anzeige **und** Export, sodass der Export
ohne Upload funktioniert (vorher Fehlermeldung „Palette laden"). Die volle
Layer-Kalibrierung bleibt erhalten, weil die Originalbytes hochgeladen werden.
Regressionstest: `defaultPalette.test.ts` (gültig + aktives Weiß).

### V-MODEL-02 — Wölbung ohne Vorschau-Wirkung
`settings.curve` (0–180°) ist als Regler vorhanden, beeinflusst die Vorschau
nicht (nur Textanzeige). Spezifikation nötig: zylindrische Projektion in der
Vorschau vs. nur Parameter-Weitergabe an das CLI. **Abhängig von V-MODEL-07.**

### V-MODEL-03 — Additive Mischung / Schichtanzahl ohne Wirkung
`colorLayers`, `colorLayerThickness`, `pixelMethod` beeinflussen die Vorschau
nicht. Das echte PIXEstL mischt transparente Filamente additiv über gestapelte
Schichten (vgl. `layers`-Daten aus V-MODEL-01). Eine getreue Vorschau müsste das
CMYK-/HSL-Schichtmodell nachbilden. Entscheidung: Wie nah soll die Vorschau am
Druckergebnis sein?

### V-MODEL-04 — AMS-Farblimit ohne Wirkung
`amsColors` (0 = unbegrenzt) begrenzt im echten Workflow die gleichzeitig
geladenen Filamente (Bambu AMS). Vorschau/Matching berücksichtigen das Limit
nicht. Anforderung klären: harte Begrenzung der aktiven Palette?

### V-MODEL-05 — Pixelmethode ohne Wirkung
`pixelMethod` (Additiv/Voll) ist nur ein Toggle. Bedeutung und Auswirkung auf
Matching/Geometrie spezifizieren.

### V-MODEL-06 — Performance des Canvas-Processing
`processCanvas` läuft synchron über alle Blöcke bei jeder Settings-Änderung.
**Umgesetzt:** Debounce (80 ms) im `useLithophaneCanvas`-Hook. **Offen:**
Auslagern in einen Web Worker / `OffscreenCanvas` bei feiner Pixelbreite (viele
Blöcke). Integrationstest mit großem Bild + kleiner Pixelbreite (Frame-Budget).

### V-MODEL-07 — Echter Export
**Umgesetzt: Backend-Variante.** Entscheidung gefallen für einen
**Rust/axum-Server, der das CLI-Binary aufruft**, **asynchron mit Job +
Polling**. Implementierung unter [`/server`](../../server):
- `POST /api/jobs` (multipart `image`+`palette`+`settings`+`format`) → `202
  { jobId }`, startet `pixestl` als Subprozess.
- `GET /api/jobs/:id` → `queued|running|done|error` (CLI-stderr im Fehlerfall).
- `GET /api/jobs/:id/download` → fertiges 3MF/ZIP.

Frontend: `useExport` lädt Original-**Bild- und Palettendatei** hoch (beide jetzt
in `useImageLoader`/`usePaletteLoader` vorgehalten), pollt den Job und stößt den
Browser-Download an. Dev: Vite proxyt `/api` → `:8787`; `VITE_API_BASE` für
separaten Host. End-to-End gegen Beispielbild + Palette verifiziert (valides
3MF und ZIP). Mapping `settings` → CLI-Flags: siehe `server/README.md`.

**Offen (Härtung vor geteiltem Deployment):** Job-Cleanup/TTL, Concurrency-Limit,
restriktive CORS, ggf. Fortschritts-Prozent statt nur Status.

### V-MODEL-08 — Responsive-Layout
**Umgesetzt** (Styling-Phase 4). `ConverterPage.module.css` macht `.page` zum
einzigen Scroll-Container; Breakpoints: > 1024 px = Desktop unverändert,
≤ 1024 px = Panels als 2×2 mit Seiten-Scroll, ≤ 640 px = alles einspaltig
(Vorschauen erhalten dabei eine Mindesthöhe). **Offen:** echte Geräte-Tests und
ggf. ein dediziertes Mobile-Drawer-Muster statt reinem Stacking.

### V-MODEL-09 — Layer-Toggles ohne Vorschau-Wirkung
`enableColor` / `enableTexture` schalten Schichten nur konzeptionell. In Vorschau
und Export-Mapping müssen sie wirken (z. B. nur-Textur-Druck).

### V-MODEL-10 — Persistenz
Settings, Palette und „Projekt" gehen bei Reload verloren. Optionen: localStorage,
URL-State, Export/Import einer Projektdatei. Anforderung offen.

### V-MODEL-11 — Internationalisierung
UI-Texte sind hart deutsch. Falls Mehrsprachigkeit gewünscht: i18n-Schicht
einziehen (Texte zentralisieren).

### V-MODEL-12 — Browser-Support / oklch
**Umgesetzt** (Styling-Phase 5). `lib/oklch.ts` rechnet die oklch-Theme-Tokens
bei Bedarf **berechnet** (oklch→oklab→sRGB, nicht geraten) nach Hex; der
`ThemeProvider` prüft `CSS.supports('color','oklch(0 0 0)')` einmalig und setzt
nur bei fehlender Unterstützung die Hex-Fallbacks. Unit-Tests in
`oklch.test.ts`. **Offen:** Zielbrowser-Matrix formal festlegen; analog Fallback
für `DecompressionStream`/`OffscreenCanvas` (Bezug V-MODEL-06).

### V-MODEL-13 — UI-Filament-Toggles im Backend-Export
**Umgesetzt** (Lösungsweg a). Das Frontend sendet die aktiven Filament-Farben
als `activeColors` (Hex-Liste) mit; `useExport` hängt sie an den Multipart-Request.
Der Server (`apply_active_toggles`) setzt im hochgeladenen, hex-keyed Palette-JSON
pro Farbe das `active`-Flag entsprechend (case-insensitiv), bevor das CLI läuft.
Vorschau (filtert bereits nach `active`) und Export sind damit konsistent.
**Verifiziert** (Integrationstest, manuell): Baseline = 8 Farb-Layer; mit nur
White+Cyan+Magenta+Yellow aktiv enthält das ZIP genau diese 4 — die deaktivierten
Farben fehlen. **Hinweis:** Im additiven Modus erzwingt das CLI ein aktives
`#FFFFFF`; wird Weiß deaktiviert, meldet der Export einen entsprechenden
CLI-Fehler (erwartetes Verhalten).

---

## Nicht übernommene Prototyp-Bestandteile

- **Bundler-/Unpacking-Logik** (`__bundler/*`, base64-Assets, DOMParser-Swap):
  reines Artefakt der Artefakt-Umgebung.
- **`postMessage` Edit-Mode-Bridge** (`__activate_edit_mode` …): ersetzt durch
  `ThemePanel` (siehe ADR-07).
- **Eingebettete woff2-Fonts**: ersetzt durch Google-Fonts-Link.

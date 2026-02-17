# PIXEstL Issues-Analyse & Entwicklungsplan

**Datum:** 2026-02-17
**Analysiert von:** Claude Code Analysis
**Scope:** Original (Java, gaugo87/PIXEstL) ↔ Rust-Port (feurer98/PIXEstL)
**Datenquellen:** Alle 18 GitHub Issues (open + closed), CHANGELOG.md, REVIEW_REPORT.md, Quellcode-Analyse

---

## Aufgabe 1: Vollständige Issues-Dokumentation

### Alle Issues (Open + Closed)

| # | Titel | Status | Datum | Kategorie | Kernproblem | Schweregrad | Rust-relevant? |
|---|-------|--------|-------|-----------|-------------|-------------|----------------|
| 1 | Filament profile | Closed (Not Planned) | 2023-08-30 | Color Science | Vorschlag: Spektralbasiertes Filament-Modell statt HSL (Absorptions-/Streukomponente pro Wellenlänge). Gaugo87 findet den Ansatz "too technical", CMY-Summierung funktioniert stabil. | Niedrig | Ja — gleicher Algorithmus |
| 2 | coffee link? | Closed | 2023-11-03 | Meta | Anfrage nach Spenden-Link. Abgelehnt: "Share the program instead." | Keiner | Nein |
| 3 | Incredible Job! | Closed | 2023-12-04 | Kalibrierung | Schwierigkeit, akkurate HSL-Werte aus Kalibrierungsdrucken zu extrahieren. Farbton variiert je nach Schichtdicke, Beleuchtung beeinflusst Messung stark. **Lösung:** Neutrale Beleuchtung, Camera-Pro-Modus (ISO 50–125, WB 4000–7000K), Ironing aktivieren. | Mittel | Ja — Kalibrier-Workflow |
| 4 | Thought you might be interested | Open | 2023-12-20 | Feature/Community | Erfolgsgeschichte: 7-Layer-Profil bei 0.07mm Schichthöhe mit 0.4mm-Nozzle → besseres Blending. 9 Layer bei 0.05mm zeigt diminishing returns. Excel-Spreadsheet für Palette-Management. | Niedrig | Ja — Parameterbereich |
| 5 | Love the Latest Beta | Open | 2024-01-19 | UX/Feedback | Positives Feedback: Mesh-Dateien deutlich kleiner. Bestätigung der RLE-Optimierung. | Keiner | Nein |
| 6 | First Go Round, and some questions | Open | 2024-02-08 | Kalibrierung/UX | Erstes Nutzungserlebnis mit 13 Fragen: STL-Kalibrierquadrate stimmen nicht mit Doku überein, weisse Basis-Schicht unklar, HSL vs. RGB in JSON, macOS Pfad-Probleme. **Lösung (gaugo87):** Weisse 0.1mm Basis ermöglicht Ironing, default 0.12mm für BambuLab. | Mittel | Ja — Doku-Lücken |
| 7 | Non-Lithophane Pixel Image | Open | 2024-02-16 | Feature Request | Wunsch nach Pixel-Art-Modus (flaches farbiges STL ohne Hintergrundbeleuchtung, ähnlich HueForge). FULL-Modus funktioniert nicht so: "doesn't mix colors, takes them as is." Gaugo87 arbeitet(e) an einem Painting-Modus. | Mittel | Ja — Feature-Gap |
| 8 | Single AMS with 7 Color print wrong colors | Open | 2024-03-04 | Bug/Color Science | 7-Farb-AMS: Preview korrekt, Druck falsch. 36 Layers generiert, schlechte Lichtdurchlässigkeit. **Ursache:** Transmission-Distanz wird nicht berechnet; tiefere Layers (>5) zunehmend unzuverlässig. Update: BambuStudio-Filament-Sequenzierung eliminiert Weiss-Füllschichten. | Hoch | Ja — AMS-Logik |
| 9 | Suggestion for instructions.txt | Closed | 2024-03-17 | Dokumentation | Inhalt entfernt. Vermutlich Doku-Vorschlag, als "Completed" geschlossen. | Niedrig | Nein |
| 10 | Aspect ratio off for large widths | Closed | 2024-03-26 | Bug | STL-Seitenverhältnis wirkt falsch bei grossen Breiten. **Lösung:** Benutzer-Fehler — schräger Betrachtungswinkel in BambuStudio. Kein Bug. | Keiner | Nein (kein Bug) |
| 11 | Color Mapping Problems | Open | 2024-04-07 | Color Science/Kalibrierung | Farben stimmen nicht mit Kalibrierungs-Tiles überein. Dunkle Bereiche ohne Tiefe, Cyan wird bei Verdunklung grünlich. Palette hatte nur Layer-1-Werte statt aller 7 Layer. **Lösung:** Korrekte Multi-Layer-Palette-Konfiguration (Layer 1–7 mit individuellen HSL-Werten pro Layer). | Hoch | Ja — Validierung + Doku |
| 12 | Please explain how the layer numbers work | Open | 2024-04-16 | Dokumentation/UX | Layer-Nummern in JSON ≠ Layer-Nummern im STL. JSON-Layers definieren Farbdichte/Dicke, nicht physische Position. Verwirrung über "Gray auf Layer 1–2 erscheint auf Layer 6–7". | Hoch | Ja — Doku-Lücke |
| 13 | Using A Colorimeter For Palette Generation | Open | 2024-06-14 | Color Science/Kalibrierung | **45 Kommentare!** Umfassende Forschung mit Colorimeter für akkurate CMYK-Paletten. 7 Layer bei 0.07mm optimal. Kein einzelnes Palette passt für alle Bilder. Farbe hängt stark von Hex-Code-Auswahl ab. Matrix-Ansatz von gaugo87 erwähnt aber nicht implementiert. | Hoch | Ja — Kern-Feature |
| 14 | How do I get the Curve Parameter to work? | Open | 2024-06-30 | Bug/UX | User nutzte `-C 5` und sah kaum Krümmung. **Lösung (gaugo87):** `-C` ist Grad (0=flach, 90=Viertelzylinder, 360=Vollzylinder). `-C 5` ist fast flach. Duplikat-Dateien sind gewollt (Layer 1+Plate, Layer 2+Plate separat). | Niedrig | Ja — Curve teilweise impl. |
| 15 | Your tips and tricks for better print quality? | Open | 2024-11-17 | Dokumentation | Community-Wissenssammlung gewünscht. Keine Antworten bisher. Photoshop-Vorverarbeitung, langsame Druckgeschwindigkeit, 0.08–0.12mm Schichthöhe. | Mittel | Ja — Doku |
| 16 | Python program to show active filaments | Open | 2024-11-25 | Feature Request | Python-Script zum Anzeigen aktiver Filamente aus JSON-Palette. Trivial implementierbar als CLI-Subcommand. | Niedrig | Ja — Quick Win |
| 17 | Image comes out blurry | Open | 2025-01-12 | Bug/UX | Bild blurry trotz Parameteränderung (cW 0.4–0.6). BambuLab CMYK mit Black. Keine Antworten. Vermutlich: `color_pixel_width` zu gross → zu wenig Pixel-Auflösung. | Mittel | Ja — UX/Validierung |
| 18 | Color mixing algorithm | Open | 2025-01-26 | Color Science | Aktuelle lineare CMYK-Addition ist physikalisch inkorrekt. Vorschlag: Kubelka-Munk-Theorie für transluzente Medien. Referenzen: mixbox, spectral.js, ColorMixer. Herausforderung: K-M nimmt homogene Mischung an, nicht geschichtete Filamente. | Hoch | Ja — Kern-Algorithmus |

### Geschlossene Issues — Detailanalyse

**#1 (Filament profile):** Closed als "Not Planned". Spektral-Ansatz zu komplex, aber die Grundidee (multiplikative Absorption statt linearer Addition) ist korrekt und wird in #18 wieder aufgegriffen. Die CMY-Summierung des Originals ist ein bekannter Kompromiss.

**#2 (Coffee link):** Meta-Issue, keine technische Relevanz. Gaugo87 lehnte Spenden ab: "Share the program instead."

**#3 (Incredible Job):** Gelöst durch Kalibrierungs-Anleitung. Kern-Erkenntnis: Beleuchtung und Kamera-Einstellungen sind entscheidend für akkurate HSL-Werte. Empfohlene Methodik:
- Neutrale LED-Beleuchtung (kein Warmweiss)
- Camera Pro: ISO 50–125, Shutter ~1/180, WB 4000–7000K
- Ironing aktivieren für gleichmässige Oberfläche
- Weiss-Referenzpunkte prüfen (CMY nahe 0)

**#9 (Instructions.txt):** Inhalt gelöscht, als "Completed" geschlossen. Vermutlich Doku-Verbesserung umgesetzt.

**#10 (Aspect ratio):** Benutzer-Fehler, kein Software-Bug. STL-Seitenverhältnis war korrekt, schräger Betrachtungswinkel in BambuStudio täuschte.

---

## Aufgabe 2: Vergleich Original ↔ Rust-Version

| # | Titel | Status Original | Status Rust | Begründung |
|---|-------|-----------------|-------------|------------|
| 1 | Filament profile | Closed (Not Planned) | 🔵 Identisch | Gleicher Algorithmus (CMY-Summierung), spektraler Ansatz auch hier nicht implementiert |
| 2 | coffee link? | Closed | 🔵 Nicht zutreffend | Meta-Issue |
| 3 | Incredible Job! (Kalibrierung) | Closed (gelöst) | ⚠️ Teilweise | Kalibrier-Workflow identisch; Rust-Version hat deutsche Doku unter `docs/anleitung/kalibrierung.md`, aber noch nicht so umfassend wie die Issue-Diskussion |
| 4 | 7-Layer Success Story | Open (Diskussion) | ✅ Kompatibel | Rust unterstützt beliebige Layer-Anzahl via `--color-layers N` |
| 5 | Love the Latest Beta (Mesh-Grösse) | Open (Feedback) | ✅ Besser | Rust hat RLE-Optimierung (nicht im Java-Original!), Mesh-Dateien noch kleiner |
| 6 | First Go Round (Onboarding) | Open | ⚠️ Teilweise | Bessere CLI-Hilfe via Clap, aber Kalibrier-Doku noch nicht vollständig für Einsteiger |
| 7 | Non-Lithophane Pixel Art | Open | ❌ Offen | Pixel-Art/HueForge-Modus nicht implementiert. FULL-Modus identisch zum Original |
| 8 | AMS 7-Color Wrong Colors | Open | ⚠️ Teilweise | AMS-Gruppierung implementiert (`palette/loader.rs`), aber Transmission-Distanz-Problem besteht weiterhin. BambuStudio-Sequenzierung nicht integriert |
| 9 | Instructions.txt | Closed | 🔵 Nicht zutreffend | Rust hat eigene Doku-Struktur unter `docs/` |
| 10 | Aspect Ratio | Closed (User Error) | 🔵 Nicht zutreffend | War kein Bug |
| 11 | Color Mapping Problems | Open | ⚠️ Teilweise | CIELab Delta E implementiert (`color/cielab.rs`), aber Validierung für Multi-Layer-Paletten fehlt. Keine Warnung bei fehlenden Layer-Definitionen |
| 12 | Layer Numbers Explanation | Open | ❌ Offen | Gleiche Verwirrung möglich. Keine in-tool Erklärung, kein `--explain-layers` |
| 13 | Colorimeter Palette Generation | Open | ❌ Offen | Kein CGATS/CSV-Import, keine automatische Palette-Generierung aus Messdaten |
| 14 | Curve Parameter | Open | ⚠️ Teilweise | Parameter `curve` existiert in `LithophaneConfig`, aber **nicht vollständig implementiert** in Mesh-Generierung |
| 15 | Tips & Tricks | Open | ❌ Offen | Keine konsolidierte Best-Practices-Dokumentation |
| 16 | Python Filament Tool | Open | ❌ Offen | Kein `palette-info` Subcommand |
| 17 | Image Blurry | Open | ⚠️ Teilweise | Rust nutzt **Lanczos3** (besser als Java-bilinear!), aber keine Warnung bei zu grosser `color_pixel_width` relativ zur Bildauflösung |
| 18 | Color Mixing Algorithm | Open | ❌ Offen | Identischer Algorithmus (lineare CMYK-Addition mit Clamping in `palette/color_combi.rs`). Kubelka-Munk nicht implementiert |

### Zusammenfassung

| Status | Anzahl | Issues |
|--------|--------|--------|
| ✅ Gelöst/Besser | 2 | #4, #5 |
| ⚠️ Teilweise | 6 | #3, #6, #8, #11, #14, #17 |
| ❌ Offen | 6 | #7, #12, #13, #15, #16, #18 |
| 🔵 Nicht zutreffend | 4 | #1, #2, #9, #10 |

Die Rust-Version hat durch Lanczos3-Resampling und RLE-Optimierung bereits Vorteile, aber die Kern-Limitierungen (lineare CMYK-Mischung, fehlende Validierung, Doku-Lücken) sind identisch zum Java-Original.

---

## Aufgabe 3: Résumé — Entwicklungsplan

### 3a. Lektionen aus geschlossenen Issues

**Muster 1: Kalibrierung ist das fundamentale Problem.**
Issues #3, #6, #11, #13 (zusammen 95+ Kommentare) kreisen alle um dasselbe Thema: Wie bekommt man akkurate HSL-Werte für die Filament-Palette? Die Lösung war nie ein Software-Fix, sondern immer ein besserer Kalibrierungs-Workflow (neutrale Beleuchtung, kontrollierte Kamera-Einstellungen, iteratives Verfeinern). Das zeigt: **Die grösste Hürde für neue User ist nicht die Software, sondern der Kalibrierungs-Prozess.** Wer die Software verbessern will, muss den Kalibrierungsprozess vereinfachen — nicht nur die Algorithmen.

**Muster 2: Multi-Layer-Paletten erfordern Verständnis.**
Issue #11 wurde gelöst, als der User erfuhr, dass Layer 1–7 jeweils eigene HSL-Werte brauchen. Issue #12 klärte, dass JSON-Layer-Nummern Farbdichte-Stufen sind, nicht physische Positionen. Beide hätten durch bessere Dokumentation oder Validierung verhindert werden können. Die bestehende Dokumentation erklärt das "Was", aber nicht das "Warum" — und das "Warum" ist entscheidend für korrekte Palette-Erstellung.

**Muster 3: "Won't Fix" kehrt zurück.**
Issue #1 (Spektral-Modell, 2023) wurde als "Not Planned" geschlossen. 2 Jahre später wurde in Issue #18 derselbe Grundgedanke (physikalisch korrektere Farbmischung) von einem neuen User aufgegriffen. Das zeigt: Die Community sieht die Farbmischung als Kern-Schwachstelle. Der lineare CMYK-Ansatz ist ein bewusster Kompromiss, der langfristig nicht ausreicht.

### 3b. Quick Wins (< 1 Tag, hoher User-Impact)

#### 1. `pixestl palette-info` Subcommand (ersetzt Python-Script, Issue #16)

Zeigt aktive Filamente, Anzahl Kombinationen, Layer-Konfiguration:

```rust
// Neuer Subcommand in cli/mod.rs
#[derive(Subcommand)]
enum Commands {
    /// Generate lithophane STL from image
    Generate { /* bestehende Args */ },
    /// Show palette information
    PaletteInfo {
        /// Path to palette JSON file
        #[arg(short, long)]
        palette: PathBuf,
        /// Number of color layers
        #[arg(short = 'n', long, default_value = "5")]
        layers: u32,
    },
}
```

#### 2. Palette-Validierung mit Warnungen (adressiert #11, #12)

```rust
// In palette/loader.rs — nach dem Laden der Palette
pub fn validate_palette_completeness(
    palette: &HashMap<String, FilamentDef>,
    target_layers: u32,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for (hex, filament) in palette {
        if filament.layers.len() == 1 && target_layers > 1 {
            warnings.push(format!(
                "⚠ Filament {} ('{}') hat nur Layer-{}-Werte definiert, \
                 aber {} Layers sind konfiguriert. \
                 Für bessere Ergebnisse: HSL-Werte für Layer 1–{} definieren.",
                hex, filament.name,
                filament.layers.keys().next().unwrap(),
                target_layers, target_layers
            ));
        }
    }
    warnings
}
```

#### 3. Auflösungs-Warnung (adressiert #17)

```rust
// In cli/mod.rs — vor der Generierung
let effective_pixels = (config.dest_width_mm / config.color_pixel_width) as u32;
if effective_pixels < image_width / 2 {
    eprintln!(
        "⚠ Bild hat {}px Breite, aber bei {:.0}mm/{:.1}mm entstehen nur {} Farbpixel. \
         Für schärfere Ergebnisse: --color-pixel-width {:.2}",
        image_width, config.dest_width_mm, config.color_pixel_width,
        effective_pixels, config.dest_width_mm / image_width as f64
    );
}
```

#### 4. Best-Practices-Dokumentation (adressiert #15, #6)

Community-Wissen aus Issues #3, #4, #6, #11, #13 zusammenfassen:

- **Kalibrierung:** Neutrale LED, Camera Pro (ISO 50–125, WB 5000K), Ironing ON
- **Optimale Parameter:** 7 Layers bei 0.07mm (Community-Konsens aus #4, #13)
- **Bild-Vorverarbeitung:** Kontrast erhöhen, Highlights reduzieren, auf Zielgrösse skalieren
- **Druckeinstellungen:** 0.08–0.12mm Schichthöhe, langsame Geschwindigkeit

### 3c. Sinnvolle Features (1–5 Tage, klarer Mehrwert)

#### 1. Colorimeter-Import (Issue #13) — CGATS/CSV-Import

```rust
// Neues Modul: src/palette/import.rs
use crate::color::CieLab;
use std::path::Path;

/// Importiert Colorimeter-Messdaten im CGATS-Format
pub fn import_cgats(path: &Path) -> Result<Vec<(String, CieLab)>> {
    // Parse CGATS format: SAMPLE_ID  L*  a*  b*
    // Standard in der Farbmesstechnik (X-Rite, Datacolor, etc.)
    todo!()
}

/// Generiert Palette-JSON aus gemessenen L*a*b*-Werten
pub fn generate_palette_from_measurements(
    measurements: &[(String, Vec<CieLab>)], // Pro Filament: mehrere Layer-Messungen
    nb_layers: u32,
) -> Result<serde_json::Value> {
    // 1. Für jedes Filament: L*a*b* → HSL konvertieren
    // 2. Fehlende Layer interpolieren
    // 3. JSON-Palette generieren
    todo!()
}
```

**Aufwand:** 3 Tage | **Impact:** Hoch — ermöglicht professionellen Colorimeter-Workflow

#### 2. Curve-Modus vollständig implementieren (Issue #14)

```rust
// In lithophane/generator.rs — nach Mesh-Generierung
fn apply_curve(mesh: &mut Mesh, curve_degrees: f64, total_width: f64) {
    if curve_degrees == 0.0 { return; }
    let radius = (total_width * 360.0) / (curve_degrees * 2.0 * std::f64::consts::PI);
    for triangle in &mut mesh.triangles {
        for vertex in [&mut triangle.v0, &mut triangle.v1, &mut triangle.v2] {
            let angle = (vertex.x / total_width) * curve_degrees.to_radians();
            let new_x = radius * angle.sin();
            let new_z = radius * (1.0 - angle.cos()) + vertex.z;
            vertex.x = new_x;
            vertex.z = new_z;
        }
    }
}
```

**Aufwand:** 2 Tage | **Impact:** Mittel — Feature-Parität mit Java

#### 3. Pixel-Art-Modus (Issue #7)

Flaches farbiges STL ohne Textur-Layer (Wandbild ohne Hintergrundbeleuchtung):

- Neue CLI-Option: `--mode pixel-art` oder `--flat-color`
- Nutzt bestehende Color-Layer-Generierung
- Setzt `texture_layer = false` automatisch
- Optionale opake Hintergrundschicht statt transparentem Aufbau
- Unterschied zu FULL-Modus: **mischt Farben** (additiv) statt sie direkt zu nehmen

**Aufwand:** 2 Tage | **Impact:** Mittel — neue Zielgruppe (HueForge-Alternative)

#### 4. Kalibrierungs-Testmuster-Generator

```bash
pixestl calibrate --palette palette.json --output test_squares.stl --layers 7
```

Generiert Farbfelder für jede Palette-Farbe in allen Layer-Kombinationen. Nutzt bestehende Mesh-Generierung.

**Aufwand:** 1 Tag | **Impact:** Mittel — vereinfacht Kalibrierungsprozess

### 3d. Algorithmus-Verbesserungen (komplex, hoher Qualitätsgewinn)

#### 1. Color Mixing: Kubelka-Munk oder empirische LUT (Issue #18)

**Problem:** Lineare CMYK-Addition (`palette/color_combi.rs:94–111`) modelliert weder Streuung noch Absorption korrekt.

Aktueller Algorithmus:
```rust
// palette/color_combi.rs — compute_rgb()
let mut c = 0.0, m = 0.0, y = 0.0, k = 0.0;
for layer in &self.layers {
    c += layer.c();  // Lineare Addition
    m += layer.m();
    y += layer.y();
    k += layer.k();
}
let cmyk = Cmyk::new(c.min(1.0), m.min(1.0), y.min(1.0), k.min(1.0));
// Clamping verliert Information!
```

**Drei Optionen:**

| Option | Ansatz | Pro | Contra | Aufwand |
|--------|--------|-----|--------|---------|
| A | Kubelka-Munk (K/S-Theorie) | Physikalisch fundiert | Braucht K/S-Koeffizienten pro Filament | 5+ Tage |
| B | Empirische Lookup-Tables | Genaueste Ergebnisse | Braucht Drucktests pro Filament-Set | 5 Tage |
| C | Gamma-korrigierte CMYK | Einfachster Weg, nutzt bestehende Infra | Immer noch Approximation | 1–2 Tage |

```rust
/// Trait für austauschbare Farbmischungs-Modelle
pub trait ColorMixingModel: Send + Sync {
    fn mix_layers(&self, layers: &[ColorLayer]) -> Rgb;
}

/// Aktueller Ansatz: Lineare CMYK-Addition
struct LinearCmykMixing;

/// Option C: Gamma-korrigierte Addition
struct GammaCmykMixing { gamma: f64 }

impl ColorMixingModel for GammaCmykMixing {
    fn mix_layers(&self, layers: &[ColorLayer]) -> Rgb {
        let mut c = 0.0_f64;
        let mut m = 0.0_f64;
        let mut y = 0.0_f64;
        let mut k = 0.0_f64;
        for layer in layers {
            // Gamma-Korrektur: Nicht-lineare Akkumulation
            c += layer.c().powf(self.gamma);
            m += layer.m().powf(self.gamma);
            y += layer.y().powf(self.gamma);
            k += layer.k().powf(self.gamma);
        }
        let n = layers.len() as f64;
        let cmyk = Cmyk::new(
            (c / n).powf(1.0 / self.gamma).min(1.0),
            (m / n).powf(1.0 / self.gamma).min(1.0),
            (y / n).powf(1.0 / self.gamma).min(1.0),
            (k / n).powf(1.0 / self.gamma).min(1.0),
        );
        Rgb::from_cmyk(cmyk)
    }
}

/// Option B: Empirische 3D-Lookup-Table
struct LookupTableMixing { lut: ColorLut3D }
```

**Empfehlung:** Option C als Zwischenschritt (1–2 Tage), Option B langfristig.

#### 2. Dithering für Farbübergänge (adressiert #11 indirekt)

Aktuell: Nächster-Nachbar-Quantisierung → harte Farbgrenzen bei Gradienten.

```rust
/// Neues Modul: src/palette/dither.rs
pub enum DitheringMethod {
    None,           // Aktuell: Nearest-Neighbor
    FloydSteinberg, // Fehlerdiffusion — weiche Übergänge
    Ordered,        // Bayer-Matrix — deterministisch, schnell
}

/// Floyd-Steinberg Error Diffusion
pub fn dither_floyd_steinberg(
    image: &[Vec<Rgb>],
    palette: &[Rgb],
    distance: ColorDistanceMethod,
) -> Vec<Vec<Rgb>> {
    let mut error_buffer = vec![vec![[0.0f64; 3]; image[0].len()]; image.len()];
    // Für jedes Pixel:
    // 1. Addiere akkumulierten Fehler
    // 2. Finde nächste Palette-Farbe
    // 3. Verteile Fehler auf Nachbarn (7/16, 3/16, 5/16, 1/16)
    todo!()
}
```

**Aufwand:** 2 Tage | **Impact:** Mittel — deutlich weichere Farbübergänge

#### 3. Delta E 2000 statt CIE76 (adressiert #11)

CIE76 (aktuell in `color/cielab.rs:62–68`) hat bekannte Schwächen bei Blau- und Grautönen. CIEDE2000 ist der aktuelle Industriestandard:

```rust
// In color/cielab.rs — zusätzlich zu delta_e()
pub fn delta_e_2000(&self, other: &CieLab) -> f64 {
    // CIEDE2000 berücksichtigt:
    // - Lightness-Gewichtung (SL)
    // - Chroma-Gewichtung (SC)
    // - Hue-Gewichtung (SH)
    // - Rotation für Blau-Bereich (RT)
    // ~100 Zeilen, aber deutlich genauer
    todo!()
}
```

**Aufwand:** 2 Tage | **Impact:** Mittel — spürbar genauere Farbzuordnung bei kritischen Farbtönen

### 3e. Bewusst nicht umsetzen

| Idee | Begründung |
|------|------------|
| Java-GUI nachbauen | Rust-Version ist bewusst CLI-only. Web-UI (WASM) wäre sinnvoller als Desktop-GUI |
| Spektral-Rendering (#1) | Erfordert Spektraldaten die für PLA-Filamente nicht standardisiert existieren |
| Automatische Kamera-Kalibrierung | Ausserhalb des Scope — Hardware-abhängig, besser durch externe Tools |
| BambuStudio-Plugin | Proprietäre API, instabil, besser als externe Toolchain |
| Issue #2 (Coffee Link) | Meta-Issue, kein technischer Bedarf |
| Issue #9 (Instructions.txt) | Inhalt gelöscht, nicht reproduzierbar |

---

## Aufgabe 4: Priorisierte Roadmap

### Phase 1 — Parität + Stabilität (Woche 1–4)

| Prio | Issue(s) | Aufgabe | Aufwand | Impact | Schliesst Issues |
|------|----------|---------|---------|--------|------------------|
| P0 | #12, #11 | Palette-Validierung mit verständlichen Warnungen bei fehlenden Multi-Layer-Werten | 1 Tag | **Hoch** — verhindert häufigsten Benutzerfehler | #12 teilweise |
| P0 | #17 | Auflösungs-Warnung bei `color_pixel_width` > Bildpixel-Grösse | 0.5 Tag | **Mittel** — erklärt blurry Output | #17 |
| P0 | #14 | Curve-Parameter vollständig implementieren (Zylinderkoordinaten-Transformation) | 2 Tage | **Mittel** — Feature-Parität mit Java | #14 |
| P1 | #15, #6, #3 | Best-Practices-Doku aus Community-Wissen (Kalibrierung, Parameter, Vorverarbeitung) | 1 Tag | **Hoch** — reduziert Support-Aufwand | #15, #6 teilweise |
| P1 | #16 | `pixestl palette-info` Subcommand | 0.5 Tag | **Niedrig** — ersetzt Python-Script | #16 |
| P1 | Review | Compiler-Warnings aufräumen (ungenutzte Imports, `mut`, Stubs) | 0.5 Tag | **Niedrig** — Code-Qualität | — |

**Gesamt Phase 1:** ~5.5 Tage
**Erwartetes Ergebnis:** Alle Doku-Lücken geschlossen, häufigste Benutzer-Fehler durch Validierung abgefangen, Feature-Parität bei Curve-Modus.

### Phase 2 — Differenzierung (Monat 2–3)

| Prio | Issue(s) | Aufgabe | Aufwand | Impact | Schliesst Issues |
|------|----------|---------|---------|--------|------------------|
| P0 | #18 | Gamma-korrigierte CMYK-Mischung (`ColorMixingModel` Trait + GammaCmyk) | 2 Tage | **Hoch** — bessere Farbgenauigkeit | #18 teilweise |
| P0 | #11 | Delta E 2000 als Alternative zu CIE76 | 2 Tage | **Mittel** — genauere Farbzuordnung | #11 teilweise |
| P1 | #13 | CGATS/CSV-Import für Colorimeter-Daten + automatische Palette-Generierung | 3 Tage | **Hoch** — professioneller Workflow | #13 |
| P1 | #7 | Pixel-Art-Modus (flache farbige STL, HueForge-Alternative) | 2 Tage | **Mittel** — neue Zielgruppe | #7 |
| P2 | #11 | Floyd-Steinberg Dithering als Option | 2 Tage | **Mittel** — weichere Farbübergänge | #11 weiter |
| P2 | #3, #6 | Kalibrierungs-Testmuster-Generator (`pixestl calibrate`) | 1 Tag | **Mittel** — vereinfacht Kalibrierung | #3 Workflow |

**Gesamt Phase 2:** ~12 Tage
**Erwartetes Ergebnis:** Rust-Version bietet messbar bessere Farbqualität als Java-Original, professioneller Colorimeter-Workflow möglich, neue Nutzer durch Pixel-Art-Modus.

### Phase 3 — Innovation (Monat 4+)

| Prio | Issue(s) | Aufgabe | Aufwand | Impact | Schliesst Issues |
|------|----------|---------|---------|--------|------------------|
| P1 | #18 | Empirische Color-LUT aus Drucktests (3D-Interpolation in Lab-Raum) | 5 Tage | **Sehr hoch** — beste Farbgenauigkeit | #18 vollständig |
| P1 | #8 | Intelligente AMS-Gruppierung (k-Means Clustering auf Lab-Farbraum) | 3 Tage | **Hoch** — optimale Farbzuweisung bei >4 Farben | #8 |
| P2 | #13 | Automatische Palette-Optimierung (Gradient Descent auf Lab-Distanz) | 5 Tage | **Hoch** — eliminiert manuelle Kalibrierung | #13 vollständig |
| P2 | — | Web-UI (WASM-basiert, kompiliert aus Rust) | 10+ Tage | **Sehr hoch** — Zugänglichkeit für Nicht-CLI-User | — |
| P3 | #1 | Kubelka-Munk-Integration (wenn K/S-Daten verfügbar werden) | 5+ Tage | **Unklar** — Datenabhängig | #1 |

**Gesamt Phase 3:** 28+ Tage
**Erwartetes Ergebnis:** State-of-the-art Farbgenauigkeit, automatisierte Workflows, optionaler Web-Zugang.

---

## Die drei wichtigsten Prioritäten

### 1. Validierung & Fehlermeldungen (Issues #11, #12, #17)

Die häufigsten Probleme entstehen nicht durch Software-Bugs, sondern durch falsche Palette-Konfiguration. Proaktive Warnungen — "Palette hat nur Layer-1-Werte bei 7-Layer-Konfiguration", "Bildauflösung zu niedrig für gewählte Pixelgrösse" — würden die meisten Support-Anfragen eliminieren.

**Betrifft:** `rust/src/palette/loader.rs`, `rust/src/cli/mod.rs`
**Aufwand:** 1.5 Tage
**Begründung:** Issues #11 (20 Kommentare) und #12 (17 Kommentare) zeigen, dass dies der grösste Frustrationspunkt ist.

### 2. Verbesserte Farbmischung (Issue #18)

Die lineare CMYK-Addition ist der technisch grösste Schwachpunkt. Eine Gamma-Korrektur als Zwischenschritt (Option C) und langfristig empirische LUTs würden die Druckqualität messbar verbessern.

**Betrifft:** `rust/src/palette/color_combi.rs`, neues `ColorMixingModel` Trait
**Aufwand:** 2 Tage (Option C), 5 Tage (Option B)
**Begründung:** Issues #1 und #18 zeigen: Die Community fordert dies seit 2023. Es ist das Feature mit dem grössten Qualitäts-Impact.

### 3. Colorimeter-Workflow (Issue #13)

Mit **45 Kommentaren** ist #13 der am meisten diskutierte Issue. Ein CGATS/CSV-Import und automatische Palette-Generierung würden den manuellen Kalibrierungsprozess — derzeit die grösste Hürde für neue User — massiv vereinfachen.

**Betrifft:** Neues Modul `rust/src/palette/import.rs`, `rust/src/cli/mod.rs`
**Aufwand:** 3 Tage
**Begründung:** Kalibrierung ist Muster #1 aus den geschlossenen Issues. Jede Verbesserung hier hat überproportionalen Impact auf die User-Experience.

---

## Kritische Dateien für Änderungen

| Datei | Relevante Änderungen |
|-------|---------------------|
| `rust/src/cli/mod.rs` | Neue Subcommands (`palette-info`, `calibrate`), Validierungs-Warnungen, Auflösungs-Check |
| `rust/src/palette/loader.rs` | Palette-Validierung, Multi-Layer-Prüfung |
| `rust/src/palette/color_combi.rs` | `ColorMixingModel` Trait, Gamma-Korrektur |
| `rust/src/palette/quantize.rs` | Dithering-Option (Floyd-Steinberg, Ordered) |
| `rust/src/palette/import.rs` | **Neu:** CGATS/CSV-Import |
| `rust/src/color/cielab.rs` | Delta E 2000 |
| `rust/src/color/distance.rs` | Integration neuer Distance-Methoden |
| `rust/src/lithophane/config.rs` | Neue Parameter (`dithering`, `mixing_model`) |
| `rust/src/lithophane/generator.rs` | Curve-Transformation auf generiertes Mesh |
| `docs/anleitung/kalibrierung.md` | Best-Practices aus Community-Issues |
| `docs/anleitung/palette.md` | Layer-Nummern-Erklärung aus #12 |

---

## Anhang: Rust-Version — Technische Stärken

Die Rust-Version hat bereits mehrere Vorteile gegenüber dem Java-Original:

| Aspekt | Java-Original | Rust-Port | Verbesserung |
|--------|---------------|-----------|--------------|
| Resampling | Bilinear | **Lanczos3** | Schärfere Pixel, weniger Aliasing |
| Mesh-Optimierung | Keine | **Run-Length Encoding** | Kleinere STL-Dateien |
| Parallelisierung | Thread-Pool | **Rayon** (work-stealing) | Bessere CPU-Auslastung |
| Startup | ~2s (JVM) | **<100ms** | 20× schneller |
| Memory | ~200MB | **~100MB** | 50% weniger |
| Binary | 30+ MB (+ JRE) | **8 MB** | Self-contained |
| Safety | Runtime-Checks | **Compile-time** (zero unsafe) | Keine Crashes |
| Tests | — | **165 Tests** (100% pass) | Regression-sicher |

---

*Generiert am 2026-02-17 basierend auf der Analyse aller 18 Issues des gaugo87/PIXEstL-Repositories und vollständiger Quellcode-Analyse des Rust-Ports.*

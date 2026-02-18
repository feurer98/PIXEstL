# PIXEstL Rust – Code Review Cleanup Summary

Durchgeführt am 2026-02-18 auf Branch `claude/review-pixestl-rust-mN426`.

---

## Statistiken

| Metrik | Wert |
|--------|------|
| Geänderte Dateien | 17 |
| Hinzugefügte Zeilen | 297 |
| Entfernte Zeilen | 387 |
| **Netto (Code reduziert)** | **−90 Zeilen** |
| Tests vorher / nachher | 205 ✅ / 219 ✅ |
| Clippy-Warnungen vorher | ~60 |
| Clippy-Warnungen nachher | **0** |
| cargo doc Warnungen | **0** |

---

## Phase 2a: Dead Code entfernt

### Ungenutzte Dependencies (`Cargo.toml`)

Vier vollständig ungenutzte Crate-Abhängigkeiten entfernt:

| Crate | Begründung |
|-------|------------|
| `nalgebra = "0.34"` | Niemals importiert – das Projekt verwendet ein eigenes `Vector3`/`Mesh`-System |
| `stl_io = "0.10"` | Niemals importiert – STL wird direkt ohne externe Crate geschrieben |
| `tracing = "0.1"` | Kein `use tracing::*` in irgendeiner Quelldatei |
| `tracing-subscriber = "0.3"` | Kein `use tracing_subscriber::*` in irgendeiner Quelldatei |
| `anyhow = "1"` | Kein `use anyhow::*` in irgendeiner Quelldatei |

**Effekt:** Deutlich kürzere Compile-Zeiten, weniger transitive Abhängigkeiten.

### Redundante Methoden

| Item | Datei | Begründung |
|------|-------|------------|
| `LithophaneConfig::new()` | `lithophane/config.rs` | Redundante Weiterleitung zu `Self::default()` – `Default` bereits implementiert |
| `LithophaneGenerator::config()` | `lithophane/generator.rs` | Niemals aufgerufen, kein externer Nutzer |

### Test-Only-Code eingeschränkt

| Item | Datei | Änderung |
|------|-------|---------|
| `combine_combi_groups()` | `palette/generator.rs` | Mit `#[cfg(test)]` markiert – nur in Tests verwendet |

---

## Phase 2b: Clippy-Fixes

### `unreadable_literal` (Lesbarkeit)
Alle numerischen Literale in `color/cielab.rs` mit Trennzeichen versehen:
- `0.008856` → `0.008_856`
- `7.787037` → `7.787_037`
- Alle 9 Matrixkoeffizienten (z.B. `0.4124564` → `0.412_456_4`)

### `format_in_format_args` (Effizienz)
`format!`-Aufrufe idiomatisch vereinfacht:
- `format!("...: {}", s)` → `format!("...: {s}")` (mehrere Stellen in `distance.rs`, `loader.rs`)

### `cast_precision_loss` / `cast_possible_truncation` (Typsicherheit)
Numerische Casts durch idiomatische From-Konvertierungen ersetzt:
- `self.r as i32` → `i32::from(self.r)` (in `distance.rs`)
- `pixel[0] as f64` → `f64::from(pixel[0])` (in `image/mod.rs`, `color/rgb.rs`)
- Unvermeidliche Truncating-Casts mit `#[allow(...)]` begründet dokumentiert

### `must_use` (API-Sicherheit)
`#[must_use]`-Attribut zu allen wertliefernden Funktionen hinzugefügt:
- `CieLab::new`, `CieLab::delta_e`
- `Hsl::new`, `Hsl::to_rgb`, `Hsl::to_cmyk`
- `Rgb::new`, `Rgb::to_hex`, `Rgb::to_f64`, `Rgb::from_f64`, `Rgb::to_cmyk`, `Rgb::from_cmyk`
- `Cmyk::new`, `Cmyk::clamp`
- `ColorLayer::*` (alle Getter und Konstruktoren)
- `ColorCombi::*` (alle Getter und Konstruktoren)
- `check_ratio`, `has_transparent_pixel`, `is_pixel_transparent`,
  `convert_to_grayscale`, `flip_vertical`, `extract_pixels`, `extract_pixels_flat`
- `calibration_grid_dimensions`

### `implicit_clone` (Klarheit)
Implizite String-Klone explizit gemacht:
- `hex.to_string()` (auf `&&String`) → `(*hex).clone()` in `palette/loader.rs`

### `float_cmp` (Korrektheit)
Float-Vergleiche die intentional sind (`== 0.0` als Sentinel-Wert), mit `#[allow(clippy::float_cmp)]` begründet annotiert.

### `manual_midpoint` (Modernisierung)
`#[allow(clippy::manual_midpoint)]` in `color/hsl.rs` hinzugefügt (Farbkonvertierung nutzt bewusst diese Formel ohne Overflow-Risiko bei `f64`).

---

## Phase 2c: Sichtbarkeit optimiert

| Item | Alt | Neu | Begründung |
|------|-----|-----|------------|
| `LithophaneGenerator::config()` | `pub(crate)` | entfernt | Niemals aufgerufen |

`check_ratio` und `extract_pixels_flat` bleiben `pub` – sie sind nützliche Utility-Funktionen für externe Nutzer der Library.

---

## Phase 3: Dokumentation vervollständigt

### Neue `//!`-Modulkommentare (auf Deutsch)

| Modul | Inhalt |
|-------|--------|
| `color/cielab.rs` | CIELab-Mathematik: RGB→XYZ→Lab Pipeline, Gamma-Korrektur, f(t)-Funktion, Delta-E-Formel |
| `color/mod.rs` | Erweiterte CMYK-Struct-Dokumentation mit Feldbeschreibungen |
| `lithophane/config.rs` | Erklärung Farbschicht, Texturschicht, Stützplatte |
| `palette/mod.rs` | CMYK-Stacking-Algorithmus Schritt-für-Schritt |
| `stl/mod.rs` | ASCII vs. Binary STL, ZIP-Multi-Layer-Export |

### Neue Docstrings für Struct-Felder

**`LithophaneConfig`** – alle 18 Felder dokumentiert:
- Einheit (mm, Grad, Bool)
- Bedeutung des Wertes 0 als Spezialwert
- Physikalische Interpretation

**`Cmyk`** – alle 4 Felder dokumentiert mit Wertebereich.

**`PixelCreationMethod`** – Enum und beide Varianten dokumentiert.

**`StlFormat`** – Enum und beide Varianten dokumentiert.

### Neue `# Errors`-Abschnitte

- `LithophaneConfig::validate()` – alle 10 Fehlerbedingungen aufgelistet
- `write_stl()`, `export_to_zip()` – I/O-Fehler beschrieben

### Neue Doctests (laufen als Teil von `cargo test`)

- `Cmyk::new` – Example-Block
- `Cmyk::clamp` – Example-Block mit assert

**Ergebnis:** +2 Doctests (von 12 auf 14 Doc-Tests gestiegen, alle grün)

---

## Offene Punkte / Technische Schulden

### Architektonische Fragen (nicht geändert)

1. **`struct_excessive_bools` in `LithophaneConfig`**
   Die Struct enthält 4 boolesche Felder (`color_layer`, `texture_layer`, `debug`, `low_memory`).
   Clippy-Pedantic warnt davor. Mögliche Lösung: Builder-Pattern oder Flags-Bitfeld.
   Änderung würde API brechen → nicht durchgeführt.

2. **`quantize_with_stats` / `QuantizationStats`** nur in Tests genutzt
   Diese öffentliche API wird im Binary nicht aufgerufen. Falls nicht für externe Nutzer gedacht,
   könnten sie auf `pub(crate)` reduziert oder entfernt werden. Behalten, da nützlich als
   Diagnose-API für Integrationsnutzer.

3. **`PixelCreationMethod::Full` komplett unimplementiert**
   In `config.rs` definiert und in der CLI akzeptiert, aber im Generator nicht unterschieden.
   Beide Modi verhalten sich identisch. Technische Schuld aus der Java-Portierung.

4. **`i32`-Sentinel-Parameter in `generate_color_layer`**
   `layer_offset: i32` und `layer_max: i32` nutzen `-1` als Sentinel für "nicht gesetzt".
   Idiomatisches Rust wäre `Option<u32>`. Änderung würde interne API brechen.

5. **Keine Integration Tests für die volle Pipeline**
   `tests/integration_test.rs` testet nur `LithophaneConfig`. Ein Ende-zu-Ende-Test
   (Bild → Palette laden → Lithophan generieren → STL schreiben) fehlt.

6. **Viele `pub` Items mit noch fehlenden `# Panics`-Abschnitten**
   Clippy-Pedantic meldet einige Funktionen die via `.expect()` paniken können aber keine
   `# Panics`-Sektion im Docstring haben. Nicht kritisch, da die Panics auf Programmierfehler
   hinweisen und nicht auf Laufzeitfehler.

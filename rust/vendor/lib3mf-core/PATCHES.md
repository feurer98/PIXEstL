# lib3mf-core Patches

Dieses Verzeichnis enthält eine gepatche Version von
[lib3mf-core](https://crates.io/crates/lib3mf-core) v0.4.0.

**Upstream:** https://github.com/sscargal/lib3mf-rs (Crate: `crates/lib3mf-core`)

## Warum Vendoring?

Die veröffentlichte Version auf crates.io enthält kritische Bugs im XML-Writer,
die den Multi-Color-3MF-Export für Slicer (Bambu Studio, PrusaSlicer, etc.)
vollständig unbrauchbar machen. Ohne diese Patches werden Farbzuweisungen
von allen getesteten Slicern ignoriert.

## Angewandte Patches

### 1. XML-Writer: ColorGroup-Namespace und pid/pindex-Attribute (kritisch)

**Datei:** `src/writer/model_writer.rs`

**Problem A – Fehlender XML-Namespace:**
```xml
<!-- Vorher (ungültig): -->
<colorgroup id="1"><color color="#FF0000FF"/></colorgroup>

<!-- Nachher (korrekt): -->
<m:colorgroup id="1"><m:color color="#FF0000FF"/></m:colorgroup>
```
Ohne `m:`-Prefix ignorieren Slicer die Farbdefinitionen komplett.

**Problem B – Fehlende Object-Attribute:**
```xml
<!-- Vorher: -->
<object id="2" type="model">

<!-- Nachher: -->
<object id="2" type="model" pid="1" pindex="0">
```
Ohne `pid`/`pindex` können Slicer Objekte keiner Farbgruppe zuordnen.

**Auswirkung:** Ohne diesen Patch ist Multi-Color-3MF-Export nicht funktional.

### 2. glam-Version auf 0.32 angehoben

**Datei:** `Cargo.toml`

Upstream pinnt `glam = "0.31"`, PIXEstL verwendet `0.32`. Bei pre-1.0 Semver
sind `0.31.x` und `0.32.x` inkompatibel, was zu Typ-Konflikten bei
`BuildItem.transform` (`Mat4`) führt.

### 3. Default-Impl für BuildItem

**Datei:** `src/model/mod.rs` (oder äquivalent)

`BuildItem` erhielt eine `Default`-Implementierung mit
`transform: Mat4::IDENTITY`, um die direkte `glam`-Abhängigkeit im
Konsumenten-Code zu eliminieren.

## Upstream-Status

Diese Patches wurden noch nicht als PR an den Upstream gesendet.

**Empfohlene nächste Schritte:**
1. PR an https://github.com/sscargal/lib3mf-rs mit Patch 1 (XML-Writer-Fix)
2. Nach Merge und Release: Vendor-Verzeichnis entfernen und
   `[patch.crates-io]` aus `Cargo.toml` löschen
3. Falls Upstream inaktiv: Fork als `lib3mf-core-pixestl` auf crates.io veröffentlichen

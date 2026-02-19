# Schnellstart

Dein erstes Ergebnis in unter 10 Minuten – wenn PIXEstL bereits installiert ist.

**Voraussetzungen auf einen Blick:** PIXEstL [installiert](installation/windows.md), ein Foto (JPG/PNG), Bambu Lab AMS mit transparenten CMYK-Filamenten.

---

## Schritt 1: Repository klonen und bauen

```bash
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust
cargo build --release
```

Dieser Befehl lädt den Quellcode herunter und erstellt das fertige Programm.
Die Kompilierung dauert beim ersten Mal 1–3 Minuten.

[Mehr Details zur Installation →](installation/windows.md)

---

## Schritt 2: Beispiel-Palette prüfen

Im Repository liegt bereits eine fertig kalibrierte Palette für Bambu Lab PLA Basic:

```bash
ls ../palette/
# filament-palette-0.10mm.json  ← diese verwenden wir
```

!!! tip "Eigene Filamente?"
    Die Beispiel-Palette ist auf Bambu Lab PLA Basic (Cyan, Magenta, Yellow, White) kalibriert.
    Für andere Filamente musst du zunächst [kalibrieren](anleitung/kalibrierung.md) – das lohnt sich!

---

## Schritt 3: Lithophanie generieren

```bash
./target/release/pixestl \
  -i mein_bild.jpg \
  -p ../palette/filament-palette-0.10mm.json \
  -o ausgabe.zip \
  -w 100
```

| Parameter | Bedeutung |
|-----------|-----------|
| `-i mein_bild.jpg` | Dein Eingabebild (JPG, PNG oder WebP) |
| `-p …palette.json` | Die Palette mit deinen Filamentfarben |
| `-o ausgabe.zip` | Die Ausgabedatei – ein ZIP-Archiv mit allen STL-Dateien |
| `-w 100` | Breite der Lithophanie in Millimetern |

Nach wenigen Sekunden liegt die Datei `ausgabe.zip` bereit.

[Alle Parameter erklärt →](anleitung/generierung.md)

---

## Schritt 4: ZIP entpacken

```bash
unzip ausgabe.zip -d ausgabe/
ls ausgabe/
```

Du siehst mehrere STL-Dateien – je eine pro Filamentfarbe:

```
ausgabe/
├── layer-plate.stl                     ← Grundplatte (Weiß)
├── layer-Cyan[PLA Basic].stl           ← Cyan-Farbschicht
├── layer-Magenta[PLA Basic].stl        ← Magenta-Farbschicht
├── layer-Yellow[PLA Basic].stl         ← Gelb-Farbschicht
├── layer-White[PLA Basic].stl          ← Weiß-Füllschicht
└── layer-texture-White[PLA Basic].stl  ← Helligkeitsschicht
```

---

## Schritt 5: In Bambu Studio laden und drucken

1. Öffne **Bambu Studio**
2. Gehe zu **File → Import** und wähle **alle** STL-Dateien aus dem `ausgabe/`-Ordner
3. Bestätige: *"Load as single object?"* → **Yes**
4. Weise jeder Schicht das richtige Filament zu (Dateiname verrät die Farbe)
5. **Infill auf 100%** setzen – das ist Pflicht!
6. Drucken starten

[Detaillierte Slicer-Anleitung →](anleitung/slicer.md)

---

## Ergebnis

Nach dem Druck die Lithophanie vor eine Lichtquelle halten – und staunen!

---

## Nächste Schritte

- [Palette kalibrieren](anleitung/kalibrierung.md) – Eigene Filamente einmessen für perfekte Farben
- [Bild vorbereiten](anleitung/bild-vorbereiten.md) – Welche Fotos funktionieren besonders gut?
- [Alle Parameter](referenz/parameter.md) – Was kann PIXEstL sonst noch?
- [Galerie](galerie.md) – Inspiration aus der Community

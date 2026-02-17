# Schnellstart

Dieses Tutorial zeigt dir in 5 Minuten, wie du deine erste Farb-Lithophanie generierst.

## Voraussetzungen

- [x] PIXEstL ist [installiert](installation.md)
- [x] Ein Eingabebild (JPG, PNG oder WebP)
- [x] Eine Palette-Datei (Beispiel liegt unter `palette/filament-palette-0.10mm.json`)

## Schritt 1: Lithophanie generieren

Fuehre folgenden Befehl aus:

```bash
pixestl \
  -i mein_bild.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o ausgabe.zip \
  -w 100
```

| Parameter | Bedeutung |
|-----------|-----------|
| `-i` | Eingabebild |
| `-p` | Palette-Datei (Filament-Farbdefinitionen) |
| `-o` | Ausgabe-Datei (ZIP mit STL-Dateien) |
| `-w 100` | Breite der Lithophanie: 100 mm |

## Schritt 2: Ausgabe pruefen

PIXEstL erzeugt eine ZIP-Datei mit mehreren STL-Dateien:

```
ausgabe.zip
├── layer-plate.stl                    # Grundplatte
├── layer-Cyan[PLA Basic].stl          # Cyan-Farbschicht
├── layer-Magenta[PLA Basic].stl       # Magenta-Farbschicht
├── layer-Yellow[PLA Basic].stl        # Gelb-Farbschicht
├── layer-White[PLA Basic].stl         # Weiss-Fuellschicht
└── layer-texture-White[PLA Basic].stl # Textur-/Helligkeitsschicht
```

## Schritt 3: In Slicer laden

1. Entpacke die ZIP-Datei
2. Oeffne Bambu Studio (oder deinen bevorzugten Slicer)
3. Importiere **alle** STL-Dateien gleichzeitig
4. Weise jeder Datei das passende Filament zu

!!! info "Detaillierte Anleitung"
    Fuer die vollstaendige Slicer-Einrichtung siehe [Bambu Studio Einrichtung](anleitung/slicer.md).

## Schritt 4: Drucken

- Stelle sicher, dass **Infill auf 100%** steht
- Verwende ein **0.10mm Druckprofil**
- Duesengroesse: **0.2mm** empfohlen

## Ergebnis

Nach dem Druck die Lithophanie vor eine Lichtquelle halten - und staunen!

---

## Naechste Schritte

- [Filament-Kalibrierung](anleitung/kalibrierung.md) - Eigene Palette erstellen fuer bessere Farben
- [CLI-Referenz](cli-referenz.md) - Alle Parameter und Optionen
- [Galerie](galerie.md) - Beispiele zur Inspiration

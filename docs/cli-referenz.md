# CLI-Referenz

Vollstaendige Referenz aller Kommandozeilen-Parameter von PIXEstL.

---

## Syntax

```bash
pixestl [OPTIONEN] -i <DATEI> -p <DATEI> -o <DATEI>
```

---

## Pflichtparameter

Diese drei Parameter sind bei jedem Aufruf erforderlich:

| Parameter        | Kurzform | Beschreibung                                      |
|------------------|----------|---------------------------------------------------|
| `--input`        | `-i`     | Pfad zum Eingabebild (JPG, PNG, WebP)             |
| `--palette`      | `-p`     | Pfad zur Palette-Datei (JSON)                     |
| `--output`       | `-o`     | Pfad zur Ausgabe-Datei (ZIP mit STL-Dateien)      |

!!! example "Minimalbeispiel"
    ```bash
    pixestl -i foto.jpg -p palette/filament-palette-0.10mm.json -o ausgabe.zip
    ```

---

## Bildgroesse

Steuert die physische Groesse der Lithophanie in Millimetern.

| Parameter    | Kurzform | Standard | Beschreibung                        |
|--------------|----------|----------|-------------------------------------|
| `--width`    | `-w`     | `0`      | Breite der Lithophanie in mm        |
| `--height`   | `-h`     | `0`      | Hoehe der Lithophanie in mm         |

!!! note "Automatische Groesse"
    Der Wert `0` bedeutet **automatisch**: Die jeweilige Dimension wird proportional zur anderen berechnet. Wird nur `--width 100` angegeben, ergibt sich die Hoehe automatisch aus dem Seitenverhaeltnis des Bildes.

!!! example "Beispiele"
    ```bash
    # Breite 100mm, Hoehe automatisch (proportional)
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100

    # Hoehe 80mm, Breite automatisch (proportional)
    pixestl -i foto.jpg -p palette.json -o out.zip -h 80

    # Exakte Groesse 100x80mm (kann verzerren)
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 -h 80
    ```

---

## Farbschicht-Einstellungen

Parameter fuer die CMYK-Farbschichten.

| Parameter                 | Standard | Beschreibung                                       |
|---------------------------|----------|----------------------------------------------------|
| `--color-pixel-width`     | `0.8`    | Breite eines Farbpixels in mm                      |
| `--color-layer-thickness` | `0.1`    | Dicke einer einzelnen Farbschicht in mm             |
| `--color-layers`          | `5`      | Anzahl der Farbschichten pro Pixel                  |
| `--no-color`              | -        | Farbschichten deaktivieren (nur Textur generieren)  |

!!! info "Pixelbreite und Aufloesung"
    Die `--color-pixel-width` bestimmt die Aufloesung der Farbschicht. Kleinere Werte erzeugen mehr Details, aber auch groessere STL-Dateien und laengere Druckzeiten. Der Standardwert von 0.8mm bietet einen guten Kompromiss.

!!! info "Schichtdicke"
    Die `--color-layer-thickness` sollte mit der Schichthoehe im Slicer uebereinstimmen. Fuer 0.10mm-Druckprofile den Standardwert `0.1` verwenden.

!!! example "Nur Graustufen (ohne Farbschicht)"
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --no-color
    ```

---

## Texturschicht-Einstellungen

Parameter fuer die Texturschicht (Helligkeitsrelief).

| Parameter               | Standard | Beschreibung                                        |
|-------------------------|----------|-----------------------------------------------------|
| `--texture-pixel-width` | `0.25`   | Breite eines Texturpixels in mm                     |
| `--texture-min`         | `0.3`    | Minimale Texturdicke in mm (hellster Punkt)          |
| `--texture-max`         | `1.8`    | Maximale Texturdicke in mm (dunkelster Punkt)        |
| `--no-texture`          | -        | Texturschicht deaktivieren (nur Farbe generieren)    |

!!! info "Texturschicht erklaert"
    Die Texturschicht erzeugt das klassische Lithophanie-Relief: Dunkle Bereiche sind dicker (weniger Lichtdurchlass), helle Bereiche duenner. Sie verleiht dem Bild **Tiefe und Kontrast**, unabhaengig von der Farbschicht.

!!! example "Nur Farbe (ohne Texturschicht)"
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --no-texture
    ```

---

## Export-Optionen

Parameter fuer die STL-Ausgabe.

| Parameter            | Standard | Beschreibung                                 |
|----------------------|----------|----------------------------------------------|
| `--format`           | `ascii`  | STL-Format: `ascii` oder `binary`            |
| `--plate-thickness`  | `0.2`    | Dicke der Grundplatte in mm                  |

!!! tip "Binaer fuer kleinere Dateien"
    Das binaere STL-Format erzeugt deutlich kleinere Dateien (ca. 50-80% kleiner als ASCII). Empfohlen fuer den regulaeren Einsatz:
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --format binary
    ```

---

## Erweiterte Optionen

Parameter fuer Feinsteuerung und Debugging.

| Parameter            | Standard   | Beschreibung                                          |
|----------------------|------------|-------------------------------------------------------|
| `--color-distance`   | `cie-lab`  | Farbdistanz-Methode: `rgb` oder `cie-lab`             |
| `--pixel-method`     | `additive` | Pixel-Methode: `additive` oder `full`                 |
| `--color-number`     | `0`        | Maximale Farbanzahl pro Gruppe (0 = alle)             |
| `--debug`            | -          | Debug-Ausgaben aktivieren                             |

### Farbdistanz-Methode

| Methode    | Beschreibung                                              |
|------------|-----------------------------------------------------------|
| `cie-lab`  | Wahrnehmungstreue Distanz im CIE-Lab-Farbraum (empfohlen) |
| `rgb`      | Euklidische Distanz im RGB-Farbraum (schneller)           |

### Pixel-Methode

| Methode     | Beschreibung                                             |
|-------------|----------------------------------------------------------|
| `additive`  | Verschiedene Farbschichten werden gestapelt (mehr Farben) |
| `full`      | Jeder Pixel verwendet nur eine Filamentfarbe (einfacher)  |

Siehe [CMYK Farbmischung](konzepte/farbmischung.md) fuer eine ausfuehrliche Erklaerung beider Methoden.

### Farbanzahl und AMS-Gruppen

Der Parameter `--color-number` steuert, wie viele Farben pro Druckgruppe verwendet werden:

| Wert | Bedeutung                                                   |
|------|-------------------------------------------------------------|
| `0`  | Alle verfuegbaren Farben verwenden (Standard)               |
| `4`  | Maximal 4 Farben pro Gruppe (1 AMS)                        |
| `8`  | Maximal 8 Farben pro Gruppe (2 AMS)                        |
| `16` | Maximal 16 Farben pro Gruppe (4 AMS)                       |

!!! example "Druck mit einem einzelnen AMS (4 Farben)"
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --color-number 4
    ```

---

## Vollstaendiges Beispiel

Ein typischer Aufruf mit den gaengigsten Optionen:

```bash
pixestl \
  -i urlaubsfoto.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o lithophanie.zip \
  -w 120 \
  --color-layers 5 \
  --color-pixel-width 0.8 \
  --texture-pixel-width 0.25 \
  --format binary \
  --color-distance cie-lab \
  --pixel-method additive
```

Dieser Befehl erzeugt eine 120mm breite Farb-Lithophanie mit 5 Farbschichten, CIE-Lab-Farbabgleich und binaerer STL-Ausgabe.

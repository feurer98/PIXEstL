# Alle Parameter – Referenz

Vollständige Übersicht aller Kommandozeilen-Parameter von PIXEstL.

---

## Aufruf-Syntax

```bash
# Farb-Lithophanie generieren
pixestl [OPTIONEN] -i <BILD> -p <PALETTE> -o <AUSGABE>

# Kalibrierungs-Testmuster generieren
pixestl --calibrate -p <PALETTE> -o <AUSGABE>

# Palette-Informationen anzeigen
pixestl --palette-info -p <PALETTE>
```

---

## Pflichtfelder

Diese Parameter müssen beim regulären Aufruf angegeben werden.

| Parameter | Kurzform | Beschreibung |
|-----------|----------|-------------|
| `--input` | `-i` | Pfad zum Eingabebild (JPG, PNG, WebP). Nicht nötig bei `--calibrate` oder `--palette-info`. |
| `--palette` | `-p` | Pfad zur Palette-Datei (JSON mit Filament-Farbdaten) |
| `--output` | `-o` | Pfad zur Ausgabe-ZIP-Datei (enthält alle STL-Dateien). Nicht nötig bei `--palette-info`. |

**Wann es Sinn macht zu ändern:** Diese Werte änderst du bei jedem Aufruf – sie sind immer bildspezifisch.

!!! example "Minimalbeispiel"
    ```bash
    pixestl -i foto.jpg -p palette/filament-palette-0.10mm.json -o ausgabe.zip
    ```

---

## Dimensionen

Steuern die physische Größe der fertigen Lithophanie in Millimetern.

| Parameter | Kurzform | Standard | Beschreibung |
|-----------|----------|---------|-------------|
| `--width` | `-w` | `0` | Breite der Lithophanie in mm. `0` = automatisch aus Höhe berechnen. |
| `--height` | `-H` | `0` | Höhe der Lithophanie in mm. `0` = automatisch aus Breite berechnen. |

**Wann es Sinn macht:** Gib mindestens einen Wert an. `0` bei beiden Werten führt zu einer Fehlermeldung.
Wenn nur Breite oder nur Höhe angegeben wird, berechnet PIXEstL die andere Dimension proportional aus dem Seitenverhältnis des Bildes.

!!! example "Beispiele"
    ```bash
    # 100 mm breit, Höhe proportional
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100

    # 80 mm hoch, Breite proportional
    pixestl -i foto.jpg -p palette.json -o out.zip -H 80

    # Exakt 120×80 mm (kann Bild verzerren)
    pixestl -i foto.jpg -p palette.json -o out.zip -w 120 -H 80
    ```

---

## Farb-Ebene

Parameter für die CMYK-Farbschichten – das Herzstück der Farb-Lithophanie.

| Parameter | Standard | Beschreibung |
|-----------|---------|-------------|
| `--color-pixel-width` | `0.8` mm | Breite eines einzelnen Farbpixels. Kleinere Werte = mehr Details, aber auch größere STL-Dateien und längere Druckzeiten. |
| `--color-layer-thickness` | `0.1` mm | Dicke einer einzelnen Farbschicht. Muss exakt mit der Schichthöhe im Slicer übereinstimmen. |
| `--color-layers` | `5` | Anzahl der Farbschichten pro Pixel (1–5). Mehr Schichten = mehr Farbtöne möglich. |
| `--no-color` | – (Flag) | Deaktiviert alle Farbschichten. Erzeugt eine klassische Graustufen-Lithophanie. |

**Wann es Sinn macht:**
- `--color-pixel-width 0.5`: Wenn du sehr feine Details willst und dir längere Druckzeiten egal sind.
- `--color-layers 3`: Für schnellere Drucke mit weniger Farbtiefe.
- `--no-color`: Für erste Tests oder wenn du kein AMS hast.

!!! warning "Schichtdicke und Kalibrierung"
    Der Wert `--color-layer-thickness` muss exakt der Schichthöhe entsprechen, für die deine Palette kalibriert wurde.
    Die mitgelieferte Beispiel-Palette ist für **0.10 mm** kalibriert. Wenn du 0.08 mm im Slicer verwendest, brauchst du eine neu kalibrierte Palette.

---

## Textur-Ebene

Parameter für die Texturschicht – sie kodiert die Helligkeit des Bildes als physische Dicke.

| Parameter | Standard | Beschreibung |
|-----------|---------|-------------|
| `--texture-pixel-width` | `0.25` mm | Auflösung der Texturschicht. Feiner als Farbpixel, um Helligkeitsverläufe detailliert abzubilden. |
| `--texture-min` | `0.3` mm | Minimale Texturdicke (hellste Bildstelle). Dünner = mehr Licht durch helle Bereiche. |
| `--texture-max` | `1.8` mm | Maximale Texturdicke (dunkelste Bildstelle). Dicker = weniger Licht durch dunkle Bereiche. |
| `--no-texture` | – (Flag) | Deaktiviert die Texturschicht. Nur Farbschichten werden erzeugt (für Tests). |

**Wann es Sinn macht:**
- `--texture-min 0.2` + `--texture-max 2.2`: Für mehr Kontrast bei sehr flachen Bildern.
- `--texture-pixel-width 0.2`: Für feinere Helligkeitsdetails bei größeren Lithophanien.
- `--no-texture`: Selten sinnvoll – Texturschicht gibt der Lithophanie Tiefe und Kontrast.

---

## Export

Parameter für das Ausgabeformat der STL-Dateien.

| Parameter | Standard | Beschreibung |
|-----------|---------|-------------|
| `--format` | `ascii` | STL-Format: `ascii` (lesbar) oder `binary` (klein, schnell). |
| `--plate-thickness` | `0.2` mm | Dicke der Grundplatte, auf der alle Farbschichten aufbauen. |

**Wann es Sinn macht:**
- `--format binary`: Fast immer empfehlenswert – erzeugt 50–80 % kleinere Dateien, die schneller vom Slicer geladen werden.
- `--plate-thickness 0.4`: Wenn du eine stabilere Grundplatte möchtest.

!!! tip "Binary immer verwenden"
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --format binary
    ```

---

## Krümmung (Curve)

Erzeugt zylindrisch gekrümmte Lithophanien statt flacher Platten – ideal für Lampenschirme und Panoramabilder.

| Parameter | Kurzform | Standard | Beschreibung |
|-----------|----------|---------|-------------|
| `--curve` | `-C` | `0` | Krümmungswinkel in Grad. 0 = flach, 90 = Viertelzylinder, 180 = Halbzylinder, 360 = Vollzylinder. |

| Wert | Ergebnis |
|------|---------|
| `0` | Flach (Standard) |
| `90` | Viertelzylinder |
| `180` | Halbzylinder – steht frei aufgestellt |
| `360` | Vollzylinder – ideal für Lampen und Vasen |

**Wann es Sinn macht:**
- `180`: Panoramafotos als freistehendes Leuchtbild.
- `360`: Als Lampenschirm – das Bild wickelt sich einmal komplett um.

!!! example "Beispiele"
    ```bash
    # Halbzylinder
    pixestl -i panorama.jpg -p palette.json -o out.zip -w 200 -C 180

    # Vollzylinder-Lampenschirm
    pixestl -i bild.jpg -p palette.json -o out.zip -w 250 -C 360
    ```

---

## Erweitert

Parameter für Feinsteuerung, AMS-Betrieb und Debugging.

| Parameter | Standard | Beschreibung |
|-----------|---------|-------------|
| `--color-distance` | `cie-lab` | Methode für den Farbvergleich: `cie-lab` (wahrnehmungstreu, empfohlen) oder `rgb` (schneller, weniger genau). |
| `--pixel-method` | `additive` | Wie Pixel erzeugt werden: `additive` (transparente Schichten stapeln) oder `full` (ein Pixel = eine Farbe). |
| `--color-number` | `0` | Maximale Farbanzahl pro Druckgruppe für AMS. `0` = alle Farben, `4` = 1 AMS, `8` = 2 AMS, `16` = 4 AMS. |
| `--calibrate` | – (Flag) | Kalibrierungsmodus: Erzeugt Testmuster statt Lithophanie. Kein Eingabebild nötig. |
| `--palette-info` | – (Flag) | Zeigt Informationen über die geladene Palette an (aktive Farben, Layer-Definitionen). |
| `--debug` | – (Flag) | Gibt zusätzliche Debug-Informationen in der Konsole aus. |

**Wann es Sinn macht:**
- `--color-number 4`: Wenn du nur ein AMS (4 Slots) hast – PIXEstL teilt die Farben in Gruppen auf.
- `--pixel-method full`: Für einfache Logos oder Grafiken ohne Farbmischung.
- `--color-distance rgb`: Selten – nur wenn CIE-Lab für eine bestimmte Palette schlechtere Ergebnisse liefert.

| AMS-Konfiguration | Parameter |
|-------------------|----------|
| 1 AMS (4 Slots) | `--color-number 4` |
| 2 AMS (8 Slots) | `--color-number 8` |
| 4 AMS (16 Slots) | `--color-number 16` |
| Alles auf einmal | `--color-number 0` (Standard) |

---

## Vollständige Beispiele

### Standard-Lithophanie (100 mm)

```bash
pixestl \
  -i foto.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o ausgabe.zip \
  -w 100 \
  --format binary
```

### Hochauflösend mit feinen Pixeln

```bash
pixestl \
  -i foto.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o ausgabe_fein.zip \
  -w 150 \
  --color-pixel-width 0.5 \
  --texture-pixel-width 0.2 \
  --format binary
```

### Nur Textur – klassische Graustufen-Lithophanie

```bash
pixestl \
  -i foto.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o graustufen.zip \
  -w 100 \
  --no-color
```

### Halbzylinder für Panoramafotos

```bash
pixestl \
  -i panorama.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o panorama.zip \
  -w 200 \
  -C 180 \
  --format binary
```

### Kalibrierungs-Testmuster

```bash
pixestl --calibrate \
  -p palette/filament-palette-0.10mm.json \
  -o kalibrierung.zip \
  --color-layers 5
```

---

## Nächster Schritt

- [Palette-Format →](palette-format.md) – Wie die Palette-JSON aufgebaut ist
- [Anleitung: Generierung →](../anleitung/generierung.md) – Parameter in der Praxis anwenden

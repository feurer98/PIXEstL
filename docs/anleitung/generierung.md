# Lithophanie generieren

Diese Seite erklaert, wie du mit PIXEstL eine Farb-Lithophanie aus einem Bild erzeugst. Du erfaehrst, welche Parameter zur Verfuegung stehen, wie du sie kombinierst und was die Ausgabe enthaelt.

---

## Grundbefehl

Der minimale Befehl fuer eine Lithophanie-Generierung lautet:

```bash
pixestl -i bild.jpg -p palette.json -o ausgabe.zip -w 100
```

| Parameter | Bedeutung |
|-----------|-----------|
| `-i bild.jpg` | Eingabebild (JPG, PNG oder WebP) |
| `-p palette.json` | Palette-Datei mit den kalibrierten Filamentfarben |
| `-o ausgabe.zip` | Ausgabe-Datei (ZIP-Archiv mit STL-Dateien) |
| `-w 100` | Breite der Lithophanie in Millimetern |

Damit generiert PIXEstL eine 100 mm breite Farb-Lithophanie mit den Standardeinstellungen.

---

## Bildauswahl

Die Qualitaet des Eingabebildes beeinflusst das Ergebnis erheblich. Beachte folgende Hinweise:

!!! tip "Geeignete Bilder"
    - **Hoher Kontrast:** Bilder mit deutlichen Hell-Dunkel-Unterschieden erzeugen die eindrucksvollsten Lithophanien.
    - **Gute Aufloesung:** 500 bis 2000 Pixel Breite sind ideal. Kleinere Bilder verlieren Details, groessere erhoehen die Rechenzeit ohne sichtbaren Gewinn.
    - **Portraits und Landschaften** funktionieren besonders gut, da sie natuerliche Helligkeitsverlaeufe enthalten.
    - **Farbsaettigung:** Kraeftige Farben im Eingabebild ergeben kraeftigere Farben in der Lithophanie.

!!! warning "Ungeeignete Bilder"
    - **Sehr dunkle Bilder:** Wenig Licht scheint durch -- das Ergebnis wirkt flach.
    - **Geringe Aufloesung (unter 300px):** Zu wenig Detail fuer eine scharfe Lithophanie.
    - **Stark komprimierte JPEGs:** Kompressionsartefakte werden in der Lithophanie sichtbar.
    - **Bilder mit wenig Kontrast:** Die Texturschicht kann kaum Helligkeitsunterschiede darstellen.

---

## Wichtige Parameter

### Groesse

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `-w` / `--width` | 0 | Breite der Lithophanie in mm |
| `-h` / `--height` | 0 | Hoehe der Lithophanie in mm |

Wenn nur `-w` oder nur `-h` angegeben wird, berechnet PIXEstL die andere Dimension automatisch, um das Seitenverhaeltnis des Eingabebildes beizubehalten. Typische Breiten liegen zwischen 80 und 150 mm.

```bash
# Breite 120mm, Hoehe wird automatisch berechnet
pixestl -i bild.jpg -p palette.json -o ausgabe.zip -w 120

# Hoehe 80mm, Breite wird automatisch berechnet
pixestl -i bild.jpg -p palette.json -o ausgabe.zip -h 80
```

### Farbpixel

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--color-pixel-width` | 0.8 mm | Breite eines einzelnen Farbpixels. Kleinere Werte ergeben feinere Details, aber groessere STL-Dateien und laengere Druckzeiten. |
| `--color-layers` | 5 | Anzahl der Farbschichten (1-5). Mehr Schichten ermoeglichen mehr Farbtiefe, erhoehen aber die Druckzeit. |
| `--color-layer-thickness` | 0.1 mm | Dicke pro Farbschicht. **Muss mit der Kalibrierung uebereinstimmen.** |

!!! warning "Schichtdicke und Kalibrierung"
    Der Wert von `--color-layer-thickness` muss exakt der Schichthoehe entsprechen, fuer die deine Palette kalibriert wurde. Die mitgelieferte Palette ist fuer 0.10mm kalibriert. Wenn du 0.08mm im Slicer verwendest, muss eine dafuer kalibrierte Palette erstellt werden.

### Textur

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--texture-pixel-width` | 0.25 mm | Aufloesung der Texturschicht. Feiner als die Farbpixel, um Helligkeitsverlaeufe detailliert abzubilden. |
| `--texture-min` | 0.3 mm | Minimale Texturdicke (hellste Stelle) |
| `--texture-max` | 1.8 mm | Maximale Texturdicke (dunkelste Stelle) |

Die Texturschicht codiert die Helligkeit des Bildes als Schichtdicke. Duenne Bereiche lassen mehr Licht durch (hell), dicke Bereiche weniger (dunkel). Der Bereich zwischen `--texture-min` und `--texture-max` bestimmt den Kontrastumfang.

### Ausgabeformat

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--format` | ascii | `ascii` oder `binary`. Binary erzeugt deutlich kleinere Dateien und ist schneller beim Export. |

```bash
# Binary-Format fuer kleinere Dateien
pixestl -i bild.jpg -p palette.json -o ausgabe.zip -w 100 --format binary
```

### Farbalgorithmus

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--color-distance` | cie-lab | Methode zur Farbabstandsberechnung. `cie-lab` ist perzeptuell genauer, `rgb` ist schneller. |
| `--pixel-method` | additive | `additive` = Farben werden durch uebereinanderliegende transparente Schichten gemischt. `full` = Jedes Pixel wird aus einem einzigen, vollfarbigen Block gedruckt. |

!!! info "CIE-Lab vs. RGB"
    CIE-Lab beruecksichtigt die menschliche Farbwahrnehmung und liefert in der Regel natuerlichere Ergebnisse. RGB ist mathematisch einfacher und kann in seltenen Faellen bei bestimmten Farbpaletten besser funktionieren. Im Zweifelsfall den Standard `cie-lab` verwenden.

### AMS-Modus

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--color-number` | 0 | Farbgruppierung fuer AMS. `0` = alle Farben in einer Gruppe. `4` = Farben werden in Gruppen zu je 4 aufgeteilt (passend fuer einen Bambu Lab AMS mit 4 Slots). |

Wenn du mehr Farben in der Palette hast, als dein AMS gleichzeitig aufnehmen kann, teilt `--color-number 4` die Farbschichten in mehrere Gruppen auf. Jede Gruppe kann als separater Druckauftrag nacheinander gedruckt werden.

### Kruemmung (Curve)

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `-C` / `--curve` | 0 | Kruemmungswinkel in Grad. 0 = flach (Standard), 90 = Viertelzylinder, 180 = Halbzylinder, 360 = Vollzylinder. |

Die Kruemmung wickelt die Lithophanie um einen Zylinder. Der Radius wird automatisch aus der Breite und dem Winkel berechnet, sodass die Bogenlaenge der eingestellten Breite entspricht.

```bash
# Halbzylinder, 150mm breit
pixestl -i panorama.jpg -p palette.json -o panorama.zip -w 150 -C 180

# Vollzylinder (Lampe), 250mm Umfang
pixestl -i bild.jpg -p palette.json -o lampe.zip -w 250 -C 360
```

!!! info "Wann Kruemmung verwenden?"
    Gekruemmte Lithophanien eignen sich besonders fuer:

    - **Panoramafotos** als freistehendes Halbzylinder-Bild
    - **Lampenschirme** als Vollzylinder (360 Grad)
    - **Bilderrahmen-Effekt** mit leichter Kruemmung (20-45 Grad)

### Weitere Parameter

| Parameter | Standard | Beschreibung |
|-----------|----------|--------------|
| `--plate-thickness` | 0.2 mm | Dicke der Grundplatte |
| `--no-color` | (Flag) | Erzeugt keine Farbschichten (nur Textur, Graustufenlithophanie) |
| `--no-texture` | (Flag) | Erzeugt keine Texturschicht (nur Farbe, flach) |
| `--calibrate` | (Flag) | Kalibrierungsmodus: Generiert Testmuster statt Lithophanie (kein Bild noetig) |
| `--debug` | (Flag) | Gibt zusaetzliche Debug-Informationen aus |

---

## Praxis-Beispiele

### 1. Standard-Portrait (100 mm breit)

```bash
pixestl \
  -i portrait.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o portrait.zip \
  -w 100
```

Erzeugt eine 100 mm breite Lithophanie mit allen Standardeinstellungen. Gut fuer einen ersten Test.

### 2. Hochaufloesend mit feinen Pixeln

```bash
pixestl \
  -i landschaft.png \
  -p palette/filament-palette-0.10mm.json \
  -o landschaft_fein.zip \
  -w 150 \
  --color-pixel-width 0.5 \
  --texture-pixel-width 0.2
```

Feinere Farbpixel (0.5 mm statt 0.8 mm) und feinere Textur (0.2 mm statt 0.25 mm) fuer mehr Detail. Die Dateigroesse und Rechenzeit steigen deutlich.

### 3. Binary-Format fuer kleinere Dateien

```bash
pixestl \
  -i foto.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o foto_binary.zip \
  -w 100 \
  --format binary
```

Binary-STL-Dateien sind typischerweise 5-10x kleiner als ASCII-STL-Dateien. Empfohlen fuer groessere Lithophanien.

### 4. AMS-Modus mit 4 Farben

```bash
pixestl \
  -i bild.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o bild_ams.zip \
  -w 100 \
  --color-number 4
```

Teilt die Farbschichten in Gruppen zu je 4 Farben auf. Jede Gruppe passt in einen Bambu Lab AMS-Slot-Satz und kann nacheinander gedruckt werden.

### 5. Nur Textur (Graustufen, ohne Farbe)

```bash
pixestl \
  -i bild.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o bild_grau.zip \
  -w 100 \
  --no-color
```

Erzeugt eine klassische Graustufen-Lithophanie ohne Farbschichten. Die Helligkeit wird ausschliesslich ueber die Texturdicke gesteuert. Benoetigt nur weisses Filament.

### 6. Gekruemmte Lithophanie (Halbzylinder)

```bash
pixestl \
  -i panorama.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o panorama_curved.zip \
  -w 200 \
  -C 180 \
  --format binary
```

Erzeugt eine 200mm breite Halbzylinder-Lithophanie. Die Kruemmung von 180 Grad wickelt das Bild um einen Halbkreis. Besonders geeignet fuer Panoramafotos, die als freistehendes Leuchtbild aufgestellt werden.

### 7. Kalibrierungs-Testmuster

```bash
pixestl --calibrate \
  -p palette/filament-palette-0.10mm.json \
  -o kalibrierung.zip \
  --color-layers 7
```

Generiert Kalibrierungs-Testfelder (10 x 10 mm) fuer alle aktiven Filamente der Palette. Jedes Filament erhaelt eine Reihe mit Quadraten bei aufsteigender Schichtanzahl (1 bis 7). Die Ausgabe enthaelt separate STL-Dateien pro Filament, sodass jedem das richtige Filament im Slicer zugewiesen werden kann.

---

## Ausgabe verstehen

PIXEstL erzeugt ein ZIP-Archiv mit mehreren STL-Dateien. Jede Datei entspricht einer separaten Druckschicht:

```
ausgabe.zip
+-- layer-plate.stl                    # Grundplatte
+-- layer-Cyan[PLA Basic].stl          # Cyan-Farbschicht
+-- layer-Magenta[PLA Basic].stl       # Magenta-Farbschicht
+-- layer-Yellow[PLA Basic].stl        # Gelb-Farbschicht
+-- layer-White[PLA Basic].stl         # Weiss-Fuellschicht
+-- layer-texture-White[PLA Basic].stl # Textur-/Helligkeitsschicht
```

| Datei | Funktion |
|-------|----------|
| `layer-plate.stl` | Die Grundplatte, auf der alle anderen Schichten aufbauen. Wird in der Farbe des Texturfilaments (typischerweise Weiss) gedruckt. |
| `layer-[Farbe].stl` | Eine Farbschicht. Enthaelt die Pixel, die diese Filamentfarbe verwenden. Pro aktiver Farbe in der Palette wird eine STL-Datei erzeugt. |
| `layer-texture-[Farbe].stl` | Die Texturschicht, die die Helligkeitsinformation traegt. Wird in Weiss (oder einer anderen hellen Farbe) gedruckt. |

![STL-Dateien](../assets/images/anleitung/sample_result.png)

!!! info "Alle Dateien importieren"
    Im Slicer muessen **alle** STL-Dateien gleichzeitig als ein Objekt importiert werden. Jede Datei wird dann einem eigenen Filament-Slot zugewiesen. Details zur Slicer-Einrichtung findest du unter [Slicer-Einrichtung](slicer.md).

---

## Leistungshinweise

Die Generierungszeit und der Speicherbedarf haengen von mehreren Faktoren ab:

| Faktor | Auswirkung |
|--------|-----------|
| **Bildgroesse** | Groessere Bilder erhoehen Rechenzeit und Speicherbedarf proportional. |
| **Farbpixelgroesse** | Kleinere Pixel (`--color-pixel-width`) erzeugen viel mehr Geometrie. Eine Halbierung der Pixelgroesse vervierfacht die Dreiecksanzahl. |
| **Texturaufloesung** | Kleinere Texturpixel (`--texture-pixel-width`) haben den gleichen Effekt wie kleinere Farbpixel. |
| **Ausgabebreite** | Groessere Lithophanien enthalten mehr Pixel und damit mehr Geometrie. |
| **Anzahl Farben** | Mehr aktive Farben erzeugen mehr separate STL-Dateien, aber die Gesamtgeometrie bleibt aehnlich. |

**Typische Richtwerte:**

| Szenario | Rechenzeit | RAM-Bedarf |
|----------|-----------|------------|
| 100 mm breit, Standardeinstellungen | 5-15 Sekunden | ~100 MB |
| 150 mm breit, feine Pixel (0.5 mm) | 30-60 Sekunden | ~200 MB |
| 200 mm breit, sehr fein (0.3 mm) | 2-5 Minuten | ~500 MB |
| 4K-Eingabebild, Standardeinstellungen | 15-30 Sekunden | ~500 MB |

!!! tip "Leistungsoptimierung"
    - Verwende `--format binary` fuer schnelleren Export und kleinere Dateien.
    - Reduziere die Bildaufloesung auf maximal 2000 Pixel Breite, bevor du es an PIXEstL uebergibst.
    - PIXEstL nutzt automatisch alle verfuegbaren CPU-Kerne fuer die parallele Verarbeitung.

---

## Naechste Schritte

- [Slicer-Einrichtung](slicer.md) -- STL-Dateien in Bambu Studio laden und konfigurieren
- [Drucken & Nachbearbeitung](druck.md) -- Druckeinstellungen und Tipps fuer das fertige Ergebnis
- [CLI-Referenz](../cli-referenz.md) -- Vollstaendige Liste aller Parameter

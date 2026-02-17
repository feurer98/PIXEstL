# Filament-Kalibrierung

Die Kalibrierung ist der wichtigste Schritt fuer eine farbgetreue Lithophanie. Ohne Kalibrierung kann PIXEstL nicht wissen, wie dein Filament tatsaechlich aussieht, wenn Licht hindurchscheint.

---

## Warum kalibrieren?

Jedes Filament verhaelt sich anders, wenn Licht hindurchscheint. Selbst Filamente gleicher Farbe unterscheiden sich je nach Hersteller, Material und Charge in ihrer Transparenz und Farbwiedergabe. Ein Cyan von Bambu Lab sieht hinterleuchtet voellig anders aus als ein Cyan von OVERTURE oder eSUN.

!!! warning "Ohne Kalibrierung keine guten Farben"
    Die mitgelieferte Beispiel-Palette (`palette/filament-palette-0.10mm.json`) ist auf bestimmte Bambu Lab PLA-Filamente kalibriert. Wenn du andere Filamente verwendest, **muessen** die HSL-Werte neu gemessen werden. Andernfalls weichen die Farben deiner Lithophanie stark vom Originalbild ab.

Die Kalibrierung erfasst die HSL-Werte (Farbton, Saettigung, Helligkeit) deiner Filamente bei verschiedenen Schichtdicken (1 bis 5 Schichten). PIXEstL nutzt diese Werte, um die optimale Farbmischung fuer jedes Pixel zu berechnen.

---

## Was wird benoetigt?

| Material / Werkzeug | Beschreibung |
|----------------------|--------------|
| **Transparente Filamente** | Mindestens Cyan, Magenta, Gelb und Weiss. Weitere Farben optional. |
| **3D-Drucker** | Mit 0.2mm-Duese und 0.10mm-Schichthoehe konfiguriert |
| **Lichtquelle** | LED-Panel oder Fenster mit Tageslicht (~6000K, gleichmaessig) |
| **Bildbearbeitungsprogramm** | Paint, GIMP, Photoshop oder aehnlich -- mit Pipetten-Werkzeug |
| **Web-Farbkonverter** | [convertacolor.com](https://convertacolor.com) oder ein vergleichbares Tool |

!!! tip "Empfehlung"
    Ein LED-Panel liefert die gleichmaessigsten Ergebnisse. Wenn du ein Fenster verwendest, waehle einen bewoelkten Tag, um direktes Sonnenlicht zu vermeiden.

---

## Schritt 1: Testmuster drucken

Drucke Farb-Teststreifen mit 1 bis 5 Schichten fuer jede Filamentfarbe. Jeder Streifen sollte gross genug sein, um spaeter eine saubere Farbmessung durchfuehren zu koennen (mindestens 10 x 10 mm pro Feld).

Die Kalibrierungs-STL kann entweder mit PIXEstL generiert oder manuell in einem CAD-Programm erstellt werden. Wichtig ist, dass jedes Feld eine exakte Anzahl von Schichten enthaelt:

| Feld | Schichten | Dicke bei 0.10mm |
|------|-----------|-------------------|
| 1    | 1 Schicht | 0.10 mm           |
| 2    | 2 Schichten | 0.20 mm         |
| 3    | 3 Schichten | 0.30 mm         |
| 4    | 4 Schichten | 0.40 mm         |
| 5    | 5 Schichten | 0.50 mm         |

![Kalibrierungsobjekt im Slicer](../assets/images/anleitung/slicer.png)

!!! info "Druckeinstellungen"
    Verwende exakt die gleiche Schichthoehe wie spaeter beim Lithophanie-Druck (Standard: 0.10mm). Setze Infill auf 100%, damit die Schichten vollstaendig gefuellt sind.

---

## Schritt 2: Testmuster fotografieren

Halte die gedruckten Teststreifen gegen eine gleichmaessige Hintergrundbeleuchtung und fotografiere sie. Die Beleuchtung muss konstant und gleichmaessig sein, damit die gemessenen Farbwerte reproduzierbar bleiben.

**Wichtig bei der Aufnahme:**

- Verwende ein LED-Panel oder ein gleichmaessig helles Fenster (~6000K Farbtemperatur)
- Halte die Teststreifen direkt vor die Lichtquelle
- Vermeide Reflexionen und Schatten auf den Testfeldern
- Fotografiere alle Farben unter identischen Lichtbedingungen
- Verwende den manuellen Weissabgleich deiner Kamera, wenn moeglich

!!! warning "Konsistente Beleuchtung"
    Aendere die Beleuchtung zwischen den einzelnen Farben nicht. Wenn du die Position oder Helligkeit der Lichtquelle aenderst, werden die gemessenen Werte inkonsistent und die Farbwiedergabe leidet.

---

## Schritt 3: HSL-Werte messen

Oeffne das Foto in einem Bildbearbeitungsprogramm und bestimme die HSL-Werte fuer jedes Testfeld. Der folgende Workflow zeigt die fuenf Schritte anhand eines konkreten Beispiels:

![Kalibrierungsworkflow](../assets/images/anleitung/calibration.png)

Die fuenf nummerierten Schritte im Detail:

1. **Pipette auswaehlen** -- Waehle das Pipetten-Werkzeug (Eyedropper / Farbaufnahme) in deinem Bildbearbeitungsprogramm aus.
2. **Farbe aufnehmen** -- Klicke auf ein Farbfeld des gedruckten Teststreifens im Foto. Waehle die Mitte des Feldes, um Randeffekte zu vermeiden.
3. **Farbwerte ablesen** -- Oeffne den Farbwaehler (Color Picker), um die RGB-Werte der aufgenommenen Farbe zu sehen. Notiere den Hex-Code (z.B. `#4ABCE8`).
4. **Hex-Code umrechnen** -- Gib den Hex-Code auf [convertacolor.com](https://convertacolor.com) ein, um ihn in HSL umzurechnen.
5. **HSL-Werte notieren** -- Lies die drei HSL-Werte ab: H (Hue / Farbton, 0-360), S (Saturation / Saettigung, 0-100) und L (Lightness / Helligkeit, 0-100).

Wiederhole diesen Vorgang fuer jedes Testfeld (1-5 Schichten) und jede Filamentfarbe.

!!! tip "Genauere Messungen"
    Messe jeden Farbwert an mehreren Stellen innerhalb des Feldes und bilde den Mittelwert. Das gleicht Ungleichmaessigkeiten im Druck und in der Beleuchtung aus.

---

## Schritt 4: Palette-JSON erstellen

Trage die gemessenen HSL-Werte fuer jede Schichtanzahl (1-5) und jede Farbe in die Palette-JSON-Datei ein. Jeder Eintrag besteht aus einem Hex-Schluessel, einem Namen, einem Aktivierungsflag und den HSL-Werten fuer bis zu 5 Schichten.

Hier ein vollstaendiges Beispiel fuer Cyan:

```json
{
  "#0086D6": {
    "name": "Cyan[PLA Basic]",
    "active": true,
    "layers": {
      "1": {
        "H": 201.9,
        "S": 96.4,
        "L": 78.2
      },
      "2": {
        "H": 199.5,
        "S": 100,
        "L": 64.3
      },
      "3": {
        "H": 199.8,
        "S": 91.5,
        "L": 53.9
      },
      "4": {
        "H": 200.6,
        "S": 95.9,
        "L": 47.6
      },
      "5": {
        "H": 203.8,
        "S": 99.1,
        "L": 45.3
      }
    }
  }
}
```

**So liest du die Werte:**

- **Schicht 1** (duennste Schicht): Hellster Wert, hohe Lightness (L), da wenig Filament das Licht nur leicht faerbt.
- **Schicht 5** (dickste Schicht): Dunkelster/saetttigster Wert, niedrige Lightness (L), da mehr Filament mehr Licht absorbiert.
- **Hue (H)** bleibt typischerweise relativ konstant ueber alle Schichten.
- **Saturation (S)** steigt meist mit zunehmender Schichtanzahl.
- **Lightness (L)** sinkt mit zunehmender Schichtanzahl.

!!! info "Vollstaendige Palette"
    Eine vollstaendige Palette enthaelt Eintraege fuer alle verwendeten Filamentfarben, einschliesslich Weiss. Details zum Palette-Format findest du unter [Palette-Format](palette.md).

---

## Tipps fuer bessere Ergebnisse

- **Konsistente Hintergrundbeleuchtung verwenden** -- Das Ergebnis steht und faellt mit der Beleuchtung. Ein LED-Panel mit 6000K Farbtemperatur liefert die besten Resultate.
- **Mehrfach messen** -- Nimm jeden Farbwert an 3-5 verschiedenen Stellen des Testfeldes auf und bilde den Mittelwert. Dies reduziert Messfehler.
- **Im abgedunkelten Raum kalibrieren** -- Fremdlicht verfaelscht die Farbwerte. Schalte Deckenleuchten aus und verdunkle Fenster, soweit moeglich.
- **Testdrucke beschriften** -- Notiere auf jedem Teststreifen Filamentname, Hersteller und Schichthoehe. So kannst du spaeter nachvollziehen, welche Werte zu welchem Filament gehoeren.
- **Kamera-Weissabgleich fixieren** -- Verwende den manuellen Weissabgleich deiner Kamera, um Farbverschiebungen durch den Automatik-Modus zu vermeiden.
- **Gleiche Schichthoehe verwenden** -- Die Kalibrierung gilt nur fuer die Schichthoehe, mit der kalibriert wurde. Wenn du von 0.10mm auf 0.08mm wechselst, muss neu kalibriert werden.

!!! warning "Kalibrierung pro Schichthoehe"
    Die HSL-Werte aendern sich bei unterschiedlichen Schichthoehen. Eine bei 0.10mm kalibrierte Palette liefert bei 0.08mm oder 0.12mm ungenaue Ergebnisse. Kalibriere fuer jede Schichthoehe separat.

---

## Naechste Schritte

- [Palette-Format](palette.md) -- Detaillierte Beschreibung der JSON-Struktur
- [Lithophanie generieren](generierung.md) -- Die kalibrierte Palette verwenden

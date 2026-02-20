# Palette-Format

Die Palette-Datei ist das Herzstück der Farbkalibrierung. Sie sagt PIXEstL genau, wie deine Filamente hinterleuchtet aussehen – und das für verschiedene Schichtdicken.

---

## Wozu dient die Palette?

Transparente Filamente sehen hinterleuchtet je nach Schichtdicke unterschiedlich aus: Eine einzelne Schicht Cyan ist sehr hell und blass, fünf Schichten übereinander sind satt und dunkel.

PIXEstL speichert diese Unterschiede pro Filament und Schichtanzahl in der Palette-Datei und berechnet damit die optimale Mischung für jeden Pixel deines Bildes.

---

## JSON-Grundstruktur

Die Palette ist ein JSON-Objekt. Jeder Schlüssel ist ein Hex-Farbcode, der Wert enthält alle Informationen zum Filament:

```json
{
  "#0086D6": {
    "name": "Cyan[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 201.9, "S": 96.4, "L": 78.2 },
      "2": { "H": 199.5, "S": 100,  "L": 64.3 },
      "3": { "H": 199.8, "S": 91.5, "L": 53.9 },
      "4": { "H": 200.6, "S": 95.9, "L": 47.6 },
      "5": { "H": 203.8, "S": 99.1, "L": 45.3 }
    }
  }
}
```

---

## Felder erklärt

| Feld | Typ | Beschreibung |
|------|-----|-------------|
| **Hex-Key** (z.B. `"#0086D6"`) | String | Eindeutiger Bezeichner des Filaments im Format `#RRGGBB`. Wird im Slicer zur Farbanzeige verwendet. |
| `name` | String | Lesbarer Name des Filaments, z.B. `"Cyan[PLA Basic]"`. Wird für die Benennung der STL-Ausgabedateien verwendet. |
| `active` | Boolean | `true` = Filament wird bei der Generierung verwendet. `false` = Filament wird ignoriert (aber bleibt in der Datei). |
| `layers` | Objekt | Enthält die Schlüssel `"1"` bis `"5"` (oder mehr). Jeder Schlüssel enthält die gemessenen HSL-Werte für diese Schichtanzahl. |
| `H` | Zahl | **Hue** (Farbton) in Grad, Bereich 0–360. Rot ≈ 0, Grün ≈ 120, Blau ≈ 240. |
| `S` | Zahl | **Saturation** (Sättigung) in Prozent, Bereich 0–100. 0 = Grau, 100 = volle Farbe. |
| `L` | Zahl | **Lightness** (Helligkeit) in Prozent, Bereich 0–100. 0 = Schwarz, 50 = reine Farbe, 100 = Weiß. |

**So ändern sich die Werte mit der Schichtanzahl:**

- **Schicht 1** (dünnste): Sehr heller Wert, hohe Lightness (L) – wenig Filament filtert kaum Licht.
- **Schicht 5** (dickste): Dunkler/sättigster Wert, niedrige Lightness (L) – mehr Filament absorbiert mehr Licht.
- **Hue (H)** bleibt über alle Schichten ungefähr gleich.
- **Saturation (S)** steigt typischerweise mit der Schichtanzahl.
- **Lightness (L)** sinkt mit der Schichtanzahl.

---

## Minimal-Beispiel (Weiß + eine Farbe)

Das kleinstmögliche gültige Beispiel mit nur zwei Filamenten:

```json
{
  "#FFFFFF": {
    "name": "White[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 0, "S": 0, "L": 95 },
      "2": { "H": 0, "S": 0, "L": 95 },
      "3": { "H": 0, "S": 0, "L": 85 },
      "4": { "H": 0, "S": 0, "L": 85 },
      "5": { "H": 0, "S": 0, "L": 85 }
    }
  },
  "#0086D6": {
    "name": "Cyan[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 201.9, "S": 96.4, "L": 78.2 },
      "2": { "H": 199.5, "S": 100,  "L": 64.3 },
      "3": { "H": 199.8, "S": 91.5, "L": 53.9 },
      "4": { "H": 200.6, "S": 95.9, "L": 47.6 },
      "5": { "H": 203.8, "S": 99.1, "L": 45.3 }
    }
  }
}
```

!!! warning "Weiß ist Pflicht"
    Weiß muss immer in der Palette enthalten und `active: true` sein. Es dient als Füllmaterial für Pixel, die weniger als 5 Farbschichten benötigen. Ohne Weiß kann PIXEstL keine Farbmischung berechnen.

---

## Vollständiges Beispiel (CMYK-Set)

Eine vollständige CMYK-Palette mit den vier Grundfarben:

```json
{
  "#FFFFFF": {
    "name": "White[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 0,   "S": 0,    "L": 95   },
      "2": { "H": 0,   "S": 0,    "L": 95   },
      "3": { "H": 0,   "S": 0,    "L": 85   },
      "4": { "H": 0,   "S": 0,    "L": 85   },
      "5": { "H": 0,   "S": 0,    "L": 85   }
    }
  },
  "#0086D6": {
    "name": "Cyan[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 201.9, "S": 96.4, "L": 78.2 },
      "2": { "H": 199.5, "S": 100,  "L": 64.3 },
      "3": { "H": 199.8, "S": 91.5, "L": 53.9 },
      "4": { "H": 200.6, "S": 95.9, "L": 47.6 },
      "5": { "H": 203.8, "S": 99.1, "L": 45.3 }
    }
  },
  "#EC008C": {
    "name": "Magenta[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 322.0, "S": 97.0, "L": 77.5 },
      "2": { "H": 321.5, "S": 99.0, "L": 63.0 },
      "3": { "H": 322.0, "S": 95.0, "L": 52.0 },
      "4": { "H": 322.5, "S": 98.0, "L": 45.0 },
      "5": { "H": 323.0, "S": 100,  "L": 40.0 }
    }
  },
  "#FCE300": {
    "name": "Yellow[PLA Basic]",
    "active": true,
    "layers": {
      "1": { "H": 53.0, "S": 97.0, "L": 83.0 },
      "2": { "H": 52.5, "S": 98.0, "L": 72.0 },
      "3": { "H": 52.0, "S": 99.0, "L": 62.0 },
      "4": { "H": 51.5, "S": 100,  "L": 55.0 },
      "5": { "H": 51.0, "S": 100,  "L": 49.0 }
    }
  }
}
```

!!! info "Werte anpassen"
    Die Werte im Beispiel sind für Bambu Lab PLA Basic kalibriert. Für andere Filamente musst du deine eigenen Werte messen – die Abweichungen können erheblich sein.

---

## Alternativer Hex-String-Notation

Statt HSL-Werte einzeln anzugeben, kannst du auch direkt Hex-Farbcodes pro Schicht angeben:

```json
{
  "#FF5500": {
    "name": "Orange[Beispiel]",
    "active": true,
    "layers": {
      "1": "#FFD4B0",
      "2": "#FFB87A",
      "3": "#FF9C4A",
      "4": "#FF7B1A",
      "5": "#FF5500"
    }
  }
}
```

PIXEstL konvertiert diese Hex-Werte automatisch in HSL. Diese Notation ist praktisch, wenn du Farben direkt aus einem Farbwähler einfügst.

---

## Farben aktivieren und deaktivieren

Mit `"active": false` kannst du Farben vorübergehend ausschalten, ohne sie zu löschen:

```json
{
  "#FFE34F": {
    "name": "Pale Yellow[OVERTURE]",
    "active": false,
    "layers": { "..." }
  }
}
```

Sinnvoll wenn:
- Das Filament gerade nicht vorrätig ist
- Du testen möchtest, wie die Lithophanie ohne eine bestimmte Farbe aussieht
- Dein AMS weniger Slots hat, als die Palette Farben enthält

---

## Häufige Fehler im JSON

??? failure "JSON-Syntaxfehler: fehlendes Komma"
    ```json
    // FALSCH – fehlendes Komma nach dem ersten Eintrag
    {
      "#FFFFFF": { ... }
      "#0086D6": { ... }
    }

    // RICHTIG
    {
      "#FFFFFF": { ... },
      "#0086D6": { ... }
    }
    ```

??? failure "Letztes Komma (Trailing Comma)"
    ```json
    // FALSCH – JSON erlaubt kein Komma nach dem letzten Eintrag
    {
      "1": { "H": 200, "S": 90, "L": 70 },
      "2": { "H": 199, "S": 92, "L": 60 },  ← Komma hier ist ein Fehler
    }
    ```

??? failure "Falscher Hex-Key-Format"
    ```json
    // FALSCH – fehlende Raute oder Kleinbuchstaben
    "0086D6": { ... }
    "#0086d6": { ... }

    // RICHTIG – Raute + Großbuchstaben
    "#0086D6": { ... }
    ```

??? failure "Kein Weiß-Eintrag"
    PIXEstL gibt eine Fehlermeldung aus, wenn die Palette keinen aktiven Weiß-Eintrag enthält. Weiß muss immer vorhanden sein.

---

## Palette validieren

```bash
pixestl --palette-info -p deine-palette.json
```

Dieser Befehl zeigt alle aktiven Farben, die Anzahl der Layer-Definitionen und warnt bei fehlenden oder inkonsistenten Einträgen.

---

## Nächster Schritt

- [Palette kalibrieren →](../anleitung/kalibrierung.md) – Wie du eigene HSL-Werte misst
- [Alle Parameter →](parameter.md) – Vollständige CLI-Referenz

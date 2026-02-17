# Palette-Format

Die Palette-Datei definiert, wie jede Filamentfarbe bei unterschiedlichen Schichtdicken aussieht, wenn Licht hindurchscheint. PIXEstL nutzt diese Informationen, um die optimale Farbmischung fuer jedes Pixel der Lithophanie zu berechnen.

---

## Uebersicht

Die Palette ist eine JSON-Datei, in der jede Filamentfarbe als eigener Eintrag gespeichert ist. Jeder Eintrag enthaelt die gemessenen HSL-Werte (Farbton, Saettigung, Helligkeit) fuer 1 bis 5 uebereinanderliegende Schichten des Filaments. Dadurch kann PIXEstL berechnen, wie eine Farbe bei unterschiedlicher Dicke wirkt -- von einer einzelnen duennen Schicht bis zu fuenf gestapelten Schichten.

Das Projekt enthaelt eine vorkalibrierte Beispiel-Palette:

```
palette/filament-palette-0.10mm.json
```

Diese Datei ist auf Bambu Lab PLA Basic und PLA Matte Filamente bei 0.10mm Schichthoehe kalibriert.

---

## JSON-Struktur

Die Palette besteht aus einem JSON-Objekt, dessen Schluessel die Hex-Farbcodes der Filamente sind. Jeder Eintrag hat folgende Struktur:

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

Die Schichten sind von `"1"` (duennste, hellste Schicht) bis `"5"` (dickste, dunkelste/saettigste Schicht) nummeriert.

---

## Felder erklaert

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| **Hex-Key** | String | Eindeutiger Schluessel fuer die Farbe im Format `"#RRGGBB"`, z.B. `"#0086D6"`. Dient als Identifikator und wird im Slicer zur Filamentzuordnung verwendet. |
| **name** | String | Lesbarer Name des Filaments, z.B. `"Cyan[PLA Basic]"`. Wird fuer die Benennung der STL-Dateien in der Ausgabe verwendet. |
| **active** | Boolean | `true` = Farbe wird bei der Generierung beruecksichtigt. `false` = Farbe wird ignoriert. |
| **layers** | Object | Objekt mit den Schluesseln `"1"` bis `"5"`. Jeder Schluessel enthaelt ein Objekt mit den HSL-Werten fuer die entsprechende Schichtanzahl. |
| **H** | Zahl | Hue (Farbton) in Grad, Bereich 0-360. Rot = 0, Gruen = 120, Blau = 240. |
| **S** | Zahl | Saturation (Saettigung) in Prozent, Bereich 0-100. 0 = Grau, 100 = volle Saettigung. |
| **L** | Zahl | Lightness (Helligkeit) in Prozent, Bereich 0-100. 0 = Schwarz, 50 = reine Farbe, 100 = Weiss. |

!!! info "Weniger als 5 Schichten"
    Nicht jede Farbe benoetigt alle 5 Schichten. Opake Farben wie Schwarz koennen mit weniger Schichten definiert werden. Beispielsweise hat Schwarz in der Beispiel-Palette nur die Schichten 4 und 5, da bei weniger Schichten kaum Schwarz sichtbar wird.

---

## Weiss-Filament

Weiss ist die einzige **zwingend erforderliche** Farbe in jeder Palette. Es dient als Fuellmaterial fuer die additive Farbmischung. An Stellen, wo ein Pixel weniger als 5 Farbschichten benoetigt, fuellt Weiss die verbleibenden Schichten auf.

!!! warning "Weiss ist Pflicht"
    Ohne einen Weiss-Eintrag in der Palette kann PIXEstL die Farbmischung nicht korrekt berechnen. Viele helle Farben und Pastelltoene sind ohne Weiss-Fuellschichten nicht darstellbar.

Beispiel-Eintrag fuer Weiss:

```json
{
  "#FFFFFF": {
    "name": "White[PLA Basic]",
    "active": true,
    "layers": {
      "1": {
        "H": 0,
        "S": 0,
        "L": 95
      },
      "2": {
        "H": 0,
        "S": 0,
        "L": 95
      },
      "3": {
        "H": 0,
        "S": 0,
        "L": 85
      },
      "4": {
        "H": 0,
        "S": 0,
        "L": 85
      },
      "5": {
        "H": 0,
        "S": 0,
        "L": 85
      }
    }
  }
}
```

Bei Weiss ist der Hue (H) und die Saettigung (S) jeweils 0. Nur die Helligkeit (L) variiert leicht je nach Schichtanzahl, da auch weisses Filament bei mehr Schichten etwas dunkler wird.

---

## Farben aktivieren und deaktivieren

Mit dem Feld `"active"` kannst du einzelne Farben ein- oder ausschalten, ohne sie aus der Palette zu loeschen:

```json
{
  "#FFE34F": {
    "name": "Pale yellow[OVERTURE]",
    "active": false,
    "layers": { ... }
  }
}
```

!!! tip "Wann deaktivieren?"
    - **Filament nicht vorraetig:** Du hast eine Farbe kalibriert, aber das Filament ist gerade leer.
    - **Testweise ausschliessen:** Du moechtest pruefen, wie die Lithophanie ohne eine bestimmte Farbe aussieht.
    - **AMS-Beschraenkung:** Dein AMS hat nur 4 Slots, aber deine Palette enthaelt 8 Farben. Deaktiviere die Farben, die nicht geladen sind.

Deaktivierte Farben bleiben in der Datei erhalten und koennen jederzeit durch Setzen von `"active": true` wieder verwendet werden.

---

## Eigene Palette erstellen

So erstellst du eine eigene Palette fuer deine Filamente:

1. **Beispiel-Palette kopieren:**

    ```bash
    cp palette/filament-palette-0.10mm.json palette/meine-palette.json
    ```

2. **Kalibrierung durchfuehren:** Drucke Teststreifen und messe die HSL-Werte wie unter [Filament-Kalibrierung](kalibrierung.md) beschrieben.

3. **Werte eintragen:** Ersetze die HSL-Werte in der kopierten Datei durch deine gemessenen Werte. Passe den `name` an, um deine Filamente zu kennzeichnen.

4. **Nicht vorhandene Farben deaktivieren:** Setze `"active": false` fuer alle Farben, die du nicht besitzt oder nicht kalibriert hast.

5. **Neue Farben hinzufuegen:** Fuege weitere Farbeintraege nach dem gleichen Schema hinzu. Waehle einen passenden Hex-Code als Schluessel.

6. **Palette verwenden:**

    ```bash
    pixestl -i bild.jpg -p palette/meine-palette.json -o ausgabe.zip -w 100
    ```

!!! tip "Palette benennen"
    Verwende die Schichthoehe im Dateinamen (z.B. `meine-palette-0.10mm.json`), um Verwechslungen zu vermeiden, wenn du Paletten fuer verschiedene Schichthoehen anlegst.

---

## Beispiel-Palette

Das Projekt enthaelt eine vollstaendige Beispiel-Palette unter:

```
palette/filament-palette-0.10mm.json
```

Diese Palette enthaelt Eintraege fuer die folgenden Filamente (aktive Farben):

| Hex-Code | Name | Aktiv |
|----------|------|-------|
| `#0086D6` | Cyan[PLA Basic] | Ja |
| `#EC008C` | Magenta[PLA Basic] | Ja |
| `#FCE300` | Yellow[PLA Basic] | Ja |
| `#FFFFFF` | White[PLA Basic] | Ja |
| `#000000` | Black[PLA Basic] | Ja |
| `#E7CEB5` | Beige[PLA Basic] | Ja |
| `#8BD5EE` | Matte Ice Blue[PLA Matte] | Ja |
| `#E4BDD0` | Matte Sakura Pink[PLA Matte] | Ja |

Zusaetzlich sind zahlreiche weitere Farben enthalten (z.B. Purple, Silver, Green, Orange), die jedoch auf `"active": false` gesetzt sind und erst nach Kalibrierung aktiviert werden koennen.

---

## Alternative Formate

Neben der ausfuehrlichen HSL-Notation koennen Schichten auch als Hex-Strings definiert werden. PIXEstL konvertiert diese automatisch in HSL-Werte:

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

In diesem Format wird jeder Schicht direkt ein Hex-Farbwert zugewiesen, anstatt die einzelnen HSL-Komponenten anzugeben. Die Hex-Strings werden beim Laden der Palette automatisch in H-, S- und L-Werte umgerechnet.

!!! info "HSL bevorzugt"
    Das HSL-Format bietet mehr Kontrolle und laesst sich leichter manuell anpassen. Die Hex-String-Variante ist praktisch, wenn du Farbwerte direkt aus einem Farbwaehler kopieren moechtest, ohne sie vorher umzurechnen.

---

!!! tip "Palette pruefen"
    Verwende `pixestl --palette-info -p deine-palette.json` um zu pruefen, welche Filamente aktiv sind, wie viele Farbkombinationen erzeugt werden und ob Layer-Definitionen fehlen.

---

## Naechste Schritte

- [Filament-Kalibrierung](kalibrierung.md) -- HSL-Werte fuer eigene Filamente messen
- [Best Practices](best-practices.md) -- Tipps und Tricks fuer bessere Ergebnisse
- [Lithophanie generieren](generierung.md) -- Die Palette im Generierungsprozess verwenden

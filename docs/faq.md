# FAQ & Fehlerbehebung

Haeufig gestellte Fragen und Loesungen fuer typische Probleme bei der Arbeit mit PIXEstL.

---

## Allgemeine Fragen

### Welche Filamente eignen sich?

Transparente oder transluzente Filamente aus **PLA** oder **PETG** sind am besten geeignet. Das Filament muss ausreichend lichtdurchlaessig sein, damit Licht durch 1 bis 5 gestapelte Schichten hindurchscheint.

!!! tip "Empfehlung"
    **Bambu Lab PLA Basic** (transparent) hat sich in der Praxis bewaehrt und liefert sehr gute Ergebnisse. Die Farben Cyan, Magenta, Yellow und White sind ideal fuer CMYK-Lithophanien.

| Eigenschaft       | Geeignet                          | Ungeeignet                      |
|--------------------|-----------------------------------|---------------------------------|
| Transparenz        | Transparent / Transluzent         | Opak / Deckend                  |
| Material           | PLA, PETG                         | ABS, TPU (zu wenig transparent) |
| Oberflaechenfinish | Glatt                             | Matt (streut Licht zu stark)    |

---

### Wie viele Farben kann ich verwenden?

Die Farbanzahl haengt von der Anzahl der verfuegbaren AMS-Einheiten (Automatic Material System) ab:

| AMS-Konfiguration | Maximale Farben | Parameter              |
|---------------------|----------------|------------------------|
| 1 AMS               | 4 Farben       | `--color-number 4`     |
| 2 AMS               | 8 Farben       | `--color-number 8`     |
| 4 AMS               | 16 Farben      | `--color-number 16`    |
| Unbegrenzt           | Alle           | `--color-number 0`     |

Mit `--color-number` werden die Farben automatisch in Gruppen aufgeteilt. Jede Gruppe kann separat gedruckt werden.

---

### Muss ich Weiss verwenden?

**Ja**, im additiven Modus ist Weiss **zwingend erforderlich**. Weiss dient als Fuellmaterial: Nicht alle Schichten eines Pixels muessen eingefaerbt sein. Die verbleibenden Schichten werden mit Weiss aufgefuellt, um eine gleichmaessige Gesamtdicke sicherzustellen.

!!! note "Ausnahme"
    Im Full-Pixel-Modus (`--pixel-method full`) ist Weiss nicht zwingend noetig, da jeder Pixel nur eine einzige Filamentfarbe verwendet. Allerdings ist die Farbvielfalt dann stark eingeschraenkt.

---

### Welcher Slicer wird unterstuetzt?

**Bambu Studio** ist der empfohlene Slicer und wurde am ausfuehrlichsten getestet. Weitere kompatible Slicer:

| Slicer          | Kompatibilitaet | Anmerkung                              |
|-----------------|-----------------|----------------------------------------|
| Bambu Studio    | Voll            | Empfohlen, beste AMS-Integration       |
| OrcaSlicer      | Voll            | Open-Source-Alternative zu Bambu Studio |
| PrusaSlicer     | Gut             | Fuer Prusa-Drucker mit MMU             |

Grundsaetzlich funktioniert PIXEstL mit jedem Slicer, der **Multi-Material-Druck** unterstuetzt und mehrere STL-Dateien als separate Objekte importieren kann.

---

### Kann ich ohne AMS drucken?

**Ja**, das ist moeglich. Ohne AMS muss der Filamentwechsel **manuell** erfolgen. Der Drucker pausiert an den konfigurierten Schichtwechseln, und das Filament wird von Hand gewechselt.

!!! warning "Aufwand"
    Manueller Filamentwechsel bei Farb-Lithophanien ist sehr zeitaufwaendig, da pro Zeile mehrere Wechsel noetig sein koennen. Fuer den Einstieg empfiehlt sich ein AMS oder alternativ der Modus `--no-color` fuer reine Graustufen-Lithophanien.

---

### Was bedeutet "additive" vs "full"?

Die beiden Pixel-Methoden unterscheiden sich grundlegend in der Art der Farberzeugung:

| Eigenschaft        | Additive (`--pixel-method additive`)        | Full (`--pixel-method full`)         |
|---------------------|---------------------------------------------|--------------------------------------|
| **Prinzip**         | Verschiedene Farbschichten werden gestapelt  | Jeder Pixel besteht aus einer Farbe  |
| **Farbvielfalt**    | Hoch (viele Mischfarben)                    | Gering (nur Grundfarben)             |
| **Weiss noetig**    | Ja (als Fuellmaterial)                       | Nein                                 |
| **Empfehlung**      | Fotorealistische Bilder                     | Einfache Grafiken, Logos             |

!!! tip "Standard"
    Die additive Methode ist der Standard und wird fuer die meisten Anwendungsfaelle empfohlen. Sie erzeugt aus wenigen Filamenten eine Vielzahl von Mischfarben.

Siehe auch: [CMYK Farbmischung](konzepte/farbmischung.md)

---

## Fehlerbehebung

### Die Farben stimmen nicht

Wenn die gedruckten Farben nicht mit dem Originalbild uebereinstimmen, pruefe folgende Punkte:

1. **Palette neu kalibrieren** - Die HSL-Werte in der Palette-Datei muessen exakt zu deinen Filamenten passen
2. **Konsistente Hintergrundbeleuchtung** - Verwende bei der Kalibrierung dieselbe Lichtquelle wie beim spaeteren Betrachten
3. **Korrekte HSL-Werte** - Messe die Filamentfarben mit einem Colorimeter oder gegen eine Referenz
4. **Filamentzuweisung im Slicer** - Stelle sicher, dass jede STL-Datei dem richtigen Filament zugewiesen ist

!!! warning "Haeufiger Fehler"
    Eine falsche Filamentzuweisung im Slicer ist die haeufigste Ursache fuer Farbfehler. Die Dateinamen enthalten die Filamentfarbe (z.B. `layer-Cyan[PLA Basic].stl`) - achte auf korrekte Zuordnung.

---

### Die STL-Dateien sind sehr gross

Grosse STL-Dateien koennen den Slicer verlangsamen. Zwei Massnahmen helfen:

1. **Binaeres Format verwenden**:
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip --format binary
    ```
    Binaere STL-Dateien sind ca. 50-80% kleiner als ASCII-Dateien.

2. **Pixelbreite erhoehen**:
    ```bash
    pixestl -i foto.jpg -p palette.json -o out.zip --color-pixel-width 1.0
    ```
    Groessere Pixel erzeugen weniger Dreiecke, allerdings auf Kosten der Detailgenauigkeit.

---

### Wie lange dauert die Generierung?

Die Generierungszeit haengt von Bildgroesse und Parametern ab:

| Bildgroesse      | Typische Zeit  | Anmerkung                        |
|-------------------|---------------|----------------------------------|
| Klein (< 1000px)  | 2-5 Sekunden  | Standardfall                     |
| Mittel (1000-2000px) | 5-15 Sekunden | 100mm Lithophanie             |
| Gross (2000-4000px) | 15-60 Sekunden | Detaillierte Lithophanien     |
| Sehr gross (> 4K)  | 1-5 Minuten   | Selten noetig                   |

!!! tip "Performance-Tipp"
    PIXEstL nutzt automatisch alle verfuegbaren CPU-Kerne. Auf Mehrkern-Prozessoren ist die Generierung deutlich schneller.

---

### Welche Bildgroesse ist optimal?

Fuer die meisten Lithophanien ist eine Bildgroesse von **500 bis 2000 Pixeln** auf der laengeren Seite optimal.

| Aspekt                   | Empfehlung                                      |
|--------------------------|--------------------------------------------------|
| Minimale Groesse          | 300px (fuer kleine Lithophanien bis 50mm)       |
| Optimale Groesse          | 500-2000px (bestes Verhaeltnis Qualitaet/Speed) |
| Maximale sinnvolle Groesse | 3000-4000px (darueber kaum Qualitaetsgewinn)   |

!!! note "Warum nicht groesser?"
    Bei einem 100mm breiten Druck mit 0.8mm Pixelbreite werden nur 125 Pixel in der Breite benoetigt. Ein 4000px breites Eingabebild wird auf diese 125 Pixel heruntergerechnet - die zusaetzlichen Pixel erzeugen nur laengere Verarbeitungszeiten, aber keinen sichtbaren Qualitaetsunterschied am fertigen Druck.

---

### Kann ich auch nur Graustufen drucken?

**Ja**, mit dem Parameter `--no-color` wird nur die Texturschicht generiert. Das Ergebnis ist eine klassische Graustufen-Lithophanie ohne Farbschichten.

```bash
pixestl -i foto.jpg -p palette.json -o out.zip -w 100 --no-color
```

Dies ist ideal fuer:

- Erste Versuche mit Lithophanien
- Drucke ohne AMS oder mit nur einem Filament
- Bilder, die auch in Graustufen gut wirken (z.B. Portraits, Landschaften)

!!! info "Umgekehrt"
    Mit `--no-texture` kann die Texturschicht weggelassen werden, sodass nur die Farbschichten generiert werden. Dies ist vor allem fuer Tests und Debugging nuetzlich.

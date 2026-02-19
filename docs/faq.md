# FAQ – Häufige Fragen

Antworten auf die häufigsten Fragen rund um PIXEstL, Kalibrierung, Farben und Druck.

---

## Farben und Qualität

### Meine Farben sehen falsch aus – was tun?

Das ist der häufigste Fehler und hat meist eine dieser Ursachen:

**1. Palette nicht oder schlecht kalibriert**
Die mitgelieferte Beispiel-Palette ist für Bambu Lab PLA Basic kalibriert. Für andere Filamente weichen die Farben stark ab.
→ [Kalibrierung neu durchführen](anleitung/kalibrierung.md)

**2. Falsche Filament-Zuweisung im Slicer**
Jede STL-Datei muss dem richtigen Filament-Slot zugewiesen sein. Der Dateiname verrät die Farbe, z.B. `layer-Cyan[...].stl` → Cyan-Slot.
→ [Slicer-Anleitung](anleitung/slicer.md)

**3. Inkonsistente Beleuchtung beim Kalibrieren**
Wenn beim Kalibrieren andere Lichtverhältnisse herrschten als beim späteren Betrachten, stimmen die Farben nicht.
→ Immer dieselbe Lichtquelle (idealerweise ein LED-Panel mit ~6000K) verwenden.

**4. Falsches Infill**
Infill muss auf 100% gesetzt sein. Bei weniger strömt Licht unkontrolliert durch die Hohlräume und verfälscht die Farbwiedergabe.

---

### Wie viele Farben kann ich verwenden?

Das hängt von deiner AMS-Konfiguration ab:

| AMS-Konfiguration | Max. gleichzeitige Farben | Parameter |
|-------------------|--------------------------|-----------|
| 1 AMS (4 Slots) | 4 Farben | `--color-number 4` |
| 2 AMS (8 Slots) | 8 Farben | `--color-number 8` |
| 4 AMS (16 Slots) | 16 Farben | `--color-number 16` |
| Theoretisch unbegrenzt | Alle Palette-Farben | `--color-number 0` |

Mit mehr Farben als AMS-Slots kannst du trotzdem drucken – PIXEstL teilt die Farben in Gruppen auf. Der Drucker pausiert dann zwischen den Gruppen für einen manuellen Filamentwechsel.

---

### Wie lange dauert die Generierung?

| Szenario | Typische Zeit |
|----------|-------------|
| 100 mm breit, Standardeinstellungen | 5–15 Sekunden |
| 150 mm breit, feine Pixel (0.5 mm) | 30–60 Sekunden |
| 200 mm breit, sehr fein (0.3 mm) | 2–5 Minuten |
| Sehr großes Eingabebild (> 4K) | 15–60 Sekunden |

PIXEstL nutzt automatisch alle CPU-Kerne – auf modernen Mehrkern-Prozessoren ist die Generierung deutlich schneller.

---

### Kann ich JPG statt PNG verwenden?

Ja, PIXEstL unterstützt JPG, PNG und WebP. JPG funktioniert gut, solange die Qualitätsstufe hoch ist.

Stark komprimierte JPEGs (erkennbar an sichtbarer Blockbildung) können zu sichtbaren Artefakten in der Lithophanie führen. Im Zweifel die PNG-Version des Bildes verwenden.

---

### Was ist der Unterschied zwischen Farb-Ebene und Textur-Ebene?

Eine Farb-Lithophanie besteht aus zwei übereinanderliegenden Schichten-Systemen:

| Ebene | Funktion | Steuert |
|-------|----------|---------|
| **Farb-Ebene** | Erzeugt die Farben durch Stapeln transparenter Filamente (CMYK-Prinzip) | Welche Farbe ein Pixel hat |
| **Textur-Ebene** | Erzeugt das klassische Lithophanie-Relief (dicker = dunkler) | Wie hell/dunkel ein Pixel erscheint |

Beide Ebenen zusammen ergeben eine vollständige Farb-Lithophanie. Mit `--no-color` erhältst du eine klassische Graustufen-Lithophanie (nur Textur), mit `--no-texture` nur eine flache Farbschicht (selten sinnvoll).

---

### Mein Slicer erkennt die Farben nicht richtig

Häufige Ursachen:

- **Dateien nicht als einzelnes Objekt geladen:** In Bambu Studio beim Import *"Load as single object?"* → **Yes** auswählen.
- **Falsche Filament-Slot-Zuweisung:** Im Objects Panel jeden Eintrag prüfen und das richtige Filament zuweisen.
- **Platte, Weiß und Textur auf demselben Slot:** `layer-plate.stl`, `layer-White[...].stl` und `layer-texture-[...].stl` müssen alle auf Slot 1 (Weiß).

[Detaillierte Slicer-Anleitung →](anleitung/slicer.md)

---

## Drucker und Hardware

### Funktioniert PIXEstL ohne AMS / mit einem anderen Drucker?

**Ja!** PIXEstL erzeugt Standard-STL-Dateien und ist drucker-unabhängig.

Ohne AMS musst du die Filamente **manuell** wechseln – zeitaufwändig, aber machbar:

1. Pausen an den richtigen Schichten im Slicer eintragen
2. Drucker pausieren lassen
3. Filament manuell wechseln und Druck fortsetzen

**Andere Slicer:** OrcaSlicer, PrusaSlicer und Cura funktionieren ebenfalls. Wichtig: Multi-Material-Druck muss unterstützt werden.

!!! tip "Ohne AMS – einfacher Einstieg"
    Verwende `--no-color`, um nur eine klassische Graustufen-Lithophanie zu erzeugen. Diese benötigt nur weißes Filament und keinen Farbwechsel.

---

### Welche Filamente eignen sich?

Transparente oder transluzente Filamente aus PLA oder PETG sind ideal.

| Eigenschaft | Geeignet | Weniger geeignet |
|-------------|---------|-----------------|
| Transparenz | Transparent / transluzent | Opak / deckend |
| Material | PLA, PETG | ABS, TPU |
| Oberfläche | Glatt (Standard PLA) | Matt (streut Licht zu stark) |

**Empfehlung:** Bambu Lab PLA Basic (Transparent) liefert sehr gute Ergebnisse.

---

### Welche Nozzle-Größe wird empfohlen?

Eine **0.2 mm Nozzle** liefert die besten Ergebnisse. Eine 0.4 mm Nozzle funktioniert ebenfalls, sollte aber mit gröberen Pixel-Einstellungen kombiniert werden (`--color-pixel-width 1.0`).

---

## Technische Fragen

### Was bedeutet "additive" vs. "full" Pixel-Methode?

| Methode | Prinzip | Wann verwenden? |
|---------|---------|----------------|
| `additive` (Standard) | Transparente Farbschichten werden gestapelt. Durch Mischung entstehen viele Farbtöne. | Fotos und Bilder mit Farbverläufen |
| `full` | Jeder Pixel besteht aus nur einer einzigen Filamentfarbe – keine Mischung. | Einfache Logos mit wenigen satten Farben |

---

### Muss ich Weiß als Filament haben?

Im additiven Modus (Standard) **ja** – Weiß ist Pflicht. Es füllt die Pixel-Schichten auf, die keine andere Farbe benötigen, und sorgt für gleichmäßige Dicke.

Im `full`-Modus ist Weiß optional (dann nur als weitere Farboption).

---

### Wie groß werden die STL-Dateien?

| Einstellung | Auswirkung auf Dateigröße |
|-------------|--------------------------|
| `--color-pixel-width` kleiner | Viel größere Dateien |
| `--texture-pixel-width` kleiner | Größere Textur-STL |
| `--format binary` | 50–80 % kleiner als ASCII |
| Lithophanie-Breite größer | Proportional mehr Geometrie |

Typische Gesamtgröße: 20–100 MB (ASCII), 5–20 MB (binary).

---

### Kann ich eine Lithophanie auch als Zylinder oder Lampenschirm drucken?

Ja! Mit dem Parameter `-C` (oder `--curve`) krümmst du die Lithophanie:

```bash
# Halbzylinder (steht frei aufgestellt)
pixestl -i foto.jpg -p palette.json -o out.zip -w 200 -C 180

# Vollzylinder (Lampenschirm)
pixestl -i foto.jpg -p palette.json -o out.zip -w 300 -C 360
```

---

### Warum ändert sich die Qualität nicht, wenn ich ein größeres Bild verwende?

Die physische Auflösung der Lithophanie ist durch `--color-pixel-width` begrenzt, nicht durch die Bildauflösung. Bei 100 mm Breite und 0.8 mm Pixelbreite gibt es nur 125 Pixel in der Breite – egal ob das Eingabebild 500 oder 4000 Pixel breit ist.

Größere Bilder sorgen nur für etwas besseres Antialiasing beim Herunterskalieren – ein 4000-Pixel-Bild ist hier kaum besser als ein 1000-Pixel-Bild.

**Empfehlung:** Bilder zwischen 500 und 2000 Pixeln Breite für optimale Rechenzeit.

---

### Warum brauche ich `--color-layer-thickness`? Kann ich das ignorieren?

Im Normalfall (0.10 mm Schichthöhe im Slicer) kannst du den Standardwert verwenden.

Wenn du eine andere Schichthöhe verwendest (z.B. 0.08 mm), **musst** du:
1. Eine neue Palette für diese Schichthöhe kalibrieren
2. `--color-layer-thickness 0.08` beim Aufruf setzen

Die Kalibrierungswerte gelten exakt nur für die gemessene Schichthöhe.

---

## Nächster Schritt

Noch Fragen? Schau in die [vollständige Anleitung →](anleitung/uebersicht.md) oder öffne ein [Issue auf GitHub](https://github.com/feurer98/PIXEstL/issues).

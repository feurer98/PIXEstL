# Best Practices fuer Farblithophanien

Diese Seite fasst die wichtigsten Tipps und Tricks zusammen, die aus der Community-Erfahrung mit PIXEstL hervorgegangen sind. Sie basiert auf Erkenntnissen aus zahlreichen Anwenderbeitraegen und Tests.

---

## Bildvorbereitung

Die Qualitaet des Eingabebildes hat grossen Einfluss auf das Endergebnis. Mit etwas Vorverarbeitung lassen sich deutlich bessere Lithophanien erzeugen.

### Kontrast und Helligkeit

- **Kontrast leicht erhoehen:** Transparente Filamente tendieren dazu, Farben auszuwaschen. Ein leicht erhoehter Kontrast im Quellbild kompensiert diesen Effekt.
- **Highlights zuruecknehmen:** Ueberbelichtete Bereiche (reine Weissflaechen) fuehren zu flachen, detaillosen Stellen in der Lithophanie. Reduziere die Highlights im Quellbild um 10-20%.
- **Schatten aufhellen:** Zu dunkle Bereiche werden in der Lithophanie undifferenziert schwarz. Helle die Schatten leicht auf, um Details zu erhalten.

### Bildgroesse und Aufloesung

- **Mindestens so viele Pixel wie Farbpixel:** PIXEstL rechnet das Bild auf die Farbpixel-Anzahl herunter. Bei `--width 100` und `--color-pixel-width 0.8` entstehen 125 Farbpixel. Ein Bild mit 125 Pixel Breite reicht theoretisch; 250-500 Pixel liefern durch das Lanczos3-Resampling bessere Ergebnisse.
- **Keine kuenstliche Hochskalierung:** Ein 200px-Bild auf 2000px hochzuskalieren bringt keine zusaetzlichen Details. Verwende das Originalbild in seiner natuerlichen Aufloesung.
- **Seitenverhaeltnis pruefen:** Wenn du `--width` und `--height` gleichzeitig angibst, achte darauf, dass das Seitenverhaeltnis zum Bild passt. PIXEstL warnt bei Abweichungen.

!!! tip "Aufloesung pruefen"
    PIXEstL zeigt beim Generieren einen Hinweis an, wenn das Bild deutlich mehr Pixel hat als die konfigurierte Farbaufloesung. In diesem Fall kannst du `--color-pixel-width` verkleinern, um die Schaerfe zu erhoehen (auf Kosten der Dateigrösse und Druckzeit).

### Motivwahl

- **Porträts:** Hauttöne sind mit nur 4 Farben (CMYK) schwierig. Beige- oder Hautfarben-Filamente verbessern Porträts erheblich. Die Beispiel-Palette enthaelt Beige aus diesem Grund.
- **Landschaften:** Funktionieren mit CMYK meist gut, da Gruen- und Blauantöne dominieren.
- **Hochkontrast-Motive:** Text, Logos und Grafiken mit klaren Farben eignen sich besonders gut.
- **Fotos mit vielen Grautönen:** Schwierig, da Grau aus CMYK gemischt wird und je nach Kalibrierung farbstichig wirken kann.

---

## Optimale Druckparameter

Die folgenden Werte sind Community-Konsens aus zahlreichen Tests.

### Schichthoehe und Farbschichten

| Parameter | Empfehlung | Erlaeuterung |
|-----------|------------|--------------|
| Schichthoehe | 0.08 - 0.12 mm | Kleinere Schichthoehen ergeben feinere Farbabstufungen. 0.10mm ist der Standard-Kompromiss. |
| Farbschichten (`--color-layers`) | 5 - 7 | 5 Schichten reichen fuer die meisten Anwendungen. 7 Schichten bei 0.07mm ergeben noch feinere Farbuebergaenge, brauchen aber eine eigene Palette-Kalibrierung. |
| Farbpixel-Groesse (`--color-pixel-width`) | 0.4 - 0.8 mm | 0.8mm ist schnell und robust. 0.4mm liefert doppelte Aufloesung, verdoppelt aber auch die Druckzeit. |

!!! warning "Kalibrierung und Schichthoehe"
    Die Palette ist immer fuer eine bestimmte Schichthoehe kalibriert. Wenn du von 0.10mm auf 0.07mm wechselst, brauchst du eine neue Kalibrierung. Verwende die Schichthoehe im Dateinamen der Palette (z.B. `palette-0.07mm.json`).

### Sieben Schichten (Community-Empfehlung)

Mehrere erfahrene Anwender empfehlen 7 Farbschichten bei 0.07mm Schichthoehe:

- **Vorteile:** Feinere Farbabstufungen, besseres Blending zwischen Farben, sattere Farben.
- **Nachteile:** Laengere Druckzeit, aufwaendigere Kalibrierung (7 statt 5 Testfelder pro Farbe), 9+ Schichten zeigen kaum noch Verbesserung (diminishing returns).
- **Tipp:** Starte mit 5 Schichten bei 0.10mm. Wenn die Ergebnisse zufriedenstellen, experimentiere mit 7 Schichten bei 0.07mm fuer noch bessere Qualitaet.

### Druckgeschwindigkeit und Ironing

- **Langsame Geschwindigkeit:** 30-50 mm/s fuer die Farbschichten verbessert die Farbgenauigkeit. Bei hohen Geschwindigkeiten kann die Extrusion ungleichmaessig werden.
- **Ironing aktivieren:** Glaettet die Oberflaeche jeder Schicht und eliminiert Druck-Artefakte. Dies ist besonders wichtig fuer die weisse Basisschicht.
- **Filament-Sequenzierung (BambuStudio):** Wenn verfuegbar, aktiviere "By Object" statt "By Layer". Dies reduziert unnoetige Filamentwechsel und verbessert die Druckqualitaet bei vielen Farben.

---

## Kalibrierung — Zusammenfassung

Die vollstaendige Kalibrierungsanleitung findest du unter [Filament-Kalibrierung](kalibrierung.md). Hier die wichtigsten Punkte:

### Die drei haeufigsten Fehler

1. **Nur einen Layer kalibriert:** Die Palette braucht HSL-Werte fuer **jeden** Layer (1 bis 5 oder 1 bis 7). Ein Eintrag mit nur Layer 5 funktioniert, verschenkt aber Farbgenauigkeit.

2. **Warme Beleuchtung verwendet:** Gluehbirnen und warme LEDs verschieben alle Farbwerte ins Gelbliche. Verwende ausschliesslich neutralweisse LED-Panels (~5500-6500K).

3. **Kamera im Automatik-Modus:** Der automatische Weissabgleich der Kamera aendert die Farbwerte je nach Umgebung. Verwende den manuellen Modus:
    - ISO: 50-125
    - Shutter: ~1/180
    - Weissabgleich: 4000-7000K (fixiert)

### Layer-Nummern verstehen

Die Layer-Nummern in der Palette (`"1"`, `"2"`, ..., `"5"`) sind **nicht** die physische Position im STL. Sie definieren die **Farbdichte** — also wie viele Schichten eines Filaments uebereinander gedruckt werden:

- **Layer "1":** Duennste Schicht → hellste, transparenteste Farbe
- **Layer "5":** Dickste Schicht → dunkelste, gesaettigtste Farbe

PIXEstL kombiniert verschiedene Filamente in verschiedenen Dichten, um die Zielfarbe jedes Pixels bestmoeglich zu approximieren. Zum Beispiel koennte ein Pixel aus "2 Schichten Cyan + 3 Schichten Weiss" bestehen, um ein helles Blau zu erzeugen.

!!! tip "Palette-Info nutzen"
    Verwende `pixestl --palette-info -p deine-palette.json` um zu pruefen, welche Filamente aktiv sind und ob alle Layer-Definitionen vollstaendig sind. PIXEstL zeigt Warnungen an, wenn Layer-Definitionen fehlen.

### Schnell-Check fuer Kalibrierungsergebnisse

Nach dem Kalibrieren solltest du folgende Werte pruefen:

- **Weiss:** H=0, S=0, L=90-100 (fast reines Weiss). Wenn L unter 85 liegt, ist die Beleuchtung zu schwach.
- **Schwarz:** K-Wert hoch (dunkel). Schwarz braucht oft nur Layer 4-5, da wenige Schichten keinen sichtbaren Schwarzanteil erzeugen.
- **Hue-Konsistenz:** Der Farbton (H) sollte ueber alle Layer relativ konstant bleiben. Wenn H bei Cyan von 200 auf 160 springt, stimmt etwas mit der Beleuchtung nicht.
- **Lightness-Verlauf:** L sollte von Layer 1 (hoch, hell) zu Layer 5 (niedrig, dunkel) monoton sinken.

---

## Fehlerbehebung

### "Farben sehen falsch aus"

1. **Palette pruefen:** Verwende `pixestl --palette-info -p palette.json --color-layers N` und pruefe die Warnungen.
2. **Beleuchtung pruefen:** Hinterleuchte die Lithophanie mit der gleichen Lichtquelle wie bei der Kalibrierung (oder gleichwertiger Farbtemperatur).
3. **Layer-Anzahl pruefen:** Wenn die Palette auf 5 Layer kalibriert ist, verwende `--color-layers 5`, nicht mehr.

### "Bild ist unscharf/blurry"

1. **Farbpixelgroesse pruefen:** Bei `--color-pixel-width 0.8` und `--width 80` entstehen nur 100 Farbpixel. Verkleinere die Pixelgroesse: `--color-pixel-width 0.4`.
2. **Quellbild pruefen:** Das Originalbild muss scharf sein. Ueberprüfe es in voller Aufloesung.
3. **PIXEstL-Hinweis beachten:** Bei der Generierung zeigt PIXEstL einen Hinweis an, wenn die effektive Aufloesung deutlich unter der Bildaufloesung liegt.

### "Vorschau korrekt, aber Druck falsch"

1. **Transmission beachten:** Die Vorschau zeigt die berechnete Farbe, aber im Druck transmittiert das Licht durch alle Schichten. Bei vielen Schichten (>5) kann die Lichtdurchlaessigkeit abnehmen.
2. **Filament-Reihenfolge pruefen:** Im Slicer die Filament-Zuordnung kontrollieren. Jede STL-Datei im ZIP ist nach dem zugehoerigen Filament benannt.
3. **AMS-Slots pruefen:** Bei mehr als 4 Farben muss die AMS-Gruppierung (`--color-number 4`) korrekt konfiguriert sein.

---

## Empfohlener Workflow

Fuer optimale Ergebnisse empfehlen wir folgenden Ablauf:

1. **Kalibrierungs-Testmuster generieren:** `pixestl --calibrate -p palette.json -o kalibrierung.zip`
2. **Testmuster drucken, fotografieren und HSL-Werte messen** (einmalig pro Filament-Set und Schichthoehe)
3. **Palette pruefen:** `pixestl --palette-info -p palette.json`
4. **Bild vorbereiten:** Kontrast erhoehen, Highlights reduzieren
5. **Testgenerierung:** Mit niedrigerer Aufloesung starten (`--color-pixel-width 0.8`)
6. **STL im Slicer pruefen:** Filamentzuordnung und Schichtreihenfolge kontrollieren
7. **Feintuning:** Bei Bedarf `--color-pixel-width` verkleinern oder `--color-layers` erhoehen
8. **Optional: Kruemmung hinzufuegen:** `-C 90` bis `-C 360` fuer zylindrische Formen
9. **Finaler Druck:** Mit Ironing, langsamer Geschwindigkeit, korrekter Schichthoehe

---

## Naechste Schritte

- [Filament-Kalibrierung](kalibrierung.md) — Detaillierte Kalibrierungsanleitung
- [Palette-Format](palette.md) — JSON-Struktur der Palette
- [Lithophanie generieren](generierung.md) — Generierungsprozess im Detail
- [Slicer-Einstellungen](slicer.md) — STL in BambuStudio importieren

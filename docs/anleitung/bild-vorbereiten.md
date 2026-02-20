# Bild vorbereiten

Bevor du PIXEstL startest, lohnt es sich, kurz über das Eingabebild nachzudenken. Nicht jedes Foto eignet sich gleich gut – mit ein paar einfachen Tricks holst du das Maximum heraus.

---

## Welche Bildformate werden unterstützt?

PIXEstL akzeptiert drei gängige Formate:

| Format | Empfehlung |
|--------|-----------|
| **PNG** | Beste Qualität, verlustfrei – ideal für Grafiken und Logos |
| **JPG / JPEG** | Gut für Fotos – achte auf geringe Komprimierung (hohe Qualitätsstufe) |
| **WebP** | Wird unterstützt, seltener verwendet |

!!! tip "JPG-Komprimierung"
    Stark komprimierte JPEGs (kleine Dateigröße, schlechte Qualität) haben sichtbare Blockartefakte, die sich in der Lithophanie zeigen. Verwende wenn möglich das Original oder eine hochqualitative Version.

---

## Was macht ein gutes Bild für eine Lithophanie?

### Gut geeignet

- **Hoher Kontrast:** Helle und dunkle Bereiche wechseln deutlich ab – die Texturschicht kann das Helligkeitsprofil gut abbilden.
- **Kräftige Farben:** Satte, nicht ausgeblichene Farben ergeben intensivere Farbschichten.
- **Klare Motive:** Portraits, Kunstwerke, Landschaften mit klarem Vordergrund funktionieren sehr gut.
- **Gute Ausleuchtung:** Gleichmäßig belichtete Fotos ohne starke Über- oder Unterbelichtung.

### Weniger geeignet

- **Sehr dunkle Bilder:** Viel Schwarz lässt kaum Licht durch – das Ergebnis wirkt flach und leblos.
- **Sehr helle / überbelichtete Bilder:** Die Helligkeitsunterschiede fehlen, der Kontrast ist gering.
- **Stark komprimierte JPEGs:** Blockartefakte werden in der Lithophanie sichtbar.
- **Bilder mit wenig Kontrast:** Nebelige oder flache Bilder verlieren noch mehr bei der Umwandlung.

!!! example "Gutes Beispiel"
    Ein Portrait mit natürlicher Beleuchtung, kräftigen Farben und scharfem Fokus auf das Gesicht – wie die Beispiele in der [Galerie](../galerie.md).

---

## Optimale Bildgröße

Die Auflösung des Eingabebildes beeinflusst nur die Rechenzeit, nicht direkt die Druckqualität – denn PIXEstL skaliert das Bild auf die physische Größe der Lithophanie herunter.

**Faustformel:** Ein 100 mm breites Druck mit 0.8 mm Pixelbreite hat nur **125 Pixel** in der Breite.

| Lithophanie-Breite | Pixels in der Breite (bei 0.8 mm) |
|--------------------|----------------------------------|
| 80 mm | 100 Pixel |
| 100 mm | 125 Pixel |
| 150 mm | 188 Pixel |
| 200 mm | 250 Pixel |

Das bedeutet: Ein 4000-Pixel-breites Bild wird auf 125 Pixel herunterskaliert – die extra Auflösung hilft beim Antialiasing, aber ein 500-Pixel-Bild reicht in der Regel vollkommen aus.

**Empfehlung:** Bildbreite zwischen **500 und 2000 Pixeln** auf der längeren Seite.

---

## Bild optimieren – schnelle Anleitung

Du brauchst dafür kein professionelles Bildbearbeitungsprogramm. Windows Paint, GIMP, Photoshop oder Lightroom funktionieren alle.

### Kontrast verbessern

Wenn dein Bild etwas flach wirkt, erhöhe Kontrast und Sättigung leicht:

- **GIMP:** Farben → Helligkeit-Kontrast, und Farben → Farbton-Sättigung
- **Lightroom:** Kontrast (+10 bis +30), Klarheit (+10 bis +20), Sättigung (+10 bis +20)
- **Photoshop:** Bild → Korrekturen → Helligkeit/Kontrast und Farbton/Sättigung

### Ausschnitt wählen

Schneide störende Randbereiche weg. Das Motiv sollte den Bildausschnitt möglichst gut ausfüllen – Randbereiche ohne Inhalt "verschwenden" Druckfläche.

### Auf sinnvolle Größe skalieren

Skaliere das Bild auf maximal 2000 Pixel Breite – das spart Rechenzeit und ändert die Ausgabequalität praktisch nicht.

- **GIMP:** Bild → Bild skalieren
- **Windows:** Rechtsklick → Größe ändern (in der Bildschau)

---

## Bildausrichtung und Seitenverhältnis

PIXEstL behält das Seitenverhältnis deines Bildes automatisch bei, wenn du nur `-w` (Breite) oder `-H` (Höhe) angibst.

Ein Hochformat-Foto (z.B. 3:4) ergibt eine Hochformat-Lithophanie, ein Querformat-Foto eine Querformat-Lithophanie.

Wenn du beide Dimensionen angibst, wird das Bild gestreckt oder gestaucht – das ist in den meisten Fällen unerwünscht.

---

## Nächster Schritt

Wenn dein Bild bereit ist: [PIXEstL ausführen →](generierung.md)

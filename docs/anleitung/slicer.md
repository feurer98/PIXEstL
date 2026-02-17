# Slicer-Einrichtung (Bambu Studio)

Diese Anleitung erklaert Schritt fuer Schritt, wie du die von PIXEstL erzeugten STL-Dateien in Bambu Studio einrichtest und fuer den Druck vorbereitest.

---

## Voraussetzungen

- [x] **Bambu Studio** ist installiert (Version 1.8 oder neuer empfohlen)
- [x] Die **PIXEstL-Ausgabe-ZIP** wurde entpackt (alle STL-Dateien liegen in einem Ordner)
- [x] Ein 3D-Drucker mit **AMS** (Automatic Material System) oder der Moeglichkeit zum **manuellen Filamentwechsel**

---

## Schritt 1: STL-Dateien importieren

1. Oeffne **Bambu Studio**
2. Gehe zu **File > Import** und waehle **alle STL-Dateien** aus dem entpackten Ordner aus

![STL-Dateien auswaehlen](../assets/images/anleitung/select_stl_files.png)

3. Bambu Studio fragt: *"Load these files as a single object with multiple parts?"* -- Klicke auf **Yes**

![Als einzelnes Objekt laden](../assets/images/anleitung/as_a_single_object.png)

!!! info "Warum als einzelnes Objekt?"
    Die STL-Dateien repraesentieren verschiedene Farbschichten, die exakt uebereinander liegen muessen. Durch das Laden als einzelnes Objekt mit mehreren Teilen bleiben die Positionen korrekt.

---

## Schritt 2: Filamente zuweisen

Im **Objects Panel** (linke Seitenleiste) siehst du jede Farbschicht als separaten Eintrag. Jeder Eintrag hat ein farbiges Quadrat, das das zugeordnete Filament anzeigt.

1. Klicke auf das **farbige Quadrat** neben jeder Schicht
2. Weise das **passende Filament** zu:

| STL-Datei | Filament-Slot |
|-----------|---------------|
| `layer-Cyan[...].stl` | Cyan-Filament |
| `layer-Magenta[...].stl` | Magenta-Filament |
| `layer-Yellow[...].stl` | Gelb-Filament |
| `layer-White[...].stl` | Weiss-Filament (Slot 1) |
| `layer-plate.stl` | Weiss-Filament (Slot 1) |
| `layer-texture-[...].stl` | Weiss-Filament (Slot 1) |

![Filament-Zuordnung](../assets/images/anleitung/bambu_color_selection.png)

!!! warning "Gleicher Slot fuer Platte, Weiss und Textur"
    Die Dateien `layer-plate`, `layer-White` und `layer-texture` muessen alle demselben Filament-Slot zugewiesen werden (Slot 1 = Weiss). Diese Schichten bilden zusammen die Basis und Oberflaeche der Lithophanie.

---

## Schritt 3: Druckparameter einstellen

![Druckparameter](../assets/images/anleitung/bambu_param.png)

Stelle die folgenden Parameter in Bambu Studio ein:

| Parameter | Wert | Hinweis |
|-----------|------|---------|
| Drucker | Bambu Lab X1C mit 0.2mm Duese | Andere Bambu-Modelle funktionieren ebenfalls |
| Druckprofil | 0.10mm Standard | Feinere Schichthoehe = bessere Farbmischung |
| Wall Loops | 4 | Sorgt fuer stabile Aussenwaende |
| Top/Bottom Shell Layers | 7 | Solide Ober- und Unterseite |
| **Infill Density** | **100%** | **KRITISCH** -- siehe Warnung unten |
| Infill Pattern | Rectilinear | Standard-Fuellmuster |

!!! danger "Infill MUSS 100% sein!"
    Lithophanien funktionieren nur mit **100% Infill**. Bei weniger als 100% werden die Farben nicht korrekt dargestellt, da das Licht durch die Hohlraeume unkontrolliert durchscheint. Ausserdem wird die Lithophanie strukturell instabil und kann beim Herausloesen vom Druckbett brechen.

---

## Schritt 4: Filament-Reihenfolge konfigurieren

Die Reihenfolge, in der die Filamente gedruckt werden, ist entscheidend fuer das Farbergebnis.

1. Oeffne die **Plate Settings** (unten links im Bedienfeld)
2. Setze **Print Sequence** auf `By layer`
3. Setze **Other layers filament sequence** auf `Customize`

![Plate Settings](../assets/images/anleitung/sequence_1.png)

4. Konfiguriere die Filament-Reihenfolge ab **Layer 7 bis Ende**:

```
Filament-Reihenfolge: 1 → 2 → 3 → 4
```

![Filament-Reihenfolge](../assets/images/anleitung/sequence_2.png)

!!! info "Warum ab Layer 7?"
    Die ersten 6 Schichten bestehen aus der Grundplatte und der weissen Basis. Ab Layer 7 beginnen die eigentlichen Farbschichten. Die Reihenfolge `1 → 2 → 3 → 4` bestimmt, in welcher Abfolge die Farben pro Schicht aufgetragen werden.

---

## Schritt 5: AMS-Pausen konfigurieren (optional)

Dieser Schritt ist nur noetig, wenn du **mehr als 4 Farben** verwendest und nur ein einzelnes AMS (4 Slots) zur Verfuegung hast.

1. Nutze den **Layer-Slider** auf der rechten Seite der Druckvorschau
2. Navigiere zur **Uebergangsschicht**, an der die naechste Farbgruppe beginnt
3. **Rechtsklicke** auf den Slider an dieser Position
4. Waehle **"Add Pause"** aus dem Kontextmenue

![Pause hinzufuegen](../assets/images/anleitung/pause.png)

Bei der Pause haelt der Drucker an, sodass du die Filamente im AMS gegen die naechste Farbgruppe tauschen kannst. Danach den Druck ueber das Display oder Bambu Handy fortsetzen.

!!! tip "Mehrere AMS-Einheiten"
    Wenn du ueber ein AMS Hub mit mehreren AMS-Einheiten verfuegst (8 oder 16 Slots), kannst du alle Farben gleichzeitig laden und benoetigst keine Pausen.

---

## Andere Slicer

Die gleichen Prinzipien gelten auch fuer andere Slicer wie **OrcaSlicer**, **PrusaSlicer** oder **Cura**:

1. **Alle STL-Dateien als Multi-Part-Objekt importieren** -- die Teile muessen als ein Objekt mit mehreren Koerpern geladen werden
2. **Filamente korrekt zuweisen** -- jede Farbschicht dem passenden Filament zuordnen
3. **100% Infill einstellen** -- ohne Ausnahme
4. **Filament-Reihenfolge konfigurieren** -- sofern der Slicer diese Option bietet

!!! note "OrcaSlicer"
    OrcaSlicer basiert auf Bambu Studio und bietet nahezu identische Einstellungsmoeglichkeiten. Die oben beschriebenen Schritte lassen sich direkt uebertragen.

---

## Naechste Schritte

- [Drucken & Nachbearbeitung](druck.md) -- Tipps zum Druckvorgang und zur Hintergrundbeleuchtung

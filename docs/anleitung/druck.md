# Drucken & Nachbearbeitung

Diese Seite beschreibt den vollstaendigen Ablauf vom Druckstart bis zur fertigen, hinterleuchteten Farb-Lithophanie.

---

## Vor dem Druck

Gehe diese Checkliste durch, bevor du den Druck startest:

- [x] Alle **Filamente geladen** (AMS oder manuell eingesetzt)
- [x] **Duese gereinigt** (0.2mm Duese empfohlen fuer beste Detailwiedergabe)
- [x] **Druckbett sauber** und korrekt gelevelt
- [x] **Richtiger Plattentyp** ausgewaehlt (Cool Plate oder PLA Plate)
- [x] **Slicer-Einstellungen** verifiziert -- insbesondere **100% Infill**
- [x] **Filament-Zuordnung** nochmals geprueft (Farben korrekt zugewiesen)

!!! warning "Vorab-Check nicht ueberspringen"
    Eine falsche Filament-Zuordnung oder fehlender 100%-Infill faellt erst nach Stunden Druckzeit auf. Ein schneller Check spart dir Zeit und Material.

---

## Druckvorgang

Der Druck einer Farb-Lithophanie laeuft in mehreren Phasen ab:

### Druckphasen im Ueberblick

| Phase | Schichten | Beschreibung |
|-------|-----------|--------------|
| Grundplatte | Layer 1--2 | Die Stuetzplatte wird mit dem Basis-Filament (Weiss) gedruckt |
| Basis-Fuellschicht | Layer 3--6 | Weisse Fuellschichten bilden die lichtdurchlaessige Grundlage |
| Farbschichten | Layer 7+ | Das AMS wechselt zwischen den Filamenten (Cyan, Magenta, Gelb, Weiss) |
| Texturschicht | Letzte Schichten | Die Relief-/Helligkeitsschicht gibt der Lithophanie ihre Tiefe |

### Typische Druckzeit

Die Druckdauer haengt von der Groesse und Komplexitaet ab:

| Breite | Ungefaehre Druckzeit |
|--------|---------------------|
| 80 mm | 3--4 Stunden |
| 120 mm | 4--6 Stunden |
| 160 mm | 6--8 Stunden |

!!! tip "Die ersten 10 Schichten beobachten"
    Bleibe waehrend der ersten 10 Schichten in der Naehe des Druckers. Wenn die Grundplatte sich vom Bett loest oder die erste Farbschicht nicht sauber haftet, kannst du frueh eingreifen und den Druck neu starten, anstatt Stunden zu verschwenden.

---

## Filamentwechsel bei mehr als 4 Farben

Wenn du mehr Farben verwendest, als dein AMS Slots hat (z.B. 6 Farben mit einem 4er-AMS), sind manuelle Filamentwechsel noetig.

### Ablauf

1. Der Drucker **pausiert automatisch** an den konfigurierten Schichten (siehe [Slicer-Einrichtung, Schritt 5](slicer.md#schritt-5-ams-pausen-konfigurieren-optional))
2. **Oeffne das AMS** und tausche die Filamentspulen gegen die naechste Farbgruppe
3. Stelle sicher, dass die neuen Filamente korrekt in die Slots eingefuehrt sind
4. **Setze den Druck fort** ueber das Drucker-Display oder die Bambu Handy App
5. Wiederhole den Vorgang fuer jede weitere Farbgruppe

!!! note "Filament-Gruppen planen"
    PIXEstL gibt in der Konsole aus, welche Farben in welcher Gruppe zusammengefasst sind. Notiere dir die Gruppen und lege die Spulen bereit, damit der Wechsel schnell geht.

---

## Nach dem Druck

### Abkuehlen und Entfernen

1. **Mindestens 30 Minuten abkuehlen lassen** -- die Lithophanie auf dem Druckbett vollstaendig auskuehlen lassen
2. **Vorsichtig mit einem Spachtel abloesen** -- setze den Spachtel flach an einer Ecke an und heble langsam
3. **Nicht biegen oder verdrehen** -- Lithophanien sind duenn und koennen leicht brechen

### Kanten saeubern

- Leichte Unebenheiten an den Raendern mit **feinem Schleifpapier** (Koernung 200--400) vorsichtig glaetten
- **Kein grobes Schleifen** -- die Farbschichten sind nur wenige Zehntel Millimeter dick

!!! danger "Vorsicht beim Entfernen!"
    Farb-Lithophanien sind duenner und fragiler als herkoemmliche 3D-Drucke. Wende niemals uebermassige Kraft an. Wenn sich der Druck nicht leicht loesen laesst, lege die Druckplatte fuer einige Minuten in den Kuehlschrank -- das thermische Schrumpfen hilft beim Abloesen.

---

## Hintergrundbeleuchtung

Eine Farb-Lithophanie entfaltet ihre volle Wirkung erst mit der richtigen Beleuchtung von hinten.

### Empfohlene Lichtquellen

| Methode | Qualitaet | Beschreibung |
|---------|-----------|--------------|
| LED-Lichtpanel | Beste | Gleichmaessige Ausleuchtung, idealerweise mit 5000--6500K (Tageslicht) |
| Helles Fenster | Gut | Die Lithophanie gegen ein sonnenbeschienenes Fenster halten |
| LED-Streifen im Rahmen | Gut | Selbstbau-Loesung: LED-Strip in einem einfachen Holz- oder 3D-Druckrahmen |
| Smartphone-Bildschirm | Ausreichend | Weisses Bild auf voller Helligkeit hinter die Lithophanie halten |

### Tipps fuer optimale Darstellung

- **Dunkler Raum** mit direkter Hintergrundbeleuchtung zeigt die Farben am intensivsten
- **Tageslicht-LEDs** (5000--6500K) geben die Farben am neutralsten wieder
- **Warmweisse LEDs** (2700--3000K) erzeugen einen leichten Gelbstich, der aber bei manchen Motiven reizvoll wirken kann
- Der **Abstand** zwischen Lichtquelle und Lithophanie beeinflusst die Gleichmaessigkeit -- 2--5 cm sind ideal

!!! tip "Beste Farbwiedergabe"
    Die Lithophanie zeigt ihre Farben am besten in einem **dunklen Raum** mit **direkter Hintergrundbeleuchtung** im Tageslicht-Bereich (5000--6500K). So kommen die CMYK-Farben am praezisesten zur Geltung.

---

## Fehlerbehebung

Hier findest du Loesungen fuer die haeufigsten Probleme:

| Problem | Moegliche Ursache | Loesung |
|---------|-------------------|---------|
| Farben stimmen nicht ueberein | Palette nicht kalibriert | Palette mit eigenen Filamenten [neu kalibrieren](kalibrierung.md) |
| Bild ist zu dunkel | Textur-Parameter nicht optimal | `texture-min` und `texture-max` in der CLI anpassen |
| Schichten loesen sich voneinander | Drucktemperatur zu niedrig | Drucktemperatur um 5--10 Grad erhoehen |
| Lithophanie bricht leicht | Zu duenne Waende oder Platte | Mehr Wall Loops verwenden oder dickere Grundplatte einstellen |
| Stringing zwischen Farben | Retraction nicht korrekt eingestellt | Retraction-Einstellungen im Slicer pruefen und erhoehen |
| Farben wirken verwaschen | Pixel zu grob aufgeloest | Feinere `color-pixel-width` verwenden (kleinerer Wert = mehr Detail) |
| Lithophanie ist komplett weiss | Falscher Filament-Slot | Filament-Zuordnung im Slicer ueberpruefen |
| Ungleichmaessige Beleuchtung | Lichtquelle nicht zentriert | LED-Panel mittig hinter der Lithophanie positionieren |

!!! info "Weitere Hilfe"
    Wenn dein Problem hier nicht aufgelistet ist, pruefe die [FAQ](../faq.md) oder oeffne ein [Issue auf GitHub](https://github.com/feurer98/PIXEstL/issues).

---

## Naechste Schritte

- [Slicer-Einrichtung](slicer.md) -- Zurueck zur Slicer-Konfiguration
- [CLI-Referenz](../cli-referenz.md) -- Alle Parameter und Optionen
- [Galerie](../galerie.md) -- Beispiele zur Inspiration

# Installation unter Windows

Diese Anleitung führt dich Schritt für Schritt durch die Installation von PIXEstL auf Windows.

---

## Was du brauchst

- Windows 10 oder Windows 11 (64-Bit)
- Eine Internetverbindung für den Download
- Etwa 500 MB freier Speicherplatz

---

## Schritt 1: Rust installieren

PIXEstL ist in der Programmiersprache Rust geschrieben. Du musst Rust einmalig installieren.

1. Öffne [https://rustup.rs](https://rustup.rs) im Browser
2. Lade den **Windows-Installer** (`rustup-init.exe`) herunter
3. Führe die Datei aus und folge dem Installer (einfach Enter drücken für die Standardoptionen)
4. Starte danach die **Eingabeaufforderung** (CMD) neu

![Screenshot: Rustup-Installer](../assets/images/anleitung/slicer.png)

!!! info "Was wird installiert?"
    Rustup installiert die Rust-Toolchain (`rustc`, `cargo`) und fügt sie automatisch zum Systempfad hinzu. Du brauchst keine weiteren Abhängigkeiten.

**Installation prüfen:**

```cmd
rustc --version
cargo --version
```

Du solltest etwas wie `rustc 1.75.0` sehen. Falls nicht, starte die Eingabeaufforderung neu.

---

## Schritt 2: Git installieren (falls noch nicht vorhanden)

Falls `git` noch nicht installiert ist, lade es von [https://git-scm.com](https://git-scm.com) herunter und installiere es.

**Prüfen:**

```cmd
git --version
```

---

## Schritt 3: PIXEstL herunterladen

Öffne die **Git Bash** oder die **Eingabeaufforderung** und führe aus:

```cmd
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL\rust
```

---

## Schritt 4: PIXEstL bauen

```cmd
cargo build --release
```

!!! info "Was macht dieser Befehl?"
    `cargo build --release` kompiliert PIXEstL mit allen Optimierungen. Das Ergebnis ist eine einzelne ausführbare Datei (`pixestl.exe`), die keine weiteren Installationen benötigt.

    Der erste Build dauert **1–3 Minuten**, da alle Abhängigkeiten heruntergeladen und kompiliert werden.

Das fertige Programm liegt unter:

```
PIXEstL\rust\target\release\pixestl.exe
```

---

## Schritt 5: Installation prüfen

```cmd
.\target\release\pixestl.exe --help
```

Du siehst die Hilfe-Ausgabe mit allen verfügbaren Optionen – die Installation ist abgeschlossen.

---

## Häufige Fehler

??? failure "`cargo` wird nicht gefunden"
    **Ursache:** Rustup wurde installiert, aber die Eingabeaufforderung wurde danach nicht neu gestartet.

    **Lösung:** Schließe alle Fenster der Eingabeaufforderung und öffne sie neu. Dann `cargo --version` erneut prüfen.

??? failure "Linker-Fehler beim Build"
    **Ursache:** Unter Windows benötigt Rust den Microsoft C++ Build-Toolchain (MSVC).

    **Lösung:** Lade die **Visual Studio Build Tools** von Microsoft herunter:
    [https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
    Wähle bei der Installation: *"Desktop-Entwicklung mit C++"*. Danach Rust erneut installieren oder `rustup update` ausführen.

??? failure "`git clone` schlägt fehl"
    **Ursache:** Git ist nicht installiert oder nicht im Systempfad.

    **Lösung:** Git von [https://git-scm.com](https://git-scm.com) installieren und die Eingabeaufforderung neu starten.

---

## Nächste Schritte

- [Schnellstart →](../schnellstart.md) – In 10 Minuten zur ersten Lithophanie
- [Palette kalibrieren →](../anleitung/kalibrierung.md) – Filamente einmessen für genaue Farben

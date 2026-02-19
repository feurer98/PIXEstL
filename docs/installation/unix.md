# Installation unter macOS / Linux

Diese Anleitung führt dich Schritt für Schritt durch die Installation von PIXEstL auf macOS oder Linux.

---

## Was du brauchst

- macOS 11+ oder eine aktuelle Linux-Distribution (Ubuntu, Fedora, Arch, …)
- Terminal-Zugang
- Internetverbindung
- Etwa 500 MB freier Speicherplatz

---

## Schritt 1: Rust installieren

PIXEstL ist in der Programmiersprache Rust geschrieben. Du musst Rust einmalig installieren.

Öffne ein Terminal und führe aus:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Der Installer fragt nach deinen Präferenzen – einfach **Enter** drücken für die Standardinstallation.

Danach musst du die Umgebungsvariablen neu laden:

```bash
source ~/.cargo/env
```

!!! info "Was wird installiert?"
    Rustup installiert `rustc` (Compiler) und `cargo` (Paketmanager) in deinem Home-Verzeichnis (`~/.cargo/`). Keine Root-Rechte nötig.

**Installation prüfen:**

```bash
rustc --version
cargo --version
```

Du solltest etwas wie `rustc 1.75.0` sehen.

---

## Schritt 2: Git prüfen

Git ist auf den meisten Systemen bereits vorinstalliert:

```bash
git --version
```

Falls nicht, installiere es:

=== "Ubuntu / Debian"
    ```bash
    sudo apt install git
    ```

=== "Fedora / RHEL"
    ```bash
    sudo dnf install git
    ```

=== "Arch Linux"
    ```bash
    sudo pacman -S git
    ```

=== "macOS (Homebrew)"
    ```bash
    brew install git
    ```

---

## Schritt 3: PIXEstL herunterladen

```bash
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust
```

---

## Schritt 4: PIXEstL bauen

```bash
cargo build --release
```

!!! info "Was macht dieser Befehl?"
    `cargo build --release` kompiliert PIXEstL mit allen Optimierungen (Link-Time-Optimization aktiv). Das Ergebnis ist eine einzelne ausführbare Datei ohne weitere Abhängigkeiten.

    Der erste Build dauert **1–3 Minuten**, da alle Abhängigkeiten heruntergeladen und kompiliert werden. Folge-Builds sind deutlich schneller.

Das fertige Programm liegt unter:

```
PIXEstL/rust/target/release/pixestl
```

---

## Schritt 5: Optional – ins System-PATH installieren

Damit du `pixestl` von überall aufrufen kannst:

```bash
# Option A: Symlink ins ~/.local/bin (kein sudo nötig)
mkdir -p ~/.local/bin
ln -s "$(pwd)/target/release/pixestl" ~/.local/bin/pixestl

# Option B: Über cargo install (empfohlen)
cargo install --path .
```

---

## Schritt 6: Installation prüfen

```bash
./target/release/pixestl --help
```

Du siehst die Hilfe-Ausgabe mit allen verfügbaren Optionen – die Installation ist abgeschlossen.

---

## Häufige Fehler

??? failure "`cargo` wird nicht gefunden"
    **Ursache:** Nach der Rustup-Installation wurde das Terminal nicht neu gestartet oder `source ~/.cargo/env` nicht ausgeführt.

    **Lösung:**
    ```bash
    source ~/.cargo/env
    # oder Terminal neu öffnen
    ```

??? failure "Linker-Fehler: `cc` not found (Linux)"
    **Ursache:** Der C-Linker fehlt. Rust benötigt auf Linux einen C-Compiler als Backend.

    **Lösung:**
    ```bash
    # Ubuntu/Debian:
    sudo apt install build-essential

    # Fedora:
    sudo dnf install gcc

    # Arch:
    sudo pacman -S base-devel
    ```

??? failure "Build schlägt auf macOS fehl: Xcode Command Line Tools"
    **Ursache:** Xcode Command Line Tools sind nicht installiert.

    **Lösung:**
    ```bash
    xcode-select --install
    ```

??? failure "`git clone` schlägt fehl: SSL-Zertifikat-Fehler"
    **Ursache:** Unternehmens-Proxy oder veraltete CA-Zertifikate.

    **Lösung:**
    ```bash
    git config --global http.sslVerify false
    # (nur temporär, auf vertrauenswürdigen Netzwerken)
    ```

---

## Nächste Schritte

- [Schnellstart →](../schnellstart.md) – In 10 Minuten zur ersten Lithophanie
- [Palette kalibrieren →](../anleitung/kalibrierung.md) – Filamente einmessen für genaue Farben

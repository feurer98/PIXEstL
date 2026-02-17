# Installation

## Systemvoraussetzungen

| Komponente | Mindestanforderung |
|------------|-------------------|
| **Betriebssystem** | Windows, macOS oder Linux |
| **Rust** | Version 1.75 oder neuer |
| **CPU** | Mehrkernprozessor empfohlen (fuer parallele Verarbeitung) |
| **RAM** | 100 MB fuer typische Bilder, 500 MB+ fuer 4K-Bilder |
| **Festplatte** | ~50 MB fuer das Programm + Platz fuer STL-Ausgaben |

## Rust installieren

Falls Rust noch nicht installiert ist:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Unter Windows: Lade den [Rust-Installer](https://www.rust-lang.org/tools/install) herunter.

## PIXEstL installieren

### Aus dem Quellcode bauen

```bash
# Repository klonen
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust

# Release-Build erstellen (optimiert)
cargo build --release
```

Das fertige Programm liegt unter:

```
PIXEstL/rust/target/release/pixestl
```

!!! tip "Tipp"
    Der Release-Build nutzt Link-Time-Optimization (LTO) und ist ca. 10x schneller als der Debug-Build. Die Kompilierung dauert dadurch etwas laenger (~1-2 Minuten).

### Installation ueberpruefen

```bash
./target/release/pixestl --help
```

Du solltest die Hilfe-Ausgabe mit allen verfuegbaren Optionen sehen.

## Naechste Schritte

Bereit? Dann weiter zum [Schnellstart](schnellstart.md)!

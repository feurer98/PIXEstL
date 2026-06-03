# Docker-Deployment (NAS-Test)

Das gesamte Web-Setup läuft als **ein einziger Container auf einem Port**: Der
`pixestl-server` liefert sowohl die `/api` als auch das gebaute React-Frontend
(statisch) aus. Vorteile fürs Testen auf dem NAS: nur ein Image, ein Port, kein
CORS, kein separater Reverse-Proxy.

```
Browser ──http://<nas-ip>:8787──▶ pixestl-server ──┬─ /            → statisches Frontend (/app/static)
                                                    ├─ /api/*       → Job-API
                                                    └─ ruft intern  → pixestl-CLI (Subprozess)
```

## Schnellstart (docker compose)

```bash
docker compose up -d --build
# Frontend öffnen:  http://<nas-ip>:8787
```

Oder ohne Compose:

```bash
docker build -t pixestl .
docker run -d --name pixestl -p 8787:8787 --restart unless-stopped pixestl
```

Anderer Host-Port: `-p 9000:8787` (Container-Port bleibt 8787).

## Auf dem NAS (Synology / QNAP)

- **Container Manager / Portainer:** Das Repo auf das NAS legen und ein
  Compose-Projekt aus `docker-compose.yml` anlegen, oder das Image woanders
  bauen (siehe unten) und hier nur starten.
- **Architektur:** `docker build` auf dem NAS baut automatisch für dessen CPU
  (die meisten NAS sind `amd64`, einige `arm64`). Baust du das Image auf einem
  anders-architekturierten Rechner, nutze
  `docker buildx build --platform linux/arm64 -t pixestl . --load`.
- **Build-Ressourcen:** Der Rust-Release-Build (LTO, `codegen-units=1`) ist
  CPU-/RAM-intensiv. Auf schwachen NAS kann er langsam sein oder das RAM
  sprengen. Dann besser **auf einem Arbeitsrechner bauen und übertragen**:

  ```bash
  # auf dem Arbeitsrechner (ggf. mit --platform für die NAS-Arch)
  docker build -t pixestl .
  docker save pixestl | gzip > pixestl.tar.gz
  # auf das NAS kopieren, dort:
  docker load < pixestl.tar.gz
  docker run -d -p 8787:8787 --restart unless-stopped pixestl
  ```

## Konfiguration (Env-Variablen)

| Variable | Default | Zweck |
|----------|---------|-------|
| `PORT` | `8787` | Lausch-Port im Container |
| `STATIC_DIR` | `/app/static` | Verzeichnis des Frontend-Bundles |
| `PIXESTL_BIN` | `/usr/local/bin/pixestl` | Pfad zum CLI-Binary |

Das Frontend ruft `/api` **same-origin** auf — im Container ist daher kein
`VITE_API_BASE` nötig.

## Wichtige Hinweise

- **Daten sind flüchtig.** Jobs liegen im Speicher, generierte Dateien in einem
  Temp-Verzeichnis im Container. Für einen Test reicht das; es gibt nichts zu
  persistieren.
- **Noch nicht härtungsreif** (siehe `docs/frontend/02-…`, V-MODEL-07): kein
  Job-TTL/Cleanup, kein Concurrency-Limit, CORS permissiv. Vor einer Freigabe
  über das lokale Netz hinaus absichern.
- **Healthcheck:** `GET /api/health` → `ok` (in Dockerfile/Compose hinterlegt).
- **Upload-Limit:** 64 MB pro Request (für große Bilder; anpassbar im Server).

## Troubleshooting

- *Build bricht beim `cargo`/`npm`-Download mit SSL-Fehler ab:* Du bist hinter
  einem TLS-aufbrechenden (MITM-)Proxy. Dann die Proxy-CA in die Build-Stages
  einbringen (z. B. CA nach `/usr/local/share/ca-certificates/` kopieren +
  `update-ca-certificates`, `CARGO_HTTP_CAINFO` / `NODE_EXTRA_CA_CERTS` setzen).
  Im normalen Heimnetz tritt das nicht auf.

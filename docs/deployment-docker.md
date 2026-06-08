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

## Build über GitHub Actions (tar.gz)

Statt lokal zu bauen, kann GitHub das Image bauen und direkt als `.tar.gz`
bereitstellen — praktisch, wenn das NAS zu schwach zum Bauen ist.

1. GitHub → **Actions** → Workflow **„Docker image (tar.gz)"** → **Run workflow**,
   Plattform wählen (`linux/amd64`, `linux/arm64` oder `both`). Läuft auch
   automatisch bei `v*.*.*`-Tags (amd64).
2. Nach dem Lauf das Artefakt `pixestl-amd64` bzw. `pixestl-arm64` herunterladen
   (enthält `pixestl-<arch>.tar.gz`).
3. Auf dem NAS laden und starten:
   ```bash
   gunzip pixestl-amd64.tar.gz
   docker load < pixestl-amd64.tar
   docker run -d -p 8787:8787 --restart unless-stopped pixestl:latest
   ```

> Hinweis: `linux/arm64` wird auf dem amd64-Runner per QEMU emuliert und ist
> daher deutlich langsamer. Workflow: `.github/workflows/docker-image.yml`.

## Konfiguration (Env-Variablen)

| Variable | Default | Zweck |
|----------|---------|-------|
| `PORT` | `8787` | Lausch-Port im Container |
| `STATIC_DIR` | `/app/static` | Verzeichnis des Frontend-Bundles |
| `PIXESTL_BIN` | `/usr/local/bin/pixestl` | Pfad zum CLI-Binary |
| `PIXESTL_MAX_JOBS` | `min(CPU-Kerne, 2)` | Gleichzeitig laufende Generierungen. Jeder Job belegt das ganze Mesh im RAM — höher nur mit ausreichend Speicher. |
| `PIXESTL_JOB_TIMEOUT_SECS` | `240` | Harte Zeitgrenze pro Job; danach wird der Subprozess **gekillt** und der Job als Fehler markiert (verhindert hängende Prozesse). |
| `PIXESTL_MAX_TRIANGLES` | `20000000` | Obergrenze der geschätzten Mesh-Größe. Größere Anfragen werden mit **400** abgelehnt, statt das RAM zu sprengen. Auf speicherarmen Hosts senken. |

Das Frontend ruft `/api` **same-origin** auf — im Container ist daher kein
`VITE_API_BASE` nötig.

## Reverse-Proxy (gegen 502 Bad Gateway)

Läuft der Container hinter einem Reverse-Proxy (Nginx Proxy Manager, Traefik,
Caddy, Cloudflare Tunnel …) — wie bei einer HTTPS-Domain üblich — **erzeugt der
Proxy einen 502**, sobald das Backend nicht rechtzeitig antwortet oder abstürzt.
Zwei Proxy-Einstellungen müssen zum Backend passen:

- **Upload-Limit ≥ 64 MB**, sonst scheitert der Bild-Upload.
  Nginx: `client_max_body_size 64m;`
- **Lese-Timeout ≥ `PIXESTL_JOB_TIMEOUT_SECS`.** Der initiale `POST /api/jobs`
  überträgt das ganze Bild; bei großen Uploads/langsamen Verbindungen kann ein
  kurzer Default-Timeout (Nginx: 60 s) in einen 502/504 laufen.
  Nginx: `proxy_read_timeout 300s; proxy_send_timeout 300s;`

Beispiel (Nginx):

```nginx
location / {
    proxy_pass http://127.0.0.1:8787;
    client_max_body_size 64m;
    proxy_read_timeout 300s;
    proxy_send_timeout 300s;
}
```

Tritt der 502 trotzdem auf, am Container-Port prüfen, ob das Backend noch lebt:
`curl http://<nas-ip>:8787/api/health`. Antwortet `health` weiter, ist es ein
Proxy-Timeout/-Limit (s. o.); ist der Container weg, wurde der Prozess
OOM-gekillt — dann `PIXESTL_MAX_TRIANGLES` / `PIXESTL_MAX_JOBS` senken bzw. dem
Container mehr RAM geben (`docker logs pixestl` zeigt OOM/`SIGKILL`).

## Wichtige Hinweise

- **Daten sind flüchtig.** Jobs liegen im Speicher, generierte Dateien in einem
  Temp-Verzeichnis im Container. Für einen Test reicht das; es gibt nichts zu
  persistieren.
- **Noch nicht härtungsreif** (siehe `docs/frontend/02-…`, V-MODEL-07): kein
  Job-TTL/Cleanup, CORS permissiv. Vor einer Freigabe über das lokale Netz
  hinaus absichern. (Concurrency-Limit, Job-Timeout und Mesh-Größen-Guard sind
  vorhanden — siehe Env-Variablen oben.)
- **Healthcheck:** `GET /api/health` → `ok` (in Dockerfile/Compose hinterlegt).
- **Upload-Limit:** 64 MB pro Request (für große Bilder; anpassbar im Server).

## Troubleshooting

- *Build bricht beim `cargo`/`npm`-Download mit SSL-Fehler ab:* Du bist hinter
  einem TLS-aufbrechenden (MITM-)Proxy. Dann die Proxy-CA in die Build-Stages
  einbringen (z. B. CA nach `/usr/local/share/ca-certificates/` kopieren +
  `update-ca-certificates`, `CARGO_HTTP_CAINFO` / `NODE_EXTRA_CA_CERTS` setzen).
  Im normalen Heimnetz tritt das nicht auf.

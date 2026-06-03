# syntax=docker/dockerfile:1
#
# Single-image build for the PIXEstL web setup: the axum server serves both the
# /api and the built React frontend, so the whole thing runs as ONE container on
# ONE port (8787) — ideal for a NAS test (no CORS, no reverse proxy needed).
#
# Build:  docker build -t pixestl .
# Run:    docker run --rm -p 8787:8787 pixestl   # open http://<host>:8787

# ---- Stage 1: build the Rust CLI + the server (release) ----
FROM rust:1-bookworm AS rust-builder
WORKDIR /build
COPY rust/ ./rust/
COPY server/ ./server/
RUN cd rust && cargo build --release --bin pixestl
RUN cd server && cargo build --release --bin pixestl-server

# ---- Stage 2: build the frontend static bundle ----
FROM node:22-bookworm-slim AS web-builder
WORKDIR /web
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# ---- Stage 3: slim runtime ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust-builder /build/rust/target/release/pixestl /usr/local/bin/pixestl
COPY --from=rust-builder /build/server/target/release/pixestl-server /usr/local/bin/pixestl-server
COPY --from=web-builder /web/dist /app/static
ENV PIXESTL_BIN=/usr/local/bin/pixestl \
    STATIC_DIR=/app/static \
    PORT=8787
EXPOSE 8787
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -fsS http://localhost:8787/api/health || exit 1
CMD ["pixestl-server"]

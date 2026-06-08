import { useCallback, useEffect, useRef, useState } from 'react';
import type { ExportFormat, Settings } from '../lib/types';

export type ExportState = 'idle' | 'exporting' | 'done' | 'error';

// ─── Export flow (backend variant, V-MODEL-07) ───────────────────────────────
// Uploads the original image + palette files plus the settings to the
// pixestl-server (see /server), polls the async job, then downloads the
// generated 3MF/ZIP. The API base is same-origin by default (Vite dev-proxies
// /api → the server); override with VITE_API_BASE for a separate host.

const API_BASE = import.meta.env.VITE_API_BASE ?? '';
const POLL_INTERVAL_MS = 1000;
const POLL_TIMEOUT_MS = 5 * 60 * 1000;
// Mirrors the server's DefaultBodyLimit (64 MB); reject early with a clear message.
const MAX_UPLOAD_BYTES = 64 * 1024 * 1024;

export interface ExportRequest {
  settings: Settings;
  format: ExportFormat;
  imageFile: File | null;
  paletteFile: File | null;
  /** Hex colors of the currently active filaments (UI toggles, V-MODEL-13). */
  activeColors: string[];
}

interface JobView {
  status: 'queued' | 'running' | 'done' | 'error';
  error?: string;
  filename?: string;
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

export function useExport() {
  const [exportState, setExportState] = useState<ExportState>('idle');
  const [exportFormat, setExportFormat] = useState<ExportFormat>('3mf');
  const [exportError, setExportError] = useState<string | null>(null);
  const mounted = useRef(true);
  useEffect(() => () => { mounted.current = false; }, []);

  const runExport = useCallback(
    async (req: ExportRequest) => {
      if (exportState === 'exporting') return; // allow retry from idle/done/error
      if (!req.imageFile) {
        setExportError('Zuerst ein Bild laden');
        setExportState('error');
        return;
      }
      if (req.imageFile.size > MAX_UPLOAD_BYTES) {
        setExportError(
          `Bild zu groß (${(req.imageFile.size / 1024 / 1024).toFixed(1)} MB, max. 64 MB)`,
        );
        setExportState('error');
        return;
      }
      if (!req.paletteFile) {
        // The backend needs the original palette file (V-MODEL-01).
        setExportError('Bitte zuerst eine Palette-Datei laden');
        setExportState('error');
        return;
      }

      setExportError(null);
      setExportState('exporting');
      try {
        const form = new FormData();
        form.append('image', req.imageFile);
        form.append('palette', req.paletteFile);
        form.append('settings', JSON.stringify(req.settings));
        form.append('format', req.format);
        form.append('activeColors', JSON.stringify(req.activeColors));

        const createRes = await fetch(`${API_BASE}/api/jobs`, { method: 'POST', body: form });
        if (!createRes.ok) throw new Error(`Server ${createRes.status}: ${await createRes.text()}`);
        const { jobId } = (await createRes.json()) as { jobId: string };

        // Poll until done or error.
        const deadline = Date.now() + POLL_TIMEOUT_MS;
        let view: JobView;
        for (;;) {
          await sleep(POLL_INTERVAL_MS);
          const res = await fetch(`${API_BASE}/api/jobs/${jobId}`);
          if (!res.ok) throw new Error(`Statusabfrage fehlgeschlagen (${res.status})`);
          view = (await res.json()) as JobView;
          if (view.status === 'done' || view.status === 'error') break;
          if (Date.now() > deadline) throw new Error('Zeitüberschreitung bei der Generierung');
        }
        if (view.status === 'error') throw new Error(view.error || 'Generierung fehlgeschlagen');

        // Download and save.
        const dl = await fetch(`${API_BASE}/api/jobs/${jobId}/download`);
        if (!dl.ok) throw new Error(`Download fehlgeschlagen (${dl.status})`);
        const blob = await dl.blob();
        triggerDownload(blob, view.filename || `lithophane.${req.format}`);

        if (!mounted.current) return;
        setExportState('done');
        await sleep(2500);
        if (mounted.current) setExportState('idle');
      } catch (err) {
        if (!mounted.current) return;
        setExportError(err instanceof Error ? err.message : 'Unbekannter Fehler');
        setExportState('error');
      }
    },
    [exportState],
  );

  /** Clear an error state back to idle (e.g. when the user retries). */
  const resetExport = useCallback(() => {
    setExportError(null);
    setExportState('idle');
  }, []);

  return { exportState, exportFormat, setExportFormat, exportError, runExport, resetExport };
}

function triggerDownload(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

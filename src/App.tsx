import { useEffect, useState } from 'react';
import { getBackendVersion } from './features/system/api/version';
import { getApiTransportKind } from './shared/api/client';
import type { BackendVersion } from './shared/api/types';

const boundaries = [
  'Packaged React/Vite assets',
  'System WebKitGTK renderer',
  'Typed allowlisted Tauri IPC',
  'No localhost backend or WebView fs/process/network privilege',
] as const;

export default function App() {
  const [health, setHealth] = useState<BackendVersion | null>(null);
  const [healthError, setHealthError] = useState<string | null>(null);

  useEffect(() => {
    void getBackendVersion()
      .then((value) => setHealth(value))
      .catch((error: unknown) => setHealthError(error instanceof Error ? error.message : 'IPC probe failed'));
  }, []);

  return (
    <main className="shell" aria-labelledby="app-title">
      <section className="hero">
        <p className="eyebrow">ARCH NATIVE · A02 TRANSPORT</p>
        <h1 id="app-title">p2pKanban</h1>
        <p className="lede">
          The presentation now talks through an explicit transport facade. Desktop routes are mapped to named Rust commands rather than a localhost HTTP server.
        </p>
      </section>

      <section className="boundary-card" aria-label="Current native boundaries">
        <h2>Current boundary</h2>
        <ul>{boundaries.map((boundary) => <li key={boundary}>{boundary}</li>)}</ul>
      </section>

      <section className="boundary-card" aria-live="polite">
        <h2>Transport probe</h2>
        <p>mode: <strong>{getApiTransportKind()}</strong></p>
        {health ? <p>{health.service} {health.version} · {health.status} · {health.env}</p> : null}
        {healthError ? <p>probe pending/failed: {healthError}</p> : null}
        {!health && !healthError ? <p>checking typed IPC…</p> : null}
      </section>

      <footer>
        <span>A02 typed transport seam</span>
        <span>Next: A03 Rust application/domain boundary</span>
      </footer>
    </main>
  );
}

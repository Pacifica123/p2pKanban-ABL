const boundaries = [
  "Packaged React/Vite assets",
  "System WebKitGTK renderer",
  "No localhost backend",
  "No WebView filesystem/process privilege",
] as const;

export default function App() {
  return (
    <main className="shell" aria-labelledby="app-title">
      <section className="hero">
        <p className="eyebrow">ARCH NATIVE · A01 FOUNDATION</p>
        <h1 id="app-title">p2pKanban</h1>
        <p className="lede">
          Native Linux shell is active. Domain, persistence and sync arrive in later
          architecture stages rather than through a temporary local server.
        </p>
      </section>

      <section className="boundary-card" aria-label="Current native boundaries">
        <h2>Current boundary</h2>
        <ul>
          {boundaries.map((boundary) => (
            <li key={boundary}>{boundary}</li>
          ))}
        </ul>
      </section>

      <footer>
        <span>A01 source foundation</span>
        <span>Next: locked build evidence, then A02 transport adapter</span>
      </footer>
    </main>
  );
}

# p2pKanban Arch Native

Dedicated Arch-based Linux desktop implementation of p2pKanban.

This repository follows the implementation sequence `A00 … A19` from the accepted Arch-native architecture. The normal runtime target is **Tauri 2 + system WebKitGTK + one in-process Rust core + SQLite**, with no mandatory Docker, PostgreSQL, Node runtime, localhost backend, root helper or system service.

## Current state

**A00 is implemented. A01a source/security foundation is implemented; A01 runtime acceptance is not yet claimed.**

A01a adds the permanent React/Vite + Tauri 2 source layout, a Rust-created WebView window, deny-by-default navigation/new-window/download hooks, CSP with network connections disabled, and an empty Tauri capability set. It deliberately does not introduce a local HTTP server, filesystem/process plugins, persistence, auth or sync.

The patch-construction runner had no Rust/Cargo toolchain or crates cache, so it would be dishonest to fabricate `Cargo.lock` or claim a successful Tauri compile. The exact next patch is **A01b — resolver-generated Cargo lock + offline build/launch/security evidence**. After A01b is green, proceed to **A02 — explicit web↔desktop transport adapter**.

## Source layout

- `src/` — React presentation source.
- `src-tauri/` — native Tauri shell source and deny-by-default WebView policy.
- `docs/architecture/` — implementation-driving architecture and ADRs.
- `docs/IMPLEMENTATION_STATUS.md` — decision → files/tests → status → next patch traceability.
- `docs/evidence/A00_EVIDENCE_BASELINE.md` — frozen source facts.
- `docs/evidence/A01_SHELL_FOUNDATION.md` — A01a evidence, classifications and A01b acceptance gap.
- `evidence/` and `fixtures/` — frozen compatibility evidence for later stages.

## Deterministic checks

```bash
python3 -B tools/check_a00.py
python3 -B tools/check_a01.py
```

These checks are offline-only and do not install packages, contact registries, start services or mutate `.devctl`. They validate source/security contracts and npm lock consistency structurally. Real frontend/Rust compilation is an explicit A01b gate rather than a fake-green A01a check.

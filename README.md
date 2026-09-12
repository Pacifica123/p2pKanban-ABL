# p2pKanban Arch Native

Dedicated Arch-based Linux desktop implementation of p2pKanban.

This repository follows the implementation sequence `A00 … A19` from the accepted Arch-native architecture. The normal runtime target is **Tauri 2 + system WebKitGTK + one in-process Rust core + SQLite**, with no mandatory Docker, PostgreSQL, Node runtime, localhost backend, root helper or system service.

## Current state

**A00 is implemented. A01a source/security foundation, A01b offline-build harness and A01c host-evidence contract are implemented; A01 compiled/WebView runtime acceptance is not yet claimed.**

A01c also corrects an implementation-evidence error: the selected Tauri 2.11.5 workspace declares Rust **1.90** as its floor, not the previously recorded 1.77.2. The correction is tracked in `DEBT-A01-002` rather than being hidden in the manifest.

The patch-construction environment still cannot resolver-generate `src-tauri/Cargo.lock` or execute an Arch graphical WebView build. These remain explicit UTS verification items rather than fake passes. A02 now provides the web↔desktop transport seam and an allowlisted typed health IPC route. Exact next implementation patch: **A03 — Rust application/domain boundary**. See `docs/UTS_A01_A02_VERIFICATION.md` for the host commands that must be run personally in UserTestSpace.

## Source layout

- `src/` — React presentation source.
- `src-tauri/` — native Tauri shell source and deny-by-default WebView policy.
- `docs/architecture/` — implementation-driving architecture and ADRs.
- `docs/IMPLEMENTATION_STATUS.md` — decision → files/tests → status → next patch traceability.
- `docs/evidence/A00_EVIDENCE_BASELINE.md` — frozen source facts.
- `docs/evidence/A01*.md` — shell/build/runtime evidence boundaries and unresolved host experiments.
- `evidence/` and `fixtures/` — frozen compatibility evidence for later stages.

## Deterministic checks

```bash
python3 -B tools/check_a00.py
python3 -B tools/check_a01.py
python3 -B tools/check_a01b.py
python3 -B tools/check_a01c.py
npm install --package-lock-only --offline --ignore-scripts --no-audit --no-fund
```

These generic checks are offline-only and do not install system packages, contact registries, start services or mutate `.devctl`. Real host acceptance is deliberately separate and strict:

```bash
python3 -B tools/a01b_host_acceptance.py doctor
python3 -B tools/a01b_host_acceptance.py offline-build
python3 -B tools/a01c_host_runtime_probe.py doctor
python3 -B tools/a01c_host_runtime_probe.py launch-probe --report /tmp/p2pkanban-a01c-runtime.json
```

Missing build/session prerequisites are failure for the corresponding host acceptance command, never a fake-green skip.

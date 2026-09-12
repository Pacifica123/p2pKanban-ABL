# p2pKanban Arch Native

Dedicated Arch-based Linux desktop implementation of p2pKanban.

This repository follows the implementation sequence `A00 … A19` from the accepted Arch-native architecture. The normal runtime target is **Tauri 2 + system WebKitGTK + one in-process Rust core + SQLite**, with no mandatory Docker, PostgreSQL, Node runtime, localhost backend, root helper or system service.

## Current state

A00 is implemented. A01 source/security foundation and A02 typed desktop transport exist. The first real Arch-family UserTestSpace build passed frontend typecheck/build and Cargo lock/fetch, then found a concrete Tauri packaging defect: `src-tauri/icons/icon.png` was missing. **A02b fixes that defect**, explicitly owns the icon, removes the unsupported Node `<23` build-time ceiling, and introduces a persistent one-command UTS verifier.

Post-fix native Cargo/WebView evidence is still verification-pending; it is not represented as green until UTS says so. Exact next architecture patch: **A03 — Rust application/domain boundary**.

## Source layout

- `src/` — React presentation source.
- `src-tauri/` — native Tauri shell source and deny-by-default WebView policy.
- `docs/architecture/` — implementation-driving architecture and ADR/debt corrections.
- `docs/IMPLEMENTATION_STATUS.md` — decision → files/tests → status → next patch traceability.
- `docs/evidence/` — frozen source facts plus implementation/UTS evidence boundaries.
- `evidence/` and `fixtures/` — frozen compatibility evidence for later stages.
- `tools/uts_plan.json` — evolving machine-readable UserTestSpace verification plan.
- `tools/uts_verify.py` — stable single-entry UTS runner.

## Deterministic devctl checks

`devctl start` uses network-free source/contracts gates only. A02b adds:

```bash
python3 -B tools/check_a02b.py
```

## UserTestSpace host verification

Run one command from the UTS project root:

```bash
python3 -B tools/uts_verify.py
```

If the summary reports missing npm/Cargo cache entries and build-time network preparation is acceptable:

```bash
python3 -B tools/uts_verify.py --allow-network
```

Reports are written under ignored `.uts-reports/`; send `latest-summary.txt` and the referenced failing log when a check fails. See `docs/UTS_VERIFICATION.md`.

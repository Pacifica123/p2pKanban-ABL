# p2pKanban Arch Native

Dedicated Arch-based Linux desktop implementation of p2pKanban.

This repository follows the implementation sequence `A00 … A19` from the accepted Arch-native architecture. The normal runtime target is **Tauri 2 + system WebKitGTK + one in-process Rust core + SQLite**, with no mandatory Docker, PostgreSQL, Node runtime, localhost backend, root helper or system service.

## Current state

**A00 is implemented. Runtime code is intentionally not implemented yet.**

A00 freezes the reviewed web/backend, Android and devctl evidence, imports the external architecture package into this repository as the living SSOT, records protocol/export fixtures and migration/source digests, and provides an offline deterministic self-check.

The next patch is **A01 — Tauri 2 shell + packaged React/Vite asset**. Do not skip directly to SQLite or auth: A02–A04 establish the transport/application/repository seams before serious persistence porting.

## SSOT

- `docs/architecture/` — implementation-driving architecture and ADRs.
- `docs/IMPLEMENTATION_STATUS.md` — decision → files/tests → status → next patch traceability.
- `docs/evidence/A00_EVIDENCE_BASELINE.md` — frozen source facts and classification policy.
- `evidence/source-anchors.json` — content hashes of high-value implementation anchors.
- `evidence/migrations.sha256` — PostgreSQL migration digest used as legacy semantic evidence, not a SQLite migration plan.
- `fixtures/` — canonical logical compatibility fixtures for later contract tests.

## Applying the initiating patch

The dedicated target repository must already have a seed commit on `main` containing the sentinel file `ARCH_NATIVE_REPO` with the single line `p2pkanban-archlinux-native`, plus normal Git author identity configured. This is a devctl v0.7.0 rollback/targeting precondition documented as `DEBT-A00-001`; it is not a native runtime requirement.

## Deterministic A00 check

```bash
python3 -B tools/check_a00.py
```

The check is offline-only and does not install packages, contact registries, start services or mutate `.devctl`.

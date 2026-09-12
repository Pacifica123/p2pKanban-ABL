# Implementation status

This ledger is normative for claims about the Arch-native repository. `implemented` means code/evidence required by that stage exists here and is covered by the named deterministic checks; architecture prose alone never counts as implementation.

| Decision / stage | Code, files, tests | Status | Next Axx |
|---|---|---|---|
| DEBT-A00-001 devctl commitless-repo rollback limitation | `docs/architecture/08-implementation-corrections-and-debt.md`; seeded-repo rollback validation | **planned tooling fix / operationally mitigated** | devctl future |
| A00 evidence baseline + protocol fixtures | `docs/architecture/**`, `docs/evidence/A00_EVIDENCE_BASELINE.md`, `evidence/**`, `fixtures/**`, `tools/check_a00.py` | **implemented** | A01 |
| ADR-001 Tauri 2 single user-session process model, no default listener/service | `src-tauri/**`, `docs/evidence/A01*.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md`, `docs/evidence/A02C_UTS_REPEATABILITY_FINDINGS.md`, `tools/check_a01*.py`, `tools/uts_verify.py` | **partially implemented; post-fix UTS Cargo test/build and non-root Wayland runtime probe passed on Manjaro; manual rendered-UI/hostile-navigation evidence remains pending** | A03 / later security evidence |
| A01 Tauri 2 shell + packaged React/Vite asset | `package*.json`, `src/**`, `src-tauri/**`, `src-tauri/icons/icon.png`, `tools/check_a01*.py`, `tools/repository_contract.py`, `tools/uts_verify.py`, `docs/evidence/A01*.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md`, `docs/evidence/A02C_UTS_REPEATABILITY_FINDINGS.md` | **partially implemented (A01a+A01b+A01c with A02b/A02c corrections); frontend + Cargo test/build + native Wayland launch probe are UTS-green; dedicated hostile-navigation and rendered-health observation remain verification-pending** | A03 / later security evidence |
| A02 explicit web↔desktop frontend transport adapter | `src/shared/api/**`, `src/shared/transport/**`, `src/features/system/api/version.ts`, `src-tauri/src/desktop_api.rs`, `tools/check_a02.py`, `tools/check_a02b.py`, `tools/check_a02c.py`, `tools/uts_verify.py`, `docs/evidence/A02_TRANSPORT_BOUNDARY.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md`, `docs/evidence/A02C_UTS_REPEATABILITY_FINDINGS.md` | **implemented at source/compile/runtime-launch level; UTS build and launch probe are green, while direct rendered WebView→Rust health observation remains manual evidence** | A03 |
| A03 Rust application/domain boundary free of `sqlx::Pg*` APIs | none yet | **planned** | A03 |
| A04 repository semantic contract suite | frozen source/fixture evidence only | **planned** | A04 |
| ADR-002 SQLite embedded profile store | architecture only; no DB file/schema exists | **planned** | A05 |
| A05 SQLite schema + atomic migration engine | none yet | **planned** | A05 |
| A06 XDG adapters + multi-instance/locking behavior | architecture only | **planned** | A06 |
| A07 auth/workspace/board minimal offline slice + vault interface | compatibility evidence only | **planned** | A07 |
| A08 cards/order/archive/delete/checklists | roaming/tombstone evidence only | **planned** | A08 |
| ADR-004 Secret Service/passphrase/session-only secret policy | architecture only | **planned** | A09 |
| A09 production Linux secret persistence matrix | none yet | **planned** | A09 |
| A10 sync-core + Nostr roaming compatibility | golden logical fixtures only; no desktop transport | **planned** | A10 |
| A11 device-link/2 + web-node-link/bundle migration | compatibility fixture/evidence only | **planned** | A11 |
| A12 labels/comments/activity/appearance parity | none yet | **planned** | A12 |
| ADR-006 foreground lifecycle; systemd-user optional only | architecture only | **planned** | A13 |
| A13 Wayland/X11 + notifications/deep links/tray capability detection | none yet | **planned** | A13 |
| ADR-005 IPC and optional bounded LAN bridge | IPC side begins A02; LAN bridge not implemented | **planned** | A14 |
| A14 optional LAN compatibility bridge, off by default | none yet | **planned** | A14 |
| ADR-003 pacman-owned packaging/update; AppImage fallback | architecture only | **planned** | A15 |
| A15 PKGBUILD + signed-repo packaging | none yet | **planned** | A15 |
| A16 backup/doctor/safe-mode/recovery | none yet | **planned** | A16 |
| A17 AppImage fallback + offline release kit | none yet | **planned** | A17 |
| A18 performance/power/rolling-release hardening | no measurements yet | **experiment-needed** | A18 |
| A19 Iroh / Arch ARM evidence track | no promotion evidence | **experiment-needed** | A19 |

## Explicit non-claims after A02c

Post-A02b UTS evidence now shows a complete frontend build, resolver lock/fetch, `cargo test --locked --offline`, `cargo build --locked --offline`, binary presence, and non-root Wayland runtime launch probe passing on Manjaro. The runtime evidence recorded no TCP listener owned by the application process tree. The second immediate UTS invocation exposed three repeatability bugs in deterministic checkers; A02c fixes those checker defects and adds a post-build deterministic replay so one verifier invocation validates its own generated state.

This still does **not** claim manual rendered WebView health text or hostile-navigation runtime behavior has been observed. `src-tauri/Cargo.lock` generated in UTS is also not silently incorporated because its exact bytes were not supplied as a source artifact. There is still no A03 application/domain service layer, SQLite profile, production XDG persistence, secret vault, Linux package, sync transport or durable offline user workflow.

# Implementation status

This ledger is normative for claims about the Arch-native repository. `implemented` means code/evidence required by that stage exists here and is covered by the named deterministic checks; architecture prose alone never counts as implementation.

| Decision / stage | Code, files, tests | Status | Next Axx |
|---|---|---|---|
| DEBT-A00-001 devctl commitless-repo rollback limitation | `docs/architecture/08-implementation-corrections-and-debt.md`; seeded-repo rollback validation | **planned tooling fix / operationally mitigated** | devctl future |
| A00 evidence baseline + protocol fixtures | `docs/architecture/**`, `docs/evidence/A00_EVIDENCE_BASELINE.md`, `evidence/**`, `fixtures/**`, `tools/check_a00.py` | **implemented** | A01 |
| ADR-001 Tauri 2 single user-session process model, no default listener/service | `src-tauri/**`, `docs/evidence/A01*.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md`, `tools/check_a01*.py`, `tools/uts_verify.py` | **partially implemented; first Arch-family build attempt reached Tauri context and exposed a missing-icon defect, fixed in A02b; post-fix runtime verification pending** | UTS verifier |
| A01 Tauri 2 shell + packaged React/Vite asset | `package*.json`, `src/**`, `src-tauri/**`, `src-tauri/icons/icon.png`, `tools/check_a01*.py`, `tools/uts_verify.py`, `docs/evidence/A01*.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md` | **partially implemented (A01a+A01b+A01c+A02b correction); frontend build verified in UTS, native post-fix Cargo/WebView verification pending** | UTS verifier |
| A02 explicit web↔desktop frontend transport adapter | `src/shared/api/**`, `src/shared/transport/**`, `src/features/system/api/version.ts`, `src-tauri/src/desktop_api.rs`, `tools/check_a02.py`, `tools/check_a02b.py`, `tools/uts_verify.py`, `docs/evidence/A02_TRANSPORT_BOUNDARY.md`, `docs/evidence/A02B_UTS_BUILD_FINDINGS.md` | **partially implemented; source/contracts green; first UTS native build exposed icon defect now fixed; post-fix IPC/runtime verification pending** | A03 |
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

## Explicit non-claims after A02b

The first real Arch-family UTS pass materially improved the evidence boundary: frontend install/typecheck/build passed, Cargo lock/fetch succeeded, and native compilation reached `tauri::generate_context!()`. It then failed on a missing repository icon. A02b fixes that concrete defect and removes the unsupported Node `<23` cap, but **does not claim** the post-fix Cargo test/build or WebView→Rust IPC launch has passed yet.

`tools/uts_verify.py` is now the canonical single-entry host verifier and writes ignored `.uts-reports/` evidence. A03 may proceed under DELIVERY-A01-003/DELIVERY-A02-006 while this host evidence is pending, but any new UTS compiler/runtime defect re-opens the affected stage. There is still no A03 application/domain service layer, SQLite profile, production XDG persistence, secret vault, Linux package, sync transport or durable offline user workflow.

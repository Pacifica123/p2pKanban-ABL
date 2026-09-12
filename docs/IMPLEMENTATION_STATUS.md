# Implementation status

This ledger is normative for claims about the Arch-native repository. `implemented` means code/evidence required by that stage exists here and is covered by the named deterministic checks; architecture prose alone never counts as implementation.

| Decision / stage | Code, files, tests | Status | Next Axx |
|---|---|---|---|
| DEBT-A00-001 devctl commitless-repo rollback limitation | `docs/architecture/08-implementation-corrections-and-debt.md`; seeded-repo rollback validation | **planned tooling fix / operationally mitigated** | devctl future |
| A00 evidence baseline + protocol fixtures | `docs/architecture/**`, `docs/evidence/A00_EVIDENCE_BASELINE.md`, `evidence/**`, `fixtures/**`, `tools/check_a00.py` | **implemented** | A01 |
| ADR-001 Tauri 2 single user-session process model, no default listener/service | `src-tauri/**`, `docs/evidence/A01_SHELL_FOUNDATION.md`, `docs/evidence/A01B_BUILD_ACCEPTANCE.md`, `tools/check_a01.py`, `tools/check_a01b.py`, `tools/a01b_host_acceptance.py` | **partially implemented (A01a+A01b harness; resolver/build/launch evidence pending)** | A01c |
| A01 Tauri 2 shell + packaged React/Vite asset | `package*.json`, `src/**`, `src-tauri/**`, `tools/check_a01.py`, `tools/check_a01b.py`, `tools/a01b_host_acceptance.py`, `docs/evidence/A01*.md` | **partially implemented (A01a+A01b harness); no compiled-runtime claim yet** | A01c |
| A02 explicit web↔desktop frontend transport adapter | none yet | **planned; blocked until A01c exit evidence** | A02 |
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

## Explicit non-claims after A01b

A Tauri/React shell **source foundation plus strict offline host-acceptance harness** now exists, but A01 has not yet earned a compiled/launchable-runtime claim because the A01b construction runner has neither Cargo/rustc nor the WebKitGTK development stack and cannot resolve external registries. There is still no application/domain transport, SQLite profile, XDG runtime, secret vault, Linux package, sync transport or durable offline user workflow. Those become real only when their stage changes status with corresponding code and checks.

# Implementation status

This ledger is normative for claims about the Arch-native repository. `implemented` means code/evidence required by that stage exists here and is covered by the named deterministic checks; architecture prose alone never counts as implementation.

| Decision / stage | Code, files, tests | Status | Next Axx |
|---|---|---|---|
| DEBT-A00-001 devctl commitless-repo rollback limitation | `docs/architecture/08-implementation-corrections-and-debt.md`; seeded-repo rollback validation | **planned tooling fix / operationally mitigated** | devctl future |
| A00 evidence baseline + protocol fixtures | `docs/architecture/**`, `docs/evidence/A00_EVIDENCE_BASELINE.md`, `evidence/**`, `fixtures/**`, `tools/check_a00.py` | **implemented** | A01 |
| ADR-001 Tauri 2 single user-session process model, no default listener/service | architecture only | **planned** | A01 |
| A01 Tauri 2 shell + packaged React/Vite asset | none yet | **planned** | A01 |
| A02 explicit web↔desktop frontend transport adapter | none yet | **planned** | A02 |
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

## Explicit non-claims after A00

There is no Tauri application, React bundle, Rust desktop core, SQLite profile, XDG runtime, secret vault, Linux packaging, sync transport or offline user workflow in this repository yet. Those become real only when their stage changes status with corresponding code and checks.

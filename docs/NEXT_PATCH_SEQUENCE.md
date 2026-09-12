# Exact next implementation sequence

A00 froze source/protocol evidence. A01 established the Tauri/React process/security shell. A02 established the explicit presentation transport seam and repeatable Git-less UTS verification. A03 established the Rust application/domain boundary. A04/A04b froze repository semantics. A05/A05b added the UTS-verified SQLite schema/migration/repository adapter.

**A06 is now implemented at source/test level:** XDG data/config/state/cache paths follow the baseline layout; the profile database and migration recovery artifacts live under `XDG_DATA_HOME/p2pkanban/profiles/<profile>/`; app-owned directories/files are private by default; single-writer ownership uses a kernel `flock` on the profile-directory inode; optional second-instance activation uses a fixed Unix datagram under a secure `XDG_RUNTIME_DIR`; missing runtime IPC degrades routing only, not writer exclusion. The native startup path acquires instance ownership before Tauri starts.

Run `python3 -B tools/uts_verify.py` in UserTestSpace (UTS). Cargo test/build remains authoritative for Rust API/compiler behavior, and the A06 post-build host probe launches the real binary twice to verify second-instance routing, no-runtime degradation and restart after primary exit. If Cargo cache preparation is missing, run once with `--allow-network` and return to offline verification afterward.

The exact next architecture patch is **A07 — minimal durable auth/workspace/board slice + vault interface**.

A07 exit target:

- create the first real application service that opens the default XDG SQLite profile under A06 writer ownership;
- define a vault interface before any durable refresh/device/board secret is introduced; use an in-memory/fake vault contract at this stage, not plaintext SQLite/localStorage;
- implement minimal workspace/board create/list/open semantics through application/repository boundaries;
- expose only typed Tauri commands through the existing A02 transport map;
- prove create/open board survives native close/reopen fully offline;
- keep cards/order/archive/delete/checklists for A08 and production Linux secret providers for A09.

Then, in order: **A07 minimal durable auth/workspace/board slice → A08 planner feature persistence → A09 production secret persistence**.

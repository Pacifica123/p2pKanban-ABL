# Exact next implementation sequence

A00 froze source/protocol evidence. A01 established the Tauri/React process/security shell. A02 established explicit typed presentation transport and repeatable Git-less UTS verification. A03 established the Rust application/domain boundary. A04/A04b froze repository semantics. A05/A05b added the UTS-verified SQLite schema/migration/repository adapter. A06 added UTS-verified XDG placement and single-writer instance control.

**A07 + A07b are now implemented at source/check level:** the default XDG profile is opened behind A06 writer ownership; schema v2 adds durable workspace/board titles with checksum-addressed migration and explicit no-downgrade reader/writer floor; `WorkspaceService` remains storage/platform independent; `SqliteWorkspaceCatalog` provides create/list/open persistence; packaged React UI exercises create workspace → create/open board through typed Tauri IPC; `SecretVault` exists before any durable auth/capability secret is introduced and the wired A07 provider is explicitly session-only.

The first A07 UTS run exposed two source compile defects: TypeScript narrowed `VaultStatus.durable` to the impossible single literal `'false'` for the existing future capability branch, and the Rust migration journal failed to escape literal JSON braces in `format!`. A07b corrects both; the reported `frontendDist` failure was cascading because the failed frontend build never created `dist/index.html`.

Run `python3 -B tools/uts_verify.py` in UserTestSpace. Cargo tests are authoritative for migration + close/reopen durability, frontend build checks the rendered slice, and the existing A06 runtime probes remain active. Use `--allow-network` only if dependency preparation reports an incomplete offline cache.

The exact next architecture patch is **A08 — cards/order/archive/delete/checklists durable planner slice**.

A08 exit target:

- connect the existing A04 card/order/tombstone semantics to the A07 opened board UI/application service;
- persist card create/move/archive/delete and checklist basics through SQLite transactions;
- preserve access-epoch/tombstone/version invariants and deterministic position ordering;
- keep pending sync/outbox semantics explicit rather than pretending local commit is remote convergence;
- extend the single canonical UTS verifier with crash/reopen and planner durability evidence.

Then, in order: **A08 planner feature persistence → A09 production secret persistence → A10 sync-core/roaming compatibility**.

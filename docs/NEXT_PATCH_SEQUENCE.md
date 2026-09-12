# Exact next implementation sequence

A00 froze source/protocol evidence. A01 established the Tauri/React process/security shell. A02 established the explicit presentation transport seam and the repeatable Git-less UTS verifier. A03 established the Rust application/domain boundary. A04/A04b froze repository semantics and are Cargo-green in UserTestSpace.

**A05 is now implemented at source/test level:** an exact-pinned bundled SQLite adapter implements the existing `PlannerRepository`; schema v1 carries explicit reader/writer metadata; file-backed profiles require foreign keys, WAL, `synchronous=FULL`, and bounded busy behavior; existing v0 profiles receive a pre-migration online backup + migration journal; integrity/foreign-key checks run before activation; forced-failure tests restore the original profile. The same A04 semantic scenario suite runs against SQLite, plus a close/reopen durability scenario.

Run `python3 -B tools/uts_verify.py`. If the new `rusqlite` sources are not yet present in the global Cargo cache, run the same command once with `--allow-network`, then return to the default offline command. Cargo test/build is the authority for A05 host verification.

The first A05 UTS compile exposed an API-shape defect: `rusqlite 0.40.2` no longer exports the `DatabaseName` type used by the initial adapter. **A05b** corrects backup/restore to the generic database-name API (`"main"`) without changing schema or recovery semantics. Repeat `python3 -B tools/uts_verify.py`; Cargo test/build is the authority for the correction.

The exact next architecture patch is **A06 — XDG adapters + multi-instance/locking behavior**.

A06 exit target:

- define XDG-compliant config/data/state/cache/runtime profile paths without assuming a desktop environment or systemd;
- move A05 temporary sidecar backup/journal placement into the final per-profile layout while preserving migration atomicity;
- introduce single-writer profile instance control with graceful stale-lock recovery while retaining SQLite locking as defense-in-depth;
- define upgrade/uninstall data-preservation ownership and expose profile diagnostics without requiring root;
- test concurrent launch, stale lock, missing runtime dir, read-only/unwritable paths, and reopen after abnormal termination;
- keep Secret Service/KWallet integration for A09 and durable application auth/board UI slice for A07.

Then, in order: **A06 XDG/instance control → A07 minimal durable auth/workspace/board slice → A08 planner feature persistence**.

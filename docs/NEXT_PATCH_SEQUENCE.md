# Exact next implementation sequence

A00 froze source/protocol evidence. A01 established the Tauri/React process and security shell. A02 established the explicit presentation transport seam, and A02b–A02d incorporated real Manjaro UserTestSpace build/runtime evidence and fixed the canonical verifier for repeatable Git-less snapshots.

**A03 is now implemented:** the health path terminates at a thin Tauri adapter backed by transport-agnostic Rust `ApplicationServices`/`SystemService` and platform-independent core values. No SQL, HTTP, filesystem, process or network dependency is allowed inside `application/` or `domain/`. This is intentionally a boundary patch, not a persistence port.

Host-dependent compilation/runtime evidence remains driven by the single canonical command `python3 -B tools/uts_verify.py`; lack of a particular tool in another execution environment does not justify weakening deterministic source contracts.

**A04 is implemented semantically, with an A04b compile correction:** source archaeology from the frozen web/backend + Android snapshots is encoded in `evidence/a04-repository-semantics.json`, and `PlannerRepository`/`PlannerTransaction` plus the reusable Rust contract scenarios freeze CRUD/order/tombstone/access-epoch/atomicity semantics without SQL or Tauri leakage. The first real UTS `cargo test` exposed that the helper `create()` was accidentally concrete while the runner was generic; A04b fixes that type boundary and records the correction. The position-allocation gap remains deliberately unfrozen because web uses 1024 while Android optimistic state uses 1000.

Run `python3 -B tools/uts_verify.py`; once Cargo test is green, the exact next architecture patch is **A05 — SQLite profile schema + atomic migration engine**.

A05 exit target:

- add the embedded SQLite adapter without changing A04 application/domain contracts;
- run the same A04 repository scenario suite against a temporary SQLite profile;
- introduce schema metadata, foreign keys, WAL/FULL policy and bounded busy behavior where supported;
- implement atomic migration/journal/backup-integrity behavior before writable activation;
- keep XDG path ownership and multi-instance policy for A06 rather than hard-coding `$HOME` paths into SQLite code;
- add persistence/reopen and migration interruption/integrity tests to the canonical UTS plan.

Then, in order: **A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

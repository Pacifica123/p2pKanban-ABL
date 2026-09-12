# Exact next implementation sequence

Historical ordering remains A00 → A01 → A02 → A03 → A04/A04b → A05/A05b → A06. These stages established evidence, native shell/transport, application/domain boundaries, repository contracts, SQLite migrations and XDG single-writer behavior. A07/A07b established the first durable workspace/board flow plus a session-only vault boundary; the user-reported canonical UTS rerun is green.

**A08 is now implemented at source/deterministic-check level.** Schema v3 adds durable column metadata, checklists/items, checklist/item tombstones, local logical-clock state and payload-free pending markers. The opened-board React flow now exercises column/card create, card move/archive/delete, checklist/item create/toggle/delete through explicit typed Tauri IPC into `PlannerService`, repository contracts and SQLite. Card semantics continue to reuse A04; SQL remains infrastructure-only.

The A08 pending table is deliberately **not** a sync protocol/outbox claim. It records only local mutation identity/sequence so local durability is explicit. A10 still owns remote payloads, relay/replay behavior, field/version merge and convergence. The observed web-vs-Android position-allocation gap is likewise not frozen as a domain constant.

Run `python3 -B tools/uts_verify.py` in UserTestSpace. The canonical verifier now includes `deterministic.a08`, the full Cargo suite and a filtered `host.a08-planner-durability` probe for A08 close/reopen, tombstone and forced-abort tests.

The exact next architecture patch is **A09 — production Linux secret persistence matrix**.

A09 exit target:

- implement the existing `SecretVault` boundary against Secret Service when available/unlocked;
- define and test locked/absent-provider behavior plus explicit session-only/passphrase fallback policy without plaintext SQLite/localStorage/config fallback;
- keep normal-use root/systemd/KWallet assumptions optional and capability-detected;
- add secret/log canaries and provider-state UTS evidence;
- do not begin A10 sync transport until durable secret handling is fail-closed.

Then, in order: **A09 secret persistence → A10 sync-core/roaming compatibility → A11 device-link/import migration**.

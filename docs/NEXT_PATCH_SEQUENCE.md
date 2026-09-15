# Exact next implementation sequence

Historical ordering remains A00 → A01 → A02 → A03 → A04/A04b → A05/A05b → A06. These stages established evidence, native shell/transport, application/domain boundaries, repository contracts, SQLite migrations and XDG single-writer behavior. A07/A07b established the first durable workspace/board flow plus a session-only vault boundary; the user-reported canonical UTS rerun is green.

**A08 is now implemented at source/deterministic-check level.** Schema v3 adds durable column metadata, checklists/items, checklist/item tombstones, local logical-clock state and payload-free pending markers. The opened-board React flow now exercises column/card create, card move/archive/delete, checklist/item create/toggle/delete through explicit typed Tauri IPC into `PlannerService`, repository contracts and SQLite. Card semantics continue to reuse A04; SQL remains infrastructure-only.

The A08 pending table is deliberately **not** a sync protocol/outbox claim. It records only local mutation identity/sequence so local durability is explicit. A10 still owns remote payloads, relay/replay behavior, field/version merge and convergence. The observed web-vs-Android position-allocation gap is likewise not frozen as a domain constant.

Run `python3 -B tools/uts_verify.py` in UserTestSpace. The canonical verifier now includes `deterministic.a08`, the full Cargo suite and a filtered `host.a08-planner-durability` probe for A08 close/reopen, tombstone and forced-abort tests.

**A09 is now implemented at source/deterministic-check level.** The existing `SecretVault` boundary now has a Linux Secret Service provider, an AEAD-encrypted typed profile vault, a versioned Argon2id passphrase provider and fail-closed session-only degradation for locked/absent/corrupt providers. No raw secret IPC or plaintext SQLite/localStorage/config fallback was added. Multi-host GNOME/KWallet/minimal-session evidence remains experiment-needed and is not inferred from one UTS host.

Run `python3 -B tools/uts_verify.py --allow-network` once if the new Rust crypto/keyring dependencies are not cached; acceptance is rechecked offline. Later runs can use the normal network-free command.

The exact next architecture patch is **A10 — sync-core + roaming compatibility**.

A10 exit target:

- introduce sync payload/version/merge logic behind application/domain boundaries, using the A00 roaming fixtures and A08 pending identities rather than treating the pending table as an outbox protocol;
- persist any refresh/device/board secret material only through `SecretVault`;
- implement deterministic replay/conflict/tombstone/order/capability-epoch tests before adding relay transport breadth;
- preserve web ↔ Android ↔ Linux compatibility and explicit schema/protocol negotiation;
- do not begin A11 device-link/import migration until local/fixture sync-core behavior is deterministic.

Then, in order: **A10 sync-core/roaming compatibility → A11 device-link/import migration → A12 parity surface**.

## A09b resolver correction

Canonical A09 UTS did not reach Cargo test/build because the final offline lock re-resolution rejected the yanked `chacha20 0.10.1` selected by `chacha20poly1305 0.11.0`. A09b pins `chacha20poly1305 0.10.1` instead and deliberately keeps the verifier fail-closed/offline.

Run `python3 -B tools/uts_verify.py --allow-network` once after A09b. A10 remains the exact next architecture patch only after this UTS rerun is green.

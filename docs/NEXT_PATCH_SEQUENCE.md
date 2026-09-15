# Exact next implementation sequence

A00→A09 are established; A09b removed the yanked AEAD line but fresh-host UTS exposed a second exact-getrandom resolver conflict. A10 carries that correction and is implemented at source/deterministic-check level.

## A10 implemented boundary

A10 adds `sync/1` and `roaming/1` domain validation/version ordering, Android-compatible board-tag/XChaCha crypto vectors, durable SQLite `sync_outbox`/seen-event/field-version state, local pending materialization, remote replay/conflict/tombstone merge, capability-epoch rejection, seed-only Android `board.snapshot` handling and a deterministic relay drop/reorder/replay convergence harness. It also aligns the A09 direct `getrandom` pin to 0.4.2 after fresh-host UTS proved exact 0.4.3 incompatible with final offline re-resolution. Board capability material crosses only the existing `SecretVault` boundary.

A08 `pending_local_changes` remains a local mutation-intent table, not a wire outbox. A10 materializes supported mutations into a separate protocol representation. Unsupported legacy roaming gaps such as post-snapshot `column.create` fail closed instead of being dropped or mapped to an invented protocol operation.

Run `python3 -B tools/uts_verify.py --allow-network` once if the new direct Rust dependencies are not already cached; final acceptance is still rechecked offline.

The exact next architecture patch after A10 UTS is green is **A11 — device-link/2 + web-node-link/bundle import migration**.

A11 exit target:

- consume the existing `p2p-kanban-device-link/2` capability/snapshot contract without exposing raw secrets to WebView;
- import/provision board/workspace state into the A07/A08/A10 native repositories atomically;
- use `SecretVault` for device/board secret material and preserve capability epoch;
- provide deterministic malformed/replay/downgrade/rollback tests against A00 device-link and portable-bundle fixtures;
- keep browser/Docker node-link assumptions outside the native runtime.

Then, in order: **A11 device-link/import migration → A12 parity surface → A13 lifecycle/integration capability detection**.

# Exact next implementation sequence

A00→A09 are established; A09b removed the yanked AEAD line but fresh-host UTS exposed a second exact-getrandom resolver conflict. A10 carries that correction and is implemented at source/deterministic-check level.

## A10 implemented boundary

A10 adds `sync/1` and `roaming/1` domain validation/version ordering, Android-compatible board-tag/XChaCha crypto vectors, durable SQLite `sync_outbox`/seen-event/field-version state, local pending materialization, remote replay/conflict/tombstone merge, capability-epoch rejection, seed-only Android `board.snapshot` handling and a deterministic relay drop/reorder/replay convergence harness. It also aligns the A09 direct `getrandom` pin to 0.4.2 after fresh-host UTS proved exact 0.4.3 incompatible with final offline re-resolution. Board capability material crosses only the existing `SecretVault` boundary.

A08 `pending_local_changes` remains a local mutation-intent table, not a wire outbox. A10 materializes supported mutations into a separate protocol representation. Unsupported legacy roaming gaps such as post-snapshot `column.create` fail closed instead of being dropped or mapped to an invented protocol operation.

Run `python3 -B tools/uts_verify.py --allow-network` once if the new direct Rust dependencies are not already cached; final acceptance is still rechecked offline.

A11 is now the implemented destination/import boundary for **device-link/2 + web-node-link/bundle migration**. Its canonical Cargo/offline UTS remains the acceptance gate; the A10 serde resolver prerequisite correction is intentionally absorbed here rather than split into A10b/A09c.

A11 implemented/acceptance target:

- consume the existing `p2p-kanban-device-link/2` capability/snapshot contract without exposing raw secrets to WebView;
- import/provision board/workspace state into the A07/A08/A10 native repositories atomically;
- use `SecretVault` for device/board secret material and preserve capability epoch;
- provide deterministic malformed/replay/downgrade/rollback tests against A00 device-link and portable-bundle fixtures;
- keep browser/Docker node-link assumptions outside the native runtime.

A11 and A12 canonical UTS are green. The current architecture patch is **A13 — lifecycle/integration capability detection**. After A13 canonical UTS is green, proceed to **A14 — optional bounded LAN compatibility bridge**.


## A11 implemented boundary

A11 adds schema v5 import receipts/principal/capability metadata, a storage-independent import plan, portable bundle v1 roundtrip/import, device-link/2 grant binding, and web-node-link v1 destination provisioning. New native device/replica identity is not copied from a legacy deployment; board/device secret bytes go only through `SecretVault`. Portable imports do not create `pending_local_changes`. Shared workspaces and node-local hides omitted by web-node-link v1 are surfaced in the import report.

The WebView still has no generic filesystem/keyring/network privilege. Live Nostr device-link event verification/chunk transport and legacy-node HTTP/session acquisition remain native transport adapters outside this A11 destination boundary and are not claimed as implemented.


## A12 implemented boundary

A12 promotes labels/card-label edges, comments, activity provenance and board appearance to first-class native schema/application/IPC/UI contracts. Label/comment mutations are durable but recorded separately as `roaming-v1-unsupported`; appearance alone reuses the proven A10 `board.appearance.put` path. A11 portable parity sections are materialized during fresh-profile import without generating local mutation markers. No new WebView filesystem/keyring/network privilege is introduced.


## A13 implemented boundary

A13 detects Wayland/X11 plus session D-Bus notification/StatusNotifier/portal capabilities, keeps tray/systemd background lifecycle disabled, and extends the A06 private activation socket with a validated bounded `deep-link-v1` message while preserving `activate-main-v1`. The WebView receives only typed capability/intents; no generic D-Bus/shell/filesystem privilege is exposed. Package-level custom-scheme registration is deferred to A15 and missing desktop services remain supported degraded mode.

After canonical A13 Cargo/runtime UTS is green, the exact next architecture patch is **A14 — optional bounded LAN compatibility bridge, off by default**.

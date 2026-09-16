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

A11 through A14 canonical UTS are green. The current architecture patch is **A15 — PKGBUILD + signed repository packaging**. After A15 canonical packaging/release acceptance is green, proceed to **A16 — backup/doctor/safe-mode/recovery**.


## A11 implemented boundary

A11 adds schema v5 import receipts/principal/capability metadata, a storage-independent import plan, portable bundle v1 roundtrip/import, device-link/2 grant binding, and web-node-link v1 destination provisioning. New native device/replica identity is not copied from a legacy deployment; board/device secret bytes go only through `SecretVault`. Portable imports do not create `pending_local_changes`. Shared workspaces and node-local hides omitted by web-node-link v1 are surfaced in the import report.

The WebView still has no generic filesystem/keyring/network privilege. Live Nostr device-link event verification/chunk transport and legacy-node HTTP/session acquisition remain native transport adapters outside this A11 destination boundary and are not claimed as implemented.


## A12 implemented boundary

A12 promotes labels/card-label edges, comments, activity provenance and board appearance to first-class native schema/application/IPC/UI contracts. Label/comment mutations are durable but recorded separately as `roaming-v1-unsupported`; appearance alone reuses the proven A10 `board.appearance.put` path. A11 portable parity sections are materialized during fresh-profile import without generating local mutation markers. No new WebView filesystem/keyring/network privilege is introduced.


## A13 implemented boundary

A13 detects Wayland/X11 plus session D-Bus notification/StatusNotifier/portal capabilities, keeps tray/systemd background lifecycle disabled, and extends the A06 private activation socket with a validated bounded `deep-link-v1` message while preserving `activate-main-v1`. The WebView receives only typed capability/intents; no generic D-Bus/shell/filesystem privilege is exposed. Package-level custom-scheme registration is deferred to A15 and missing desktop services remain supported degraded mode.

A13 and A14 canonical Cargo/runtime UTS are green. A15 is now the current implementation stage. After canonical A15 package preparation plus release acceptance is green, the exact next architecture patch is **A16 — backup/doctor/safe-mode/recovery**.


## A14 implemented boundary

A14 adds one explicit, short-lived LAN compatibility listener for device-link/portable provisioning. It binds only a user-selected detected private/link-local IPv4 address on a random high port, uses a 256-bit one-time XChaCha20-Poly1305 capability, allows one POST endpoint, closes after one authenticated request or TTL, and requires durable SecretVault storage. Fresh native device identity is generated locally; only its public key leaves the process. No generic planner REST API, mDNS, firewall change, background service or WebView network privilege is added.


## A15 implemented boundary

A15 introduces a pacman-owned Arch x86_64 packaging/release boundary without changing the application runtime, schema or protocol surface. A deterministic release-preparation tool binds the canonical UTS Cargo.lock into the retained source archive, resolves an exact PKGBUILD source checksum, and records lock/npm/migration/protocol digests. The package owns only `/usr/bin` plus desktop/icon/license metadata; no install/remove hook touches XDG data, keyrings, services or package-manager configuration.

Package-level `x-scheme-handler/p2pkanban` registration now routes `%u` to the A13 validated argv/single-instance path. Repository staging signs package and pacman DB through an external GPG fingerprint/agent and retains hashes/fingerprint metadata; no private signing material enters the repository. Public AUR/mirror publication remains blocked until the owner supplies a software license. Clean-chroot/package namcap/pacman-Qkk/uninstall and real-key publication remain explicit A15 release evidence.

After A15 acceptance, the next stage is **A16 — backup/doctor/safe-mode/recovery**.

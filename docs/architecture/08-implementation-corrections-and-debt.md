# 08 — Implementation corrections and debt ledger

This file records implementation-time evidence that narrows or corrects the imported blueprint. Corrections are explicit; they must not be hidden in code.

## DEBT-A00-001 — devctl v0.7.0 cannot auto-reset a partially applied patch in a commitless repository

**Classification:** [FACT] from attached `devctl.py` plus an A00 dry application test.  
**Architecture impact:** delivery/precondition only; no native runtime ADR is changed.  
**Status:** open tooling debt; safe operational precondition defined.

The attached devctl v0.7.0 rollback path ultimately uses `git reset --hard HEAD`. In a Git repository with no commits, `HEAD` is unborn. If a patch has already copied files and then fails, the normal auto-reset cannot resolve `HEAD` and therefore cannot provide the advertised atomic rollback.

### A00 policy

The dedicated `p2pkanban-archlinux-native` target repository must have:

1. an existing seed commit on `main` before the first devctl patch is applied;
2. that seed contains `ARCH_NATIVE_REPO` with exactly `p2pkanban-archlinux-native\n`; and
3. normal Git author identity (`user.name` + `user.email`) configured so devctl can create its commit.

The sentinel is deliberately more specific than `README.md`: devctl Patch Intake uses it as target evidence, reducing the chance that A00 is routed to the existing web/Docker repository. A00 carries the same sentinel forward as a permanent repository identity marker.

### Why this is not hidden in `.devctl`

The patch must not change workspace policy or devctl itself. Therefore A00 records the limitation in the repository SSOT and tests rollback against a seeded repository. A future devctl release may remove this precondition by handling an unborn branch explicitly; when that happens this debt can be closed without changing the desktop runtime architecture.

## DEBT-A01-001 — A01 build evidence requires a provisioned Linux host; generic devctl application must remain offline-safe

**Classification:** [FACT] from the A01b construction environment and upstream Tauri Linux prerequisites.  
**Architecture impact:** validation/environment only; ADR-001 remains unchanged.  
**Status:** open acceptance evidence, bounded to A01c.

The patch-construction environment does not contain Cargo/rustc or WebKitGTK development metadata and cannot reach external package/registry endpoints. Installing a hidden toolchain during `devctl start`, weakening checks to a source grep, or fabricating `Cargo.lock` would all violate the repository's evidence rules and unstable-network constraint.

A01b therefore adds a fail-closed offline host harness. Dependency/cache population is an explicit build-time preparation activity outside `devctl start`; normal deterministic patch checks remain network-free. A01c must execute the strict harness and real WebView runtime probes on a provisioned Arch-family host before A01 can become `implemented`.

## DEBT-A01-002 — A01a recorded an obsolete/incorrect Tauri Rust floor

**Classification:** [FACT] from current upstream Tauri 2.11.5 workspace metadata, re-checked during A01c.  
**Architecture impact:** build compatibility/evidence; ADR-001 process model is unchanged.  
**Status:** source declaration corrected in A01c; host verification remains open until A01d.

A01a recorded `rust-version = 1.77.2` and described that value as the selected Tauri release's Rust floor. Current upstream Tauri 2.11.5 workspace metadata declares **Rust 1.90** via `rust-version = 1.90`, and the `tauri` 2.11.5 crate inherits that workspace value. A01c therefore changes the native package declaration to `1.90` and supersedes the A01a compatibility claim.

This correction does **not** claim that 1.90 is the final reproducible build toolchain. The exact toolchain pin must be chosen from a successful A01d Arch-host build and cached/prepared explicitly so offline builds do not unexpectedly invoke rustup/network access.

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
**Status:** source declaration corrected in A01c; host verification remains open in the canonical A02b UTS pipeline.

A01a recorded `rust-version = 1.77.2` and described that value as the selected Tauri release's Rust floor. Current upstream Tauri 2.11.5 workspace metadata declares **Rust 1.90** via `rust-version = 1.90`, and the `tauri` 2.11.5 crate inherits that workspace value. A01c therefore changes the native package declaration to `1.90` and supersedes the A01a compatibility claim.

This correction does **not** claim that 1.90 is the final reproducible build toolchain. The exact toolchain pin must be chosen from a successful Arch-family UTS build and cached/prepared explicitly so offline builds do not unexpectedly invoke rustup/network access.

## DELIVERY-A01-003 — host-dependent verification delegated to UserTestSpace

**Status:** accepted delivery-process correction; does not convert missing evidence into a pass.

The patch-construction environment may lack Cargo/Rust, Arch WebKitGTK development packages, a graphical Wayland/X11 session, or a populated offline npm/Cargo cache. Starting with A02, those unavailable host-dependent checks are explicitly delegated to UserTestSpace (UTS) when the deterministic source/contracts for the patch are green.

This changes the **blocking policy**, not the evidence claim: A01/A02 host results remain `verification-pending` until the documented UTS commands pass. Dependent source stages may proceed when they do not rely on the unverified runtime property. Any UTS failure re-opens the affected Axx stage and must be corrected before release/packaging claims. Generic devctl checks remain offline/network-independent and never turn a missing toolchain into a fake success.

## CORR-A02-004 — Tauri context required an application icon that A01/A02 omitted

**Classification:** [FACT] from the first real UTS `cargo test --locked --offline` on the A02 snapshot.  
**Architecture impact:** packaging/build correctness only; no process, IPC, persistence or privilege ADR changes.  
**Status:** fixed in A02b; post-fix UTS compile/launch verification pending.

`tauri::generate_context!()` failed before native tests could run because `src-tauri/icons/icon.png` was absent. This was a repository defect, not an unavailable-host limitation. A02b adds a repository-owned RGBA PNG and makes the icon path explicit in `tauri.conf.json` so the asset is part of the checked source contract rather than an undocumented Tauri default.

## CORR-A02-005 — Node `<23` build-time upper bound was unsupported by evidence

**Classification:** [FACT] from the same UTS run: Node 26.8.1 completed `npm ci`, TypeScript typecheck and Vite production build, while npm only warned about the repository's declared engine range.  
**Architecture impact:** build tooling only; the packaged desktop runtime still has no Node dependency.  
**Status:** corrected in A02b.

The earlier `>=20 <23` range was a conservative assumption without an implementation-driving compatibility reason. A02b removes the artificial upper bound and retains `>=20`. Future incompatibility must be based on an actual toolchain failure or upstream requirement, not an arbitrary major-version ceiling.

## DELIVERY-A02-006 — single-entry UTS verifier replaces per-patch manual command lists

**Classification:** [PROPOSAL implemented as tooling contract], motivated by repeated host-only verification steps.  
**Architecture impact:** verification workflow only.  
**Status:** implemented in A02b.

`tools/uts_verify.py` is now the stable UTS entry point and `tools/uts_plan.json` is the evolving machine-readable plan. Patches update the plan as host checks change. The verifier is offline by default, may populate caches only with explicit `--allow-network`, continues through independent checks, and stores text/JSON/log evidence under ignored `.uts-reports/`.

This does not weaken `devctl`: generic patch checks remain deterministic/network-free, while host/toolchain/runtime evidence remains explicit and separately attributable.

## CORR-A02-007 — deterministic hygiene confused repository content with ignored UTS build state

**Classification:** [FACT] from two consecutive real UTS verifier runs after A02b.  
**Architecture impact:** verification correctness only; no runtime ADR changes.  
**Status:** fixed in A02c.

The first post-A02b UTS run passed completely and intentionally left ignored `dist/`, `node_modules/`, `src-tauri/target/` and a resolver-generated Cargo lock in the working tree. The immediate second invocation then failed A00 because `dist/` merely existed, caused A01 to recursively scan generated Cargo output until the process was killed, and caused A01b to reject the current Cargo lock because it hard-coded serialization format `version = 3`.

Deterministic source gates must answer “is forbidden/generated state owned by the repository?” rather than “has this developer ever built the project?”. A02c therefore derives hygiene from `git ls-files`, scans only tracked source for forbidden runtime assumptions, and accepts resolver-generated Cargo lock formats 3/4. Ignored caches remain available for incremental offline UTS work.

## DELIVERY-A02-008 — one UTS invocation must prove post-build repeatability

**Classification:** [PROPOSAL implemented as verification tooling].  
**Architecture impact:** verification workflow only.  
**Status:** implemented in A02c.

The canonical verifier now reruns the entire deterministic gate list after frontend/Cargo/runtime steps. This makes state-pollution bugs observable during the same invocation that created the state. The verifier does not use `git clean`, resets, or cache deletion to manufacture a pass.

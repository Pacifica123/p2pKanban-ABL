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

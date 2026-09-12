# UserTestSpace verification — canonical entry point

This file is the SSOT for host/toolchain/runtime verification that cannot be honestly executed by generic `devctl start`.

## Normal command

From the project root inside UserTestSpace:

```bash
python3 -B tools/uts_verify.py
```

The verifier is **network-free by default**. It runs every deterministic Axx gate, host doctors, cached frontend preparation/build, Cargo lock/fetch/test/build, and (when the native binary builds) the Linux runtime probe. It then reruns the deterministic gates **after** those generated artifacts exist, proving repeatability without deleting useful caches. It continues through independent checks and writes all results under:

```text
.uts-reports/<UTC timestamp>/
├── summary.txt
├── results.json
└── logs/
```

Stable pointers are also written as `.uts-reports/latest-summary.txt` and `.uts-reports/latest-results.json`. The directory is ignored by Git and must not be included in devctl patches.

If dependency caches are incomplete, rerun explicitly allowing **build-time cache preparation**:

```bash
python3 -B tools/uts_verify.py --allow-network
```

`--allow-network` is never implicit. The verifier may use npm/crates.io only for cache/lock preparation and then rechecks the acceptance path offline. It never uses `sudo`, `pacman`, `systemctl`, Docker, or edits `.devctl`/workspace policy.

For a headless/remote session where GUI evidence is impossible:

```bash
python3 -B tools/uts_verify.py --no-runtime
```

That is not runtime evidence; the summary records the runtime probe as skipped.

## Current A02c manual evidence

After a successful automatic run, keep the native window open long enough to confirm:

```text
mode: desktop
p2pkanban-arch-native 0.1.0 · ok · desktop
```

This visual check is currently the remaining WebView→Rust IPC observation. It must not be silently promoted to an automated pass.

## How subsequent patches change verification

`tools/uts_plan.json` is the machine-readable verification plan. Each Axx patch updates that plan when it adds/removes host checks. `tools/uts_verify.py` is the stable entry point, so the user does not need to maintain a growing command list manually.

A failing UTS check re-opens the stage whose property failed. `postbuild.deterministic.*` failures specifically mean a source contract is not repeatable in a built workspace. Send `.uts-reports/latest-summary.txt` plus the referenced failing `logs/*.log`; raw caches, `node_modules`, `target`, `dist`, secrets and the entire report directory are not patch inputs. A resolver-generated `src-tauri/Cargo.lock` is source provenance rather than a cache; if a later patch is to commit it, provide that exact file explicitly rather than reconstructing it from logs.

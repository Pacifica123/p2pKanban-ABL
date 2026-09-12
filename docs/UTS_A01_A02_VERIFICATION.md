# UserTestSpace verification — A01 + A02

Run from the repository root on the Arch/EndeavourOS/Manjaro-family UTS host as a **normal user**. Do not use `sudo` to launch the app.

## 1. Deterministic repository gates

```bash
python3 -B tools/check_a00.py
python3 -B tools/check_a01.py
python3 -B tools/check_a01b.py
python3 -B tools/check_a01c.py
python3 -B tools/check_a02.py
```

Expected: every command ends in `OK`.

## 2. Materialize/verify JavaScript dependencies and frontend

If the npm cache is already prepared for offline use:

```bash
npm ci --offline --ignore-scripts --no-audit --no-fund
npm run typecheck
npm run build
```

If the cache is not prepared, perform `npm ci --ignore-scripts --no-audit --no-fund` once on an approved network/mirror, retain the cache outside the repository, then repeat the three commands above with `--offline` for acceptance.

Expected: typecheck/build exit 0; `dist/index.html` exists. Do **not** commit `node_modules/` or `dist/`.

## 3. Materialize Cargo lock once, then prove locked/offline build

On an approved network/mirror only for lock/cache preparation:

```bash
cargo generate-lockfile --manifest-path src-tauri/Cargo.toml
cargo fetch --manifest-path src-tauri/Cargo.toml --locked
```

Review and commit only `src-tauri/Cargo.lock` in the follow-up evidence patch. Then disconnect external network (or enforce your UTS offline policy) and run:

```bash
python3 -B tools/a01b_host_acceptance.py doctor
python3 -B tools/a01b_host_acceptance.py offline-build
```

Expected: `cargo test --locked --offline` and `cargo build --locked --offline` pass and `src-tauri/target/debug/p2pkanban-arch-native` exists. Do **not** commit `target/`.

## 4. A01 runtime evidence

In a real Wayland or X11 desktop session:

```bash
python3 -B tools/a01c_host_runtime_probe.py doctor
python3 -B tools/a01c_host_runtime_probe.py launch-probe --seconds 8 --report /tmp/p2pkanban-a01-runtime.json
```

Expected: non-root launch; isolated XDG dirs; no owned TCP listener; no Node/PostgreSQL/Docker/systemd runtime process; no `libpq`/`libnode` linkage; secret canary absent from captured logs.

## 5. A02 typed IPC smoke

Launch the debug binary produced above:

```bash
./src-tauri/target/debug/p2pkanban-arch-native
```

Expected in the window:

- `mode: desktop`;
- `p2pkanban-arch-native 0.1.0 · ok · desktop`;
- no `probe pending/failed` message.

With the app running, confirm no localhost listener was introduced:

```bash
ss -ltnp | grep -i p2pkanban && echo 'UNEXPECTED LISTENER' || echo 'OK: no p2pKanban TCP listener'
```

For a negative IPC check, temporarily change the desktop route in a disposable UTS copy from `/health` to `/not-allowed`, rebuild the frontend, and confirm the UI reports `DESKTOP_ROUTE_UNSUPPORTED` rather than issuing HTTP or invoking an arbitrary Rust command. Revert the disposable change afterwards.

## Report back

Send the command output plus `/tmp/p2pkanban-a01-runtime.json` (after checking it contains no local sensitive paths you do not want to share). A later evidence-only patch can record the verified host/package/session matrix without committing build artifacts.

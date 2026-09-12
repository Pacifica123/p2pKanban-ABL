# Exact next implementation sequence

A01a established the permanent Tauri/React shell source/security boundary. A01b adds the strict, fail-closed offline build and Arch host acceptance harness without fabricating unavailable compile evidence.

The exact next patch is **A01c — resolver lock + Arch host build/launch evidence**.

A01c exit target:

- generate and review `src-tauri/Cargo.lock` with Cargo for the exact pinned Tauri dependency set;
- prepare approved npm/Cargo caches as a separate build-time step, then disconnect external network;
- pass `python3 -B tools/a01b_host_acceptance.py offline-build`;
- launch the built binary from packaged React/Vite assets;
- verify real WebView denial of hostile HTTP/HTTPS/file/data/javascript navigation, `window.open`, and downloads;
- verify runtime has no Docker/PostgreSQL/Node requirement, listener, root requirement, or mandatory system/user service;
- record Arch-family WebKitGTK/GTK/GLib/session evidence without claiming the full Wayland/X11 matrix.

Only after A01c is green may A01 be marked **implemented** and the architecture advance to **A02 — explicit web↔desktop frontend transport adapter**.

Then, in order: **A02 transport adapter → A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

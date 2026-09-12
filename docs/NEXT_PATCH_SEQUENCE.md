# Exact next implementation sequence

The next patch is **A01 — Tauri 2 shell + packaged React/Vite asset**.

A01 exit target:

- dedicated Tauri 2 desktop shell on Linux using system WebKitGTK;
- presentation assets are packaged/local and render with network unavailable;
- no Docker/PostgreSQL/Node requirement at **runtime**;
- no localhost backend or listening socket introduced;
- remote/top-level WebView navigation is denied by explicit policy;
- WebView has no arbitrary filesystem/process/network privilege;
- frontend typecheck/build and Rust/Tauri build checks are real when build dependencies are present;
- environment-dependent Wayland/X11 package smoke is recorded separately from deterministic source gates;
- no persistence/auth/sync claims yet.

Then, in order: **A02 transport adapter → A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

Do not combine A03–A07 merely to demonstrate a board early; that would erase the repository and migration rollback boundaries the architecture was designed to preserve.

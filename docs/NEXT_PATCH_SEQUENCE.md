# Exact next implementation sequence

A01 is being delivered as a bounded two-patch stage rather than faking build evidence. **A01a — shell source/security foundation** is present after the current patch.

The exact next patch is **A01b — locked offline build + Arch launch/security evidence**.

A01b exit target:

- review and commit a resolver-generated `src-tauri/Cargo.lock` for the exact Tauri dependency set;
- `npm ci --offline` from an approved local cache/mirror, then real frontend typecheck/build;
- `cargo test --locked --offline` and `cargo build --locked --offline` from an approved Cargo cache/vendor source;
- packaged React/Vite assets are embedded/available to the Tauri binary;
- built application launches with external network unavailable;
- hostile HTTP/HTTPS/file/data/javascript top-level navigation and `window.open` are denied;
- normal runtime has no Docker/PostgreSQL/Node requirement, localhost listener or mandatory system/user service;
- record Arch-family WebKitGTK/GTK/glibc/session evidence separately from deterministic source gates.

Only after A01b is green does the architecture advance to **A02 — explicit web↔desktop frontend transport adapter**.

Then, in order: **A02 transport adapter → A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

Do not combine A03–A07 merely to demonstrate a board early; that would erase the repository and migration rollback boundaries the architecture was designed to preserve.

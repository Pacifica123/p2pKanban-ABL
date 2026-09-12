# Exact next implementation sequence

A00 is complete. A01a–A01c established the permanent Tauri/React source/security boundary, offline build harness and Linux runtime probe. Per DELIVERY-A01-003, unavailable Cargo/Arch GUI evidence is now delegated to UserTestSpace and remains explicitly verification-pending rather than blocking source-stage progression.

A02 now establishes the explicit presentation transport seam:

- `apiRequest` remains the presentation-facing facade where practical;
- web mode owns HTTP/fetch behavior;
- desktop mode maps only explicit route/method pairs to named Tauri commands;
- the first route is `GET /health` → `desktop_api_health`;
- no localhost fallback, arbitrary command dispatcher, filesystem/process/network plugin or new capability permission exists.

The exact next patch is **A03 — Rust application/domain boundary**.

A03 exit target:

- introduce an application service module independent of Tauri/Axum/SQL;
- move `desktop_api_health` behind that application boundary as the first end-to-end example;
- define command DTO/error mapping conventions without importing PostgreSQL/SQLite concerns;
- add deterministic tests proving application APIs contain no `sqlx::Pg*`, HTTP, Tauri or filesystem/network dependencies;
- keep A01/A02 UTS commands in `docs/UTS_A01_A02_VERIFICATION.md`; a UTS failure re-opens the affected stage.

Then, in order: **A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

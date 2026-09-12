# Exact next implementation sequence

A00 is complete. A01a–A01c established the permanent Tauri/React source/security boundary and host probes. A02 established the typed presentation transport seam. The first real UTS native build then found a concrete source/package defect: Tauri context expected `src-tauri/icons/icon.png`. **A02b** fixes that defect, removes the unsupported Node `<23` build-time cap, and makes `tools/uts_verify.py` + `tools/uts_plan.json` the canonical evolving UserTestSpace verification pipeline.

The UTS verifier must be run after applying A02b. A new compiler/runtime failure is evidence and re-opens the affected stage; it is not a reason to hide or weaken the gate. Host-dependent evidence may remain verification-pending without blocking unrelated source-stage work under DELIVERY-A01-003.

The exact next architecture patch is **A03 — Rust application/domain boundary**.

A03 exit target:

- introduce an application service module independent of Tauri/Axum/SQL;
- move `desktop_api_health` behind that application boundary as the first end-to-end example;
- define command DTO/error mapping conventions without importing PostgreSQL/SQLite concerns;
- add deterministic tests proving application APIs contain no `sqlx::Pg*`, HTTP, Tauri or filesystem/network dependencies;
- append A03 checks to `tools/uts_plan.json` rather than creating a new manual UTS command list.

Then, in order: **A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

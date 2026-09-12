# Exact next implementation sequence

A01a established the permanent Tauri/React shell source/security boundary. A01b added the fail-closed offline build harness. A01c corrects the selected Tauri 2.11.5 Rust floor to 1.90 and adds a non-root Linux runtime evidence collector for XDG isolation, process-tree/runtime-dependency leakage, TCP listeners and log secret canaries.

A01 remains **partially implemented** because this patch-construction environment cannot resolver-generate the project lock or execute an Arch graphical WebView build honestly.

The exact next patch is **A01d — materialized Cargo lock + Arch build/WebView runtime evidence**.

A01d exit target:

- resolver-generate/review `src-tauri/Cargo.lock` on the exact repository manifest;
- record the exact Rust toolchain that actually passes the Arch-family build, then pin it without introducing an implicit network requirement;
- prepare approved npm/Cargo caches outside generic `devctl start`, disconnect external network, then pass `python3 -B tools/a01b_host_acceptance.py offline-build`;
- launch the built binary as a non-root user in a real Wayland or X11 session and pass `python3 -B tools/a01c_host_runtime_probe.py launch-probe`;
- prove real WebView denial of hostile HTTP/HTTPS/file/data/javascript navigation, new windows and downloads;
- retain host/package/WebKitGTK/GTK/GLib/session evidence without claiming the untested display backend or Arch ARM.

Only after A01d is green may A01 be marked **implemented** and the architecture advance to **A02 — explicit web↔desktop frontend transport adapter**.

Then, in order: **A02 transport adapter → A03 application/domain boundary → A04 repository contracts → A05 SQLite → A06 XDG/instance control → A07 minimal durable auth/workspace/board slice**.

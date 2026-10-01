# A17: AppImage fallback and signed offline installation/recovery kit

Base: supplied snapshot `e88af2c`.

Implements the A17 packaging/recovery channel from the accepted Arch roadmap. Adds an opt-in Tauri AppImage overlay, immutable A15-source build wrapper, release kit staging through an external GPG agent, independently pinned signature/version/replay verification, and private FUSE or explicit extract/run. The kit retains source/lock/package metadata and A16 recovery docs. XDG profile, flock, schema-v6, IPC/CSP and live transport behavior are unchanged.

All inherited A00–A16 offline gates plus A17 are checked. A16 checker now accepts later stages while retaining its recovery assertions; shared evidence hashes are advanced to the new docs/UTS ledger. Test fixture contains public synthetic signed data encoded in JSON, never a delivered binary.

Real AppImage bundling, release-agent signing, FUSE/extract/WebKitGTK, distribution ABI and offline GUI acceptance remain explicit host/release gates. A17 does not implement the deferred live ABL relay coordinator. Next implementation is A18 after that acceptance.

Rollback: devctl source rollback; an app/package downgrade alone does not roll back a profile. Use verified A16 backups and explicit restore. No schema or wire-protocol migration.

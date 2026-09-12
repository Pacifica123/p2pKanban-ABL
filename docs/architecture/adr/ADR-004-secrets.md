# ADR-004 — Arch secret storage and at-rest claims

**Status:** proposed  
**Decision:** protect a random application vault root through freedesktop Secret Service when usable; otherwise require an explicit Argon2id-derived passphrase vault or session-only credentials. Encrypt typed secret records with authenticated encryption. Do not claim full planner DB encryption in v1.

## Grounds
- [FACT] Android already separates memory-only access token from securely persisted refresh/capability secrets.
- [FACT] Linux desktop environments vary and may have Secret Service provider present, locked or absent.
- [INFERENCE] A provider abstraction plus explicit secure fallback preserves usability without silently downgrading secrecy.

## Rejected alternatives
- plaintext config/SQLite/localStorage;
- hard dependency on one GNOME/KDE keyring implementation;
- automatically configuring PAM/keyring daemons;
- storing every application object directly in Secret Service;
- promising SQLCipher/full-database encryption without a separate threat/performance decision.

## Costs
Multiple unlock/recovery UX paths; provider matrix testing; passphrase KDF/version/rotation handling.

## Failure modes
Provider locked/disappears; lost passphrase; corrupted vault root record; same-user malware; secret leakage in diagnostics.

## Verification
GNOME/KDE/minimal session matrix; wrong-passphrase/corruption fail-closed tests; provider migration/restart; secret canary scan across DB/config/log/crash bundle; explicit at-rest security wording review.

#!/usr/bin/env python3
"""Repository-view helpers for deterministic gates.

Deterministic checks validate repository-owned content, not ignored build/runtime
artifacts that UserTestSpace intentionally creates. devctl itself is Git-based,
so `git ls-files` is the canonical source of tracked paths.
"""
from __future__ import annotations

import subprocess
from pathlib import Path

GENERATED_PREFIXES = (
    "node_modules/",
    "dist/",
    "target/",
    "src-tauri/target/",
    "src-tauri/gen/",
    ".uts-reports/",
    "coverage/",
    "build/",
)


def tracked_relative_paths(root: Path) -> list[str]:
    proc = subprocess.run(
        ["git", "-C", str(root), "ls-files", "-z"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        message = proc.stderr.decode("utf-8", errors="replace").strip()
        raise RuntimeError(f"git ls-files failed: {message or proc.returncode}")
    return [item.decode("utf-8", errors="strict") for item in proc.stdout.split(b"\0") if item]


def tracked_files(root: Path, *prefixes: str) -> list[Path]:
    normalized = tuple(prefix.rstrip("/") + "/" for prefix in prefixes)
    result: list[Path] = []
    for rel in tracked_relative_paths(root):
        if normalized and not any(rel.startswith(prefix) for prefix in normalized):
            continue
        path = root / rel
        if path.is_file():
            result.append(path)
    return result


def generated_tracked_paths(root: Path) -> list[str]:
    bad: list[str] = []
    for rel in tracked_relative_paths(root):
        if any(rel == prefix.rstrip("/") or rel.startswith(prefix) for prefix in GENERATED_PREFIXES):
            bad.append(rel)
    return bad

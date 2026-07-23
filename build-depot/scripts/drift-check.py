#!/usr/bin/env python3
"""
drift-check.py — boundary registry drift checker

Reads the canonical boundary registry at KB/04-architecture/current-system-map.md
and compares its claims against:

1. Per-project README "## Boundary" blockquotes — must match the registry's
   blockquote verbatim (after whitespace normalization).
2. Per-project Cargo.toml workspace versions — should match the version stated
   in the registry's anchor.
3. Forge-templates floor versions — flagged when more than a minor or two behind
   actual workspace head.

Output: severity-grouped report to stdout. Exit code:
  0 — no drift
  1 — warnings only
  2 — critical drift (or warnings + --strict)

Usage:
  python3 KB/scripts/drift-check.py
  python3 KB/scripts/drift-check.py --strict      # treat warnings as critical
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
REGISTRY_PATH = REPO_ROOT / "KB" / "04-architecture" / "current-system-map.md"
FORGE_ENG_README = REPO_ROOT / "forge-templates" / "converge-engagement" / "README.md"
FORGE_ENG_ARCHIVED = REPO_ROOT / "forge-templates" / "converge-engagement" / "_ARCHIVED.md"

# Threshold (minor-version distance) above which a forge-templates floor is
# treated as critical rather than warning.
FORGE_FLOOR_CRITICAL_DRIFT = 3


findings_critical: list[str] = []
findings_warning: list[str] = []
findings_info: list[str] = []


@dataclass
class Anchor:
    name: str                    # H3 title, e.g. "Converge"
    blockquote: str              # whitespace-normalized canonical claim
    home: str | None = None      # e.g. "bedrock-platform/converge/"
    version: str | None = None   # raw version line from registry


def normalize_whitespace(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def extract_blockquote_after(text: str) -> str | None:
    """Extract contiguous `> `-prefixed lines, skipping leading blanks. Returns
    whitespace-normalized text, or None if no blockquote is found before a
    non-blank, non-blockquote line."""
    lines = []
    in_bq = False
    for raw in text.splitlines():
        stripped = raw.rstrip()
        if stripped.startswith(">"):
            in_bq = True
            lines.append(re.sub(r"^>\s?", "", stripped))
        elif in_bq:
            break
        elif stripped == "":
            continue
        else:
            return None
    if not lines:
        return None
    return normalize_whitespace("\n".join(lines))


def parse_registry() -> list[Anchor]:
    if not REGISTRY_PATH.exists():
        findings_critical.append(f"registry missing: {REGISTRY_PATH}")
        return []

    text = REGISTRY_PATH.read_text()
    m = re.search(r"^## Project Boundary Anchors\s*$", text, re.MULTILINE)
    if not m:
        findings_critical.append(
            "registry missing '## Project Boundary Anchors' section"
        )
        return []

    section = text[m.end():]
    h3_blocks = re.split(r"^### ", section, flags=re.MULTILINE)
    anchors: list[Anchor] = []
    for blk in h3_blocks[1:]:  # first chunk is preamble before the first H3
        lines = blk.splitlines()
        if not lines:
            continue
        title = lines[0].strip()
        body = "\n".join(lines[1:])

        bq = extract_blockquote_after(body)
        if bq is None:
            findings_warning.append(
                f"registry anchor '{title}' has no blockquote (canonical claim missing)"
            )
            continue

        home_match = re.search(r"^- Home:\s*`([^`]+)`", body, re.MULTILINE)
        ver_match = re.search(r"^- Version:\s*([^\n]+)", body, re.MULTILINE)

        anchors.append(Anchor(
            name=title,
            blockquote=bq,
            home=home_match.group(1).strip() if home_match else None,
            version=ver_match.group(1).strip() if ver_match else None,
        ))
    return anchors


def extract_readme_boundary(readme_path: Path) -> str | None:
    if not readme_path.exists():
        return None
    text = readme_path.read_text()
    m = re.search(r"^## Boundary\s*$", text, re.MULTILINE)
    if not m:
        return None
    return extract_blockquote_after(text[m.end():])


def read_cargo_version(repo_path: Path) -> str | None:
    """Return version from [workspace.package] or fallback to [package]."""
    cargo = repo_path / "Cargo.toml"
    if not cargo.exists():
        return None
    text = cargo.read_text()
    m = re.search(
        r"\[workspace\.package\][^\[]*?version\s*=\s*\"([^\"]+)\"",
        text, re.DOTALL,
    )
    if m:
        return m.group(1)
    m = re.search(
        r"\[package\][^\[]*?version\s*=\s*\"([^\"]+)\"",
        text, re.DOTALL,
    )
    if m:
        return m.group(1)
    return None


def check_readme(anchor: Anchor) -> None:
    if not anchor.home:
        findings_info.append(f"{anchor.name}: no Home in registry; skipping README check")
        return
    home_clean = anchor.home.rstrip("/")
    readme = REPO_ROOT / home_clean / "README.md"
    if not readme.exists():
        findings_warning.append(
            f"{anchor.name}: README missing at {readme.relative_to(REPO_ROOT)}"
        )
        return
    bq = extract_readme_boundary(readme)
    if bq is None:
        findings_warning.append(
            f"{anchor.name}: README at {readme.relative_to(REPO_ROOT)} has no '## Boundary' block"
        )
        return
    if bq == anchor.blockquote:
        findings_info.append(f"{anchor.name}: README Boundary matches registry ✓")
    else:
        findings_critical.append(
            f"{anchor.name}: README Boundary DRIFTS from registry\n"
            f"      registry: {anchor.blockquote[:140]}{'…' if len(anchor.blockquote) > 140 else ''}\n"
            f"      readme:   {bq[:140]}{'…' if len(bq) > 140 else ''}"
        )


def check_version(anchor: Anchor) -> None:
    if not anchor.home or not anchor.version:
        return
    home_clean = anchor.home.rstrip("/")
    actual = read_cargo_version(REPO_ROOT / home_clean)
    if actual is None:
        return  # silent — not all repos have Cargo.toml
    bare = re.search(r"v?(\d+\.\d+\.\d+)", anchor.version)
    if not bare:
        findings_info.append(
            f"{anchor.name}: registry version '{anchor.version}' has no semver; skipping"
        )
        return
    reg_v = bare.group(1)
    if reg_v == actual:
        findings_info.append(f"{anchor.name}: version matches Cargo.toml (v{actual}) ✓")
    else:
        findings_warning.append(
            f"{anchor.name}: registry says v{reg_v}, Cargo.toml says v{actual} "
            f"({home_clean}/Cargo.toml)"
        )


def check_forge_floors(anchors: list[Anchor]) -> None:
    if FORGE_ENG_ARCHIVED.exists():
        findings_info.append(
            "forge-templates/converge-engagement is archived "
            "(see 2026-06-07-retire-engagement-template ADR); skipping floor check"
        )
        return
    if not FORGE_ENG_README.exists():
        return
    text = FORGE_ENG_README.read_text()
    floor_rows = re.findall(
        r"^\|\s*([A-Za-z][^|]*?)\s*\|[^|]+\|\s*(\d+\.\d+\.\d+)\s*\|",
        text, re.MULTILINE,
    )
    anchors_by_name = {a.name: a for a in anchors}
    for layer, floor in floor_rows:
        layer = layer.strip()
        anchor = anchors_by_name.get(layer)
        if not anchor or not anchor.home:
            continue
        actual = read_cargo_version(REPO_ROOT / anchor.home.rstrip("/"))
        if not actual:
            continue
        if actual == floor:
            findings_info.append(
                f"forge-templates floor for {layer} matches actual (v{actual}) ✓"
            )
            continue
        floor_t = tuple(int(x) for x in floor.split("."))
        actual_t = tuple(int(x) for x in actual.split("."))
        drift = actual_t[1] - floor_t[1] if actual_t[0] == floor_t[0] else 999
        bucket = (
            findings_critical
            if drift >= FORGE_FLOOR_CRITICAL_DRIFT
            else findings_warning
        )
        bucket.append(
            f"forge-templates floor: {layer} floor v{floor}, actual v{actual} "
            f"(minor drift: {drift})"
        )


def main() -> None:
    parser = argparse.ArgumentParser(description="Boundary registry drift checker")
    parser.add_argument(
        "--strict", action="store_true",
        help="Treat warnings as critical for exit code purposes",
    )
    args = parser.parse_args()

    anchors = parse_registry()
    for a in anchors:
        check_readme(a)
        check_version(a)
    check_forge_floors(anchors)

    print(f"\n=== Drift Check Report — {len(anchors)} registry anchors ===\n")

    if findings_critical:
        print(f"🔴 Critical ({len(findings_critical)}):")
        for f in findings_critical:
            print(f"  - {f}")
        print()
    if findings_warning:
        print(f"🟡 Warning ({len(findings_warning)}):")
        for f in findings_warning:
            print(f"  - {f}")
        print()

    oks = [f for f in findings_info if "✓" in f]
    others = [f for f in findings_info if "✓" not in f]
    if oks:
        print(f"⚪ OK ({len(oks)}):")
        for f in oks:
            print(f"  - {f}")
        print()
    if others:
        print(f"⚪ Info ({len(others)}):")
        for f in others:
            print(f"  - {f}")
        print()

    print(
        f"Summary: {len(findings_critical)} critical, "
        f"{len(findings_warning)} warning, "
        f"{len(findings_info)} info."
    )

    if findings_critical:
        sys.exit(2)
    if findings_warning and args.strict:
        sys.exit(2)
    if findings_warning:
        sys.exit(1)
    sys.exit(0)


if __name__ == "__main__":
    main()

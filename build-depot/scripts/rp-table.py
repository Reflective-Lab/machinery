#!/usr/bin/env python3
"""Generate the Markdown RP-* table in QUALITY_BACKLOG.md from JSON source.

Reads `KB/05-engineering/standards/recurring-properties.json` and emits a
Markdown table to stdout. Closes QF-2026-06-02-18 — the table is generated
from a machine-readable source rather than hand-edited.

Invoked by `just rp-table` (print only) and `just rp-table-sync` (writes
the table into QUALITY_BACKLOG.md between the marker pair).
"""

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
SOURCE = REPO_ROOT / "KB/05-engineering/standards/recurring-properties.json"

COLUMNS = ["ID", "Property", "Enforcement", "Status", "Tracked by"]
FIELD_ORDER = ["id", "property", "enforcement", "status", "tracked_by"]


def render() -> str:
    data = json.loads(SOURCE.read_text())
    properties = data["properties"]

    lines = []
    lines.append("| " + " | ".join(COLUMNS) + " |")
    lines.append("|" + "|".join(["---"] * len(COLUMNS)) + "|")
    for p in properties:
        cells = [p[f] for f in FIELD_ORDER]
        # Cells must not contain literal newlines (Markdown tables are
        # line-oriented). The source already enforces single-line prose;
        # this is belt-and-suspenders.
        cells = [c.replace("\n", " ").strip() for c in cells]
        lines.append("| " + " | ".join(cells) + " |")
    return "\n".join(lines) + "\n"


def main() -> int:
    sys.stdout.write(render())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

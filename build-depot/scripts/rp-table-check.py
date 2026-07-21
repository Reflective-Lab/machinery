#!/usr/bin/env python3
"""Drift detector for the RP-* table in QUALITY_BACKLOG.md.

Compares the region between `<!-- BEGIN GENERATED RP-TABLE -->` and
`<!-- END GENERATED RP-TABLE -->` in QUALITY_BACKLOG.md against the
output of `scripts/rp-table.py`. Prints one of: `OK`, `DRIFT`,
`MISSING_MARKERS`. Used by `just quality-doctor` check 8.
"""

import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    backlog = REPO_ROOT / "QUALITY_BACKLOG.md"
    text = backlog.read_text()
    m = re.search(
        r"<!-- BEGIN GENERATED RP-TABLE -->\n(.*?)\n<!-- END GENERATED RP-TABLE -->",
        text,
        re.DOTALL,
    )
    if not m:
        print("MISSING_MARKERS")
        return 0

    generator = REPO_ROOT / "scripts/rp-table.py"
    in_file = m.group(1).strip()
    generated = subprocess.check_output(
        ["python3", str(generator)], text=True
    ).strip()

    print("OK" if in_file == generated else "DRIFT")
    return 0


if __name__ == "__main__":
    sys.exit(main())

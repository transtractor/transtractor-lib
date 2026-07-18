#!/usr/bin/env python3
"""Render pip-licenses JSON output as GitHub-flavoured Markdown.

Used by the CI audit job to show Python license findings in the workflow summary.
The renderer compares each package license against a configurable allow-list.
"""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Any

ALLOWED_LICENSES = [
    "Apache-2.0",
    "Apache-2.0 OR BSD-3-Clause",
    "BSD-3-Clause",
    "BSD-3-Clause, Apache-2.0, dependency licenses",
    "MIT",
    "MIT-0",
    "MIT-CMU",
    "MIT License",
    "PSF-2.0",
]


def load_json(path: Path) -> list[dict[str, Any]] | None:
    """Load pip-licenses JSON from file and return a list of packages."""
    if not path.exists():
        return None

    text = path.read_text(encoding="utf-8").strip()
    if not text:
        return None

    data = json.loads(text)
    if isinstance(data, list):
        return data
    if isinstance(data, dict):
        return [data]
    return None


def _normalize_license(license_name: str | None) -> str:
    """Normalize license values for comparison."""
    if not license_name:
        return ""
    return license_name.strip().lower()


def render(
    data: list[dict[str, Any]] | None,
    allowed_licenses: list[str] | None = None,
) -> tuple[str, int]:
    """Render pip-licenses JSON data as Markdown and return (body, finding_count)."""
    lines = ["## Python License Audit (pip-licenses)", ""]

    if data is None:
        lines.append("_License check produced no output._")
        return "\n".join(lines), 0

    allowed = {
        _normalize_license(item)
        for item in (allowed_licenses or ALLOWED_LICENSES)
        if item
    }

    findings: list[dict[str, Any]] = []
    for entry in data:
        if not isinstance(entry, dict):
            continue
        license_name = str(entry.get("License") or "")
        if _normalize_license(license_name) in allowed:
            continue
        findings.append(
            {
                "name": entry.get("Name") or "unknown",
                "version": entry.get("Version") or "",
                "license": license_name or "unknown",
            }
        )

    if not findings:
        lines.append("✅ No disallowed licenses found.")
        return "\n".join(lines), 0

    total = len(findings)
    plural = "s" if total != 1 else ""
    lines.append(f"**{total} disallowed license{plural}** found.")
    lines.append("")
    lines.append("| Package | Version | License |")
    lines.append("| --- | --- | --- |")

    for finding in findings:
        lines.append(
            f"| `{finding['name']}` | `{finding['version']}` | `{finding['license']}` |"
        )

    return "\n".join(lines), len(findings)


def write_github_output(name: str, value: str) -> None:
    output_path = os.environ.get("GITHUB_OUTPUT")
    if output_path:
        with open(output_path, "a", encoding="utf-8") as fh:
            fh.write(f"{name}={value}\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="Path to pip-licenses JSON file")
    parser.add_argument("output", type=Path, help="Path to output Markdown file")
    args = parser.parse_args()

    data = load_json(args.input)
    body, finding_count = render(data)
    args.output.write_text(body + "\n", encoding="utf-8")

    has_findings = "true" if finding_count > 0 else "false"
    write_github_output("has_findings", has_findings)


if __name__ == "__main__":
    main()

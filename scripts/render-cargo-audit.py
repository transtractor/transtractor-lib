#!/usr/bin/env python3
"""Render cargo audit --json output as Markdown.

Used by the CI audit job to show findings in the workflow summary.
"""

from __future__ import annotations

import argparse
import json
import os
from collections import Counter
from pathlib import Path


def load_json(path: Path) -> dict | None:
    """Load JSON from file, return None if file doesn't exist or is empty."""
    if not path.exists():
        return None
    text = path.read_text().strip()
    if not text:
        return None
    return json.loads(text)


def render(data: dict | None) -> tuple[str, int]:
    """Render cargo audit JSON data as Markdown."""
    lines = ["## Rust Dependency Audit (cargo-audit)", ""]

    if data is None:
        lines.append("_Audit produced no output._")
        raise ValueError("cargo audit output is empty or missing")

    vulnerabilities = data.get("vulnerabilities", {}).get("vulnerabilities", [])

    if not vulnerabilities:
        lines.append("✅ No known vulnerabilities found.")
        return "\n".join(lines), 0

    # Count vulnerabilities by severity
    by_severity = Counter(
        v.get("advisory", {}).get("severity", "unknown").lower()
        for v in vulnerabilities
    )

    total = len(vulnerabilities)
    plural = "s" if total != 1 else ""
    lines.append(f"**{total} known vulnerability{plural}** found.")
    lines.append("")

    # Severity summary
    severity_order = ["critical", "high", "medium", "low"]
    lines.append("### Summary by Severity")
    lines.append("")
    lines.append("| Severity | Count |")
    lines.append("| --- | ---: |")
    for severity in severity_order:
        if severity in by_severity:
            count = by_severity[severity]
            lines.append(f"| `{severity.upper()}` | {count} |")
    lines.append("")

    # Details grouped by package
    by_package = {}
    for v in vulnerabilities:
        pkg = v.get("package", {})
        pkg_name = pkg.get("name", "unknown")
        if pkg_name not in by_package:
            by_package[pkg_name] = []
        by_package[pkg_name].append(v)

    lines.append("### Vulnerabilities by Package")
    lines.append("")
    lines.append("| Package | Severity | Advisory | Title |")
    lines.append("| --- | --- | --- | --- |")

    for pkg_name in sorted(by_package.keys()):
        for v in by_package[pkg_name]:
            advisory = v.get("advisory", {})
            severity = advisory.get("severity", "unknown").upper()
            advisory_id = advisory.get("id", "N/A")
            title = advisory.get("title", "N/A")
            lines.append(f"| `{pkg_name}` | `{severity}` | {advisory_id} | {title} |")

    return "\n".join(lines), len(vulnerabilities)


def write_github_output(name: str, value: str) -> None:
    output_path = os.environ.get("GITHUB_OUTPUT")
    if output_path:
        with open(output_path, "a", encoding="utf-8") as fh:
            fh.write(f"{name}={value}\n")


def main() -> None:
    """Parse arguments and render audit output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="Path to cargo audit --json file")
    parser.add_argument("output", type=Path, help="Path to output Markdown file")
    args = parser.parse_args()

    body, vulnerability_count = render(load_json(args.input))
    args.output.write_text(body + "\n", encoding="utf-8")

    has_findings = "true" if (vulnerability_count) > 0 else "false"
    write_github_output("has_findings", has_findings)


if __name__ == "__main__":
    main()

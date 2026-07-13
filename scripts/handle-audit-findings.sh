#!/usr/bin/env bash
# Script to fail a GitHub Actions workflow if any vulnerabilities or disallowed licenses 
# are found. This allows all audit findings to be rendered and uploaded as artifacts 
# before failing the workflow.
#
# Arguments:
#   any number of has-findings flags ("true" or "false")

set -e

if [[ $# -eq 0 ]]; then
    echo "Error: At least one argument is required (has-findings flag: 'true' or 'false')"
    exit 1
fi

# Validate all arguments are "true" or "false"
has_findings=false
for arg in "$@"; do
    # Trim whitespace
    arg="${arg##+([[:space:]])}"
    arg="${arg%%+([[:space:]])}"
    if [[ "$arg" != "true" && "$arg" != "false" ]]; then
        echo "Error: Invalid argument '$arg'. Expected 'true' or 'false'."
        exit 1
    fi
    # If any argument is "true", set has_findings to true
    if [[ "$arg" == "true" ]]; then
        has_findings=true
    fi
done

if [[ "$has_findings" == "true" ]]; then
    echo "❌ Audit findings detected (vulnerabilities or disallowed licenses)."
    echo "See summary and uploaded artifacts for detailed findings."
    exit 1
else
    echo "✅ No known vulnerabilities or disallowed licenses detected."
    exit 0
fi

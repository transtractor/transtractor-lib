"""Tests for the Parser debug functionality."""

# pylint: disable=duplicate-code

import tempfile
from pathlib import Path

import pytest

from transtractor.parser import Parser
from transtractor.exceptions import StatementNotSupported


def test_debug_generates_correct_output():
    """Test that debug for test1.pdf generates output matching test1_debug.txt."""
    parser = Parser()

    # Parse the test PDF
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))
    expected_debug = fixtures_dir / "test1_debug.txt"

    # Generate debug output in a temporary file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as tmp_file:
        tmp_debug_path = tmp_file.name

    try:
        parser.debug(str(test_pdf), tmp_debug_path)

        # Read both debug files
        with open(tmp_debug_path, 'r', encoding='utf-8') as generated:
            generated_content = generated.read()

        with open(expected_debug, 'r', encoding='utf-8') as expected:
            expected_content = expected.read()

        # Compare content
        assert generated_content == expected_content, (
            "Debug output mismatch:\n"
            f"Generated length: {len(generated_content)}\n"
            f"Expected length: {len(expected_content)}"
        )
    finally:
        # Clean up temporary file
        Path(tmp_debug_path).unlink(missing_ok=True)


def test_debug_raises_statement_not_supported_without_config():
    """Test that debug without loading a config raises StatementNotSupported."""
    parser = Parser()

    # Try to debug the test PDF without loading the config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"

    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as tmp_file:
        tmp_debug_path = tmp_file.name

    try:
        # Should raise StatementNotSupported since no config is loaded
        with pytest.raises(StatementNotSupported):
            parser.debug(str(test_pdf), tmp_debug_path)
    finally:
        # Clean up temporary file
        Path(tmp_debug_path).unlink(missing_ok=True)

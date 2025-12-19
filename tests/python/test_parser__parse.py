"""Tests for the Parser parse method."""

# pylint: disable=duplicate-code

import tempfile
from pathlib import Path

import pytest

from transtractor.exceptions import StatementNotSupported
from transtractor.parser import Parser
from transtractor.structs.statement_data import StatementData
from transtractor.transtractor import NoErrorFreeStatementData # pylint: disable=no-name-in-module


def test_parse_generates_correct_csv():
    """Test that parsing test1.pdf generates a CSV matching test1_parsed.csv."""
    parser = Parser()

    # Parse the test PDF
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))
    expected_csv = fixtures_dir / "test1_parsed.csv"

    statement_data: StatementData = parser.parse(str(test_pdf))

    # Generate CSV in a temporary file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False, newline='') as tmp_file:
        tmp_csv_path = tmp_file.name
        statement_data.to_csv(tmp_csv_path)

    try:
        # Read both CSV files
        with open(tmp_csv_path, 'r', encoding='utf-8') as generated:
            generated_lines = generated.readlines()

        with open(expected_csv, 'r', encoding='utf-8') as expected:
            expected_lines = expected.readlines()

        # Compare line by line
        assert len(generated_lines) == len(expected_lines), (
            f"CSV line count mismatch: generated {len(generated_lines)} lines, "
            f"expected {len(expected_lines)} lines"
        )

        for i, (generated_line, expected_line) in enumerate(zip(generated_lines, expected_lines)):
            assert generated_line == expected_line, (
                f"CSV content mismatch at line {i + 1}:\n"
                f"Generated: {generated_line}\n"
                f"Expected: {expected_line}"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)


def test_parse_raises_statement_not_supported_without_config():
    """Test that parsing without loading a config raises StatementNotSupported."""
    parser = Parser()

    # Parse the test PDF without loading the config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"

    # Should raise StatementNotSupported since no config is loaded
    with pytest.raises(StatementNotSupported):
        parser.parse(str(test_pdf))


def test_parse_raises_no_error_free_statement_data_with_misconfigured_config():
    """Test that parsing with a misconfigured config raises NoErrorFreeStatementData."""
    parser = Parser()

    # Load the misconfigured config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    misconfigured_config = fixtures_dir / "test1_config_misconfigured.json"
    parser.load(str(misconfigured_config))

    # Should raise NoErrorFreeStatementData since the config is misconfigured
    with pytest.raises(NoErrorFreeStatementData):
        parser.parse(str(test_pdf))

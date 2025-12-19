"""Tests for Parser.load() method."""

from pathlib import Path

import pytest

from transtractor.parser import Parser
from transtractor.transtractor import ConfigLoadError # pylint: disable=no-name-in-module


def test_load_raises_config_load_error_with_invalid_config():
    """Test that loading an invalid config file raises ConfigLoadError."""
    parser = Parser()

    # Try to load an invalid config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    invalid_config = fixtures_dir / "test1_config_invalid.json"

    # Should raise ConfigLoadError since the config has an invalid country code
    with pytest.raises(ConfigLoadError):
        parser.load(str(invalid_config))

"""Handle loading of default configurations for statement parsing."""

from pathlib import Path

from ..transtractor import LibConfigDB  # Rust PyO3 class


# Register all configs in packages python/transtractor/configs directory. Caching is
# disabled for the base config DB for minimal memory usage. Configs
# are cached in the ConfigDB of a Parser instance when requested.
BASE_CONFIG_DB = LibConfigDB(caching=False, str_caching=False)

# Find and register all config files from python/transtractor/configs directory
_package_dir = Path(__file__).parent.parent
_configs_dir = _package_dir / "configs"
for _config_file in _configs_dir.rglob("*.json"):
    BASE_CONFIG_DB.register_from_file(str(_config_file))


def get_base_config_db() -> LibConfigDB:
    """Get the base configuration database containing all default configs.

    :return: The base LibConfigDB instance
    """
    return BASE_CONFIG_DB

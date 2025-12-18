"""Stub file for transtractor Rust extension module."""

class LibConfigDB:
    """Database for storing and managing statement configurations."""
    
    def __init__(self, caching: bool, str_caching: bool) -> None:
        """
        Create a new LibConfigDB instance.

        :param caching: If True, keep instances of loaded configs in memory
        :type caching: bool
        :param str_caching: If True, cache JSON string representations of configs
        :type str_caching: bool
        """
        ...
    
    def register_from_str(self, py_json_str: str) -> None:
        """
        Add config directly from a JSON string. Caching must be enabled.

        :param py_json_str: JSON string containing the configuration
        :type py_json_str: str
        :raises ConfigLoadError: If the configuration cannot be loaded
        """
        ...
    
    def register_from_file(self, py_file_path: str) -> None:
        """
        Add config from a JSON file path.

        :param py_file_path: Path to the JSON configuration file
        :type py_file_path: str
        :raises ConfigLoadError: If the configuration file cannot be loaded
        """
        ...
    
    def get_config_json_str(self, key: str) -> str:
        """
        Get JSON configuration string for a given key from the parser database.

        :param key: The configuration key to retrieve
        :type key: str
        :returns: JSON string representation of the configuration
        :rtype: str
        :raises ConfigAccessError: If the configuration key is not found
        """
        ...
    
    def get_account_terms(self, key: str) -> list[str]:
        """
        Get list of all account_terms from the registered config by key.

        :param key: The configuration key to retrieve account terms for
        :type key: str
        :returns: List of account term strings
        :rtype: list[str]
        :raises ConfigAccessError: If the configuration key is not found
        """
        ...
    
    def get_all_config_keys(self) -> list[str]:
        """
        Get list of all registered config keys.

        :returns: List of all configuration keys
        :rtype: list[str]
        """
        ...
    
    def has_config(self, key: str) -> bool:
        """
        Check if a config with the given key is registered.

        :param key: The configuration key to check
        :type key: str
        :returns: True if the key exists, False otherwise
        :rtype: bool
        """
        ...


class LibParser:
    """Parser for extracting statement data from text items."""
    ...

class NoErrorFreeStatementData(Exception):
    """Raised when no error-free statement data could be found."""
    ...


class ConfigLoadError(Exception):
    """Raised when a configuration cannot be loaded."""
    ...


class ConfigAccessError(Exception):
    """Raised when a configuration cannot be accessed."""
    ...

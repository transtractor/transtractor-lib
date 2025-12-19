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

    def register_from_str(self, py_json_str: str) -> None:
        """
        Add config directly from a JSON string. Caching must be enabled.

        :param py_json_str: JSON string containing the configuration
        :type py_json_str: str
        :raises ConfigLoadError: If the configuration cannot be loaded
        """

    def register_from_file(self, py_file_path: str) -> None:
        """
        Add config from a JSON file path.

        :param py_file_path: Path to the JSON configuration file
        :type py_file_path: str
        :raises ConfigLoadError: If the configuration file cannot be loaded
        """


    def get_config_json_str(self, key: str) -> str:
        """
        Get JSON configuration string for a given key from the parser database.

        :param key: The configuration key to retrieve
        :type key: str
        :returns: JSON string representation of the configuration
        :rtype: str
        :raises ConfigAccessError: If the configuration key is not found
        """


    def get_account_terms(self, key: str) -> list[str]:
        """
        Get list of all account_terms from the registered config by key.

        :param key: The configuration key to retrieve account terms for
        :type key: str
        :returns: List of account term strings
        :rtype: list[str]
        :raises ConfigAccessError: If the configuration key is not found
        """


    def get_all_config_keys(self) -> list[str]:
        """
        Get list of all registered config keys.

        :returns: List of all configuration keys
        :rtype: list[str]
        """


    def has_config(self, key: str) -> bool:
        """
        Check if a config with the given key is registered.

        :param key: The configuration key to check
        :type key: str
        :returns: True if the key exists, False otherwise
        :rtype: bool
        """



class LibParser:
    """Parser for extracting statement data from text items."""

    def __init__(self) -> None:
        """Create a new LibParser instance."""


    def import_config_from_json_str(self, py_json_str: str) -> None:
        """
        Import JSON configuration string into the parser database and update the StatementTyper.

        :param py_json_str: JSON string containing the configuration
        :type py_json_str: str
        :raises ConfigLoadError: If the configuration cannot be loaded
        """


    def register_config_from_json_str(self, py_json_str: str) -> None:
        """
        Register JSON configuration string into the parser database without 
        updating the StatementTyper.

        :param py_json_str: JSON string containing the configuration
        :type py_json_str: str
        :raises ConfigLoadError: If the configuration cannot be loaded
        """


    def import_config_from_file(self, py_file_path: str) -> None:
        """
        Import JSON configuration file into the parser database and update the StatementTyper.

        :param py_file_path: Path to the JSON configuration file
        :type py_file_path: str
        :raises ConfigLoadError: If the configuration file cannot be loaded
        """


    def add_account_terms(self, key: str, terms: list[str]) -> None:
        """
        Add account terms (list of strings) to the StatementTyper for a given config key.

        :param key: The configuration key
        :type key: str
        :param terms: List of account term strings
        :type terms: list[str]
        """


    def get_applicable_config_keys(self, py_text_items: list[dict]) -> list[str]:
        """
        Return a list of keys applicable to the provided text items.

        :param py_text_items: List of text item dictionaries
        :type py_text_items: list[dict]
        :returns: List of applicable config keys
        :rtype: list[str]
        """


    def get_unregistered_config_keys(self, keys: list[str]) -> list[str]:
        """
        Return a list of keys not yet registered in the parser database.

        :param keys: List of keys to check
        :type keys: list[str]
        :returns: List of unregistered keys
        :rtype: list[str]
        """


    def py_text_items_to_py_statement_data(
        self, py_text_items: list[dict], applicable_config_keys: list[str]
    ) -> object:
        """
        Process a Python list of text items and return statement data.

        :param py_text_items: List of text item dictionaries
        :type py_text_items: list[dict]
        :param applicable_config_keys: List of applicable config keys
        :type applicable_config_keys: list[str]
        :returns: StatementData object
        :rtype: object
        :raises NoErrorFreeStatementData: If no error-free statement data could be found
        """


    def py_text_items_to_debug_py_str(
        self, py_text_items: list[dict], applicable_config_keys: list[str]
    ) -> str:
        """
        Process a Python list of text items and return debug information as a string.

        :param py_text_items: List of text item dictionaries
        :type py_text_items: list[dict]
        :param applicable_config_keys: List of applicable config keys
        :type applicable_config_keys: list[str]
        :returns: Debug information string
        :rtype: str
        """


    def py_text_items_to_layout_py_str(
        self, py_text_items: list[dict], y_bin: float, x_gap: float
    ) -> str:
        """
        Process a Python list of text items and return layout text as a string.

        :param py_text_items: List of text item dictionaries
        :type py_text_items: list[dict]
        :param y_bin: Y coordinate bin size for sorting/merging text items
        :type y_bin: float
        :param x_gap: X coordinate gap size for merging text items
        :type x_gap: float
        :returns: Layout text string
        :rtype: str
        """


class NoErrorFreeStatementData(Exception):
    """Raised when no error-free statement data could be found."""


class ConfigLoadError(Exception):
    """Raised when a configuration cannot be loaded."""


class ConfigAccessError(Exception):
    """Raised when a configuration cannot be accessed."""

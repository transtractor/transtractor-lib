"""Custom exceptions for the Transtractor parser module. Note that
some exceptions are defined in the Rust extension module and exposed"""


class StatementNotSupported(Exception):
    """Raised when the statement format is unsupported or unidentifiable."""

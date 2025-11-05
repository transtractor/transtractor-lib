"""
Type stubs for transtractor

A library for extracting and processing bank statement data from PDF files.
This module provides tools to parse PDF bank statements from various Australian banks,
automatically identify statement types and formats, extract transaction data with
comprehensive error correction, and export clean data to CSV format.
"""

from .transtractor import Parser as Parser

__all__ = ["Parser"]
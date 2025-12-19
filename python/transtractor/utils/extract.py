"""Utilities for extracting text items from PDF files."""

from typing import List

import pdfplumber


def pdf_to_text_items(pdf_path: str) -> List[dict]:
    """Extract PDF into a list of dictionaries. These are used to create
    TextItem objects in Rust for extraction of bank statement data.

    :param pdf_path: Path to the PDF file to be processed
    :return: List of dictionaries representing text items extracted from the PDF
    """
    items: List[dict] = []
    with pdfplumber.open(pdf_path) as pdf:
        for page_index, page in enumerate(pdf.pages):
            for word in page.extract_words(x_tolerance=2):
                items.append({
                    "text": word.get("text", ""),
                    "x1": int(float(word["x0"])),
                    "y1": int(float(word["bottom"])),
                    "x2": int(float(word["x1"])),
                    "y2": int(float(word["top"])),
                    "page": page_index,
                })
    return items

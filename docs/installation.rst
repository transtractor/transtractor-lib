Installation
============

Installing from PyPI
--------------------

The easiest way to install the Transtractor is using pip:

.. code-block:: bash

   pip install transtractor


Installing from Source
----------------------

First install Rust by following the instructions at `rust-lang.org <https://www.rust-lang.org/tools/install>`_. 
Then install Maturin, the build tool for Python bindings:

.. code-block:: bash

   pip install maturin

To install the latest development version from GitHub:

.. code-block:: bash

   git clone https://github.com/transtractor/transtractor-lib.git
   cd transtractor-lib
   maturin develop --release


Testing the Installation
------------------------

To test the Rust library installation:

.. code-block:: bash
    
   cargo test

To run the Python tests:

.. code-block:: bash

   pip install pytest
   pytest tests/python

# Development Guide
This guide brings together the current development, validation, and release workflows for Transtractor. It covers local setup, CI/CD automation, testing and coverage, linting and type checking, dependency auditing, Sphinx documentation, and the release-day checklist used to ship new versions.

## Table of Contents
- [Local Development Environment](#local-development-environment)
- [CI/CD](#cicd)
- [Unit Tests & Coverage](#unit-tests--coverage)
- [Linting and Formatting](#linting-and-formatting)
- [Static Type Checking](#static-type-checking)
- [Dependency Audits](#dependency-audits)
- [Sphinx User Documentation](#sphinx-user-documentation)
- [Release-Day Workflow](#release-day-workflow)

## Local Development Environment
Before proceeding, ensure the following tools are installed:

* [**Rust Toolchain**](https://rust-lang.org/tools/install/) — Required to compile and test the core processing engine.
* [**uv**](https://docs.astral.sh/uv/getting-started/installation/) — Used to create the Python virtual environment and install all package dependencies.
* [**VS Code**](https://code.visualstudio.com/) — Recommended IDE. Project‑specific settings are provided in `.vscode/extensions.json` and `.vscode/settings.json`.

Clone the repository:

```shell
git clone https://github.com/transtractor/transtractor-lib.git
```

Then setup the Python virtual environment (*.venv*) and install the Transtractor by:

```shell
cd transtractor-lib
uv sync --locked --all-groups
```

This command installs Maturin and all Python runtime dependencies, compiles the Rust components, and installs the Python module. Development Python dependencies are also included. Some additional Rust tooling will need to be installed manually; these are described in their respective sections.


## CI/CD
The Transtractor repository implements a set of GitHub Actions workflows that automate testing, validation, security auditing, and release publishing across both the Rust and Python components. 

### ci.yml
This workflow runs on every push or pull request targeting the main branch. It guards against regressions, enforces code quality, and surfaces useful diagnostics for ongoing improvement. It consists of the following jobs:

* **test** — Runs unit tests (`pytest`, `cargo test`), linting (`ruff check`, `cargo clippy`), formatting (`ruff format`, `cargo fmt`), and a Sphinx documentation build (`make html`) across all [actively supported Python versions](https://devguide.python.org/versions/) and the major operating systems (Windows, macOS, and Ubuntu). The job fails if any test fails or if linting or formatting issues are detected. These checks run only on the default architecture of GitHub‑hosted runners and do not attempt to cover all build targets published to PyPI via the *build-release.yml* workflow. 

* **typecheck** — Performs static type checking on Python components using `pyright` and writes the results to the GitHub Markdown summary. Type errors do not fail the pipeline, but they should be fully resolved before release.

* **coverage** — Computes and renders per‑module test coverage for Rust (`cargo llvm-cov`) and Python (`pytest`) in the GitHub Markdown summary. Cobertura XML artifacts for both languages are uploaded to Codecov, which produces a combined coverage metric displayed in the *README.md*. The workflow does not enforce a minimum coverage threshold, though this may be introduced as the project matures beyond beta.

### audit.yml
This workflow runs on every push or pull request targeting the *main* branch. It scans Rust and Python dependencies for known vulnerabilities (`cargo audit`, `pip-audit`) and for license incompatibilities (`pip-licenses`, `cargo deny`). It is also scheduled to run weekly to catch any issues that may have surfaced since the last push or pull request. All findings are written to the GitHub Markdown summary. The workflow is configured to fail if any issues are detected, but only after they have been fully rendered in the summary.

### build.yml
This workflow runs on every push or pull request targeting the main branch and verifies that the package can be built across all supported architectures intended for publication to PyPI. Its contents are adapted from the configuration generated automatically by `maturin generate-ci github`.

### release.yml
This workflow runs whenever a new version tag is pushed to the repository. It performs a series of sequential checks designed to prevent faulty or incomplete releases from being published to [PyPI](https://pypi.org/project/transtractor/). These checks include tag validation and the full suite of tests drawn from the **ci.yml**, **build.yml**, and **audit.yml** workflows. Once all checks pass and the package has been successfully built across all supported architectures, the new version is published to PyPI and a draft GitHub release is created.

Creating this draft release triggers the Sphinx documentation build on [**Read the Docs**](https://transtractor-lib.readthedocs.io/en/latest/), ensuring that the hosted user documentation is updated to match the newly released version.


## Unit tests & Coverage

### Python
Python unit tests are maintained in the *tests/python* directory and follow the naming convention `test_{module|class}__{function|method}`. They can be run with:

```shell
uv run pytest
```

This produces line‑by‑line coverage data in `cov-python.xml`, which is automatically detected by the **Coverage Gutters** extension to display coverage highlighting directly in VS Code. All Pytest configuration is defined in `pyproject.toml`, so no additional flags are required when invoking `pytest`.

If an interactive HTML coverage report is desired instead, run:

```shell
uv run pytest --cov=transtractor --cov-report=html:htmlcov-python
```

This generates a browsable report in the `htmlcov-python` directory.

### Rust
Rust unit tests are defined alongside the source code in their respective modules and can be run with:

```shell
cargo test
```

For coverage analysis, install the `llvm-cov` tool:

```shell
cargo install cargo-llvm-cov
```

Then generate the `cov-rust.xml` Cobertura report, which is used by the Coverage Gutters extension in VS Code, by running:

```shell
cargo llvm-cov --all-features --cobertura --output-path cov-rust.xml
```

This produces line‑level coverage information for all Rust modules included in the build.

### Rendering Markdown Reports
The Python script `render-coverage.py` converts Cobertura XML coverage outputs into per‑module Markdown reports suitable for inclusion in the GitHub Actions summary. It can be invoked as follows:

```shell
uv run python scripts/render-coverage.py cov-rust.xml cov-rust.md Rust
uv run python scripts/render-coverage.py cov-python.xml cov-python.md Python
```


## Linting and Formatting
The GitHub Actions workflows treat any linting or formatting issue as a failure. To minimise friction during development, the project’s `settings.json` configures VS Code to automatically format files on save, reducing the likelihood of issues surfacing in CI.

### Python
Python linting and formatting are handled by the high‑performance **Ruff** tool. Ruff’s configuration is defined in `pyproject.toml` and enforces checks for unused imports and variables, PEP 8 compliance, indentation and whitespace conventions, import ordering, modernised Python syntax, bug‑prone patterns, security‑sensitive patterns, and accidental `print` statements. Assert statements are explicitly permitted in test files, following standard Pytest conventions. Additional settings enforce maximum line length, quoting style, and indentation rules.

To lint the Python source and test suite:

```shell
uv run ruff check python tests
```

To verify that all formatting rules are satisfied:

```shell
uv run ruff format --check python tests
```

### Rust
Rust linting is performed using **cargo clippy** with default settings, while formatting is enforced using **cargo fmt**.

To lint all Rust targets:

```shell
cargo clippy --all-targets --all-features -- -D warnings
```

The `-- -D warnings` flag instructs Clippy to treat all warnings as errors, ensuring that any remaining issues fail the GitHub Actions workflow.

To assert formatting:

```shell
cargo fmt --check
```

## Static Type Checking
Rust enforces typing at compile time, but Python requires an external type checker to validate type hints. This project uses **Pyright**. To run Pyright and capture its output:

```shell
uv run pyright --outputjson > pyright.json
```

The **render-pyright.py** script converts the JSON output into a Markdown summary suitable for inclusion in GitHub Actions job summaries. Generate the report with:

```shell
uv run python scripts/render-pyright.py pyright.json pyright-summary.md
```

## Dependency Audits
Dependency auditing validates both Python and Rust dependencies for security vulnerabilities and licences that may in conflict with the MIT licence applied to this package.

### Python
Python dependency audits are performed in three phases:

* Export the runtime dependencies set from `uv` as a requirements-style lock file *requirements-audit.txt*
* Scan this lock file with `pip-audit` and `pip-licenses`
* Render the outputs to GitHub-flavoured Markdown files

Export only Python dependencies:

```shell
uv export \
    --format requirements-txt \
    --no-dev \
    --no-emit-project \
    --all-extras \
    -o requirements-audit.txt
```

Scan resolved dependencies for known vulnerabilities:

```shell
uv run pip-audit \
    -r requirements-audit.txt \
    --require-hashes \
    --disable-pip \
    --format=json \
    -o pip-audit.json
```

Render the vulnerability findings into a Markdown report:

```shell
uv run python scripts/render-pip-audit.py pip-audit.json pip-audit.md
```

Get licences for all dependencies:

```shell
PACKAGES=$(grep -oE '^[A-Za-z0-9][A-Za-z0-9._-]*' requirements-audit.txt | sort -u | xargs)
uv run pip-licenses \
    --from=mixed \
    --packages "$PACKAGES" \
    --format=json > pip-licenses.json
```

Evaluate and render the licence audit results into Markdown:

```shell
uv run python scripts/render-pip-licenses.py pip-licenses.json pip-licenses.md
```

Only packages with licences that have not been explicitly allowed within the script will be written to the report.

### Rust
Rust dependency auditing is split between vulnerability scanning and licence scanning, using `cargo audit` for security and `cargo deny` for licence policy checks. Both outputs are rendered into Markdown summaries.

Scan Rust dependencies for known vulnerabilities:

```shell
cargo audit --json > cargo-audit.json
```

Render Rust vulnerability findings into Markdown:

```shell
uv run python scripts/render-cargo-audit.py cargo-audit.json cargo-audit.md
```

Scan Rust dependencies for licence issues:

```shell
cargo deny --format json check licenses 2> cargo-deny.json
```

This command references *deny.toml* for a list of allowed licences.

Render Rust licence audit findings into Markdown:

```shell
uv run python scripts/render-cargo-deny.py cargo-deny.json cargo-deny.md
```

## Sphinx User Documentation
User documentation is authored using **Sphinx** and published to [**Read the Docs**](https://transtractor-lib.readthedocs.io/en/latest/). Deployment is handled via a repository‑level webhook that triggers builds on Read the Docs whenever a release is created, edited, published, unpublished or deleted. Therefore, this process is automatically triggered when the `release.yml` GitHub Actions workflow has created a draft release once all preflight checks have passed and artifacts have been published to PyPI. Read the Docs automatically pulls the repository, builds the documentation, and publishes it so that the hosted docs always reflect the latest released version of the code. The build configuration used by Read the Docs is defined in `.readthedocs.yaml`.

To build the documentation locally:

```shell
cd docs
uv run make html
```

The Sphinx documentation is intended for standard Python users and should focus on usage, installation, and API‑level guidance. Development‑specific or maintenance‑oriented technical details should instead be placed in the Markdown files within the repository, where they serve core contributors and maintainers rather than end‑users.


## Release-Day Workflow
A standard package release will typically involve the following steps:

1. Updating Rust and Python dependencies
2. Updating documentation
3. Real-world parsing tests
4. Bumping version numbers
5. Merging changes
6. Tagging a new release triggering deployments to PyPI and Read the Docs
7. Publishing release notes

### Step 1: Update dependencies
A release is an ideal opportunity to update dependencies to stay ahead of emerging vulnerabilities and benefit from free performance improvements.

#### Rust Dependencies
Update Rust dependencies by editing `Cargo.toml` to reference the latest versions published on [crates.io](https://crates.io/). Then refresh the `cargo.lock` file:

```shell
cargo update
```

After updating, run the full Rust test suite and address any breaking changes:

```shell
cargo test
```

#### Python Dependencies
First drop support for any Python version that have reached [their end of life](https://devguide.python.org/versions/). This is done by editing the following:

* The `requires-python`, `pythonVersion` and `classifiers` fields in **pyproject.toml**
* The `python-version` fields in **ci.yml**, **audit.yml** and **release.yml**, especially those under `matrix`
* The `python` field in **.readthedocs.yaml**

Update Python dependencies by editing `pyproject.toml` to reference the latest versions published on [PyPI](https://pypi.org/). Then refresh the `uv.lock` file:

```shell
uv sync --upgrade
```

Run all unit tests and fix any breaking changes that may have been introduced.

```shell
uv run pytest
```

If Maturin has been updated, then regenerate release workflows by `maturin generate-ci github` and update the respective `steps` in the **build.yml** and **release.yml** files.

### Step 2: Updating Documentation
Review and update documentation as required.

User documentation:

* **api_reference.rst** — Python API features 
* **configuration.rst** — User contribution instructions, statement-parsing parameters and recognised date/currency formats
* **supported_statements.rst** — Supported bank statements

Developer documentation:

* **README.md** — Top-level index of entire project
* **develop.md** — Core developer and maintainer instructions
* **architecture.md** — Key application components and concepts

### Step 3: Real-World Parsing Tests
The CI/CD unit tests should catch most regressions, but it is still valuable to trial new updates against real bank statements. These files are not included in the repository for privacy reasons. To perform a quick manual check, place any available statement PDFs into the `pdfs` directory, start an interactive Python session (`uv run python`), and run:

```python
from transtractor import Parser
parser = Parser()
parser.parse("pdfs/")
```

All PDF files in this directory and its subdirectories should parse successfully.

### Step 4: Bump Version
Version numbers must be updated in both `Cargo.toml` and `docs/conf.py` to reflect the new release.

This project follows **Semantic Versioning (SemVer)**, using the `Major.Minor.Patch` convention. The initial release was set to `0.9.0` to indicate a near‑production beta. Subsequent increments follow these rules:

* **Major** — Breaking changes or promotion from beta to full production
* **Minor** — Newly supported statement formats or the introduction of new features
* **Patch** — Bug fixes, refactoring, dependency updates, performance improvements, or documentation changes

While the project remains in beta, both Minor and Patch updates will simply increment the Patch version.

### Step 5: Merge Changes
Steps 1–4 should be completed in a pull request originating from a development fork. Once all GitHub Actions checks have passed, merge the pull request into the `main` branch of the parent repository.

### Step 6: Tagging and Triggering a New Release
In a development environment pointing to the parent repository, create a new version tag. For example:

```shell
git tag v1.0.1
git push origin v1.0.1
```

Pushing the tag triggers the `release.yml` workflow. As a pre‑flight check, the workflow verifies that the tag conforms to the SemVer format and that the version has been updated in both `Cargo.toml` and `docs/conf.py`. Once validated, the package is automatically published to PyPI, and the Sphinx documentation is rebuilt on **Read the Docs**, ensuring the hosted docs match the newly released version.

### Step 7: Completing the Release Notes
Complete the Release Notes using the template generated at the end of the `release.yml` workflow. The draft is already associated with the correct version tag, removing one more detail to remember to set.
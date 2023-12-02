## Configuration
## =============

# Have zero effect by default to prevent accidental changes.
.DEFAULT_GOAL := help

# Delete targets that fail to prevent subsequent attempts incorrectly assuming
# the target is up to date.
.DELETE_ON_ERROR: ;

# Prevent pesky default rules from creating unexpected dependency graphs.
.SUFFIXES: ;


## Verbs
## =====

.PHONY: none

help:
	@./bin/mkhelp.py help < $(MAKEFILE_LIST)

## Checks
## ------

## Run all checks
check: check_format check_lint check_tests check_tests_rs_duration check_types_py;
.PHONY: check

## Run formatters for all parts of the project
check_format: check_format_py check_format_rs;
.PHONY: check_format

check_format_py:
	isort bin/*.py --check
	black bin/*.py --check
.PHONY: check_format_py

check_format_rs:
	cargo fmt --check
.PHONY: check_format

## Run linters for all parts of the project
check_lint: check_lint_py check_lint_rs;
.PHONY: check_lint

check_lint_py:
	ruff bin/
.PHONY: check_lint_py

check_lint_rs:
	cargo clippy --tests --examples
.PHONY: check_lint_rs

## Run unit tests for all parts of the project
check_tests:
	cargo test
	cargo test --examples --release
.PHONY: check_tests

## Run unit tests for rust code in release mode and time them
check_tests_rs_duration:
	cargo +nightly test --release -- -Z unstable-options --report-time
.PHONY: check_tests_rs_duration

## Run type checkers for python code
check_types_py:
	mypy bin/
.PHONY: check_types_py

## Fixes
## -----

## Attempt to fix formatting problems in all parts of the project
fix_format: fix_format_py fix_format_rs;
.PHONY: fix_format

fix_format_py:
	isort bin/*.py
	black bin/*.py
.PHONY: fix_format_py

fix_format_rs:
	cargo fmt
.PHONY: fix_format_rs

## Attempt to fix linting problems in rust parts of the project
fix_lint_rs:
	cargo clippy --fix
.PHONY: fix_lint_rs

## Other verbs
## -----------

## Install the recommended python dependencies for development
install_deps_py:
	PIP_CONSTRAINT=constraints.txt pip install -r requirements.txt

## Serve webapp with automatic reloading
serve-dev: crates/webapp/index.html
	mkdir -p dist/debug/
	trunk serve \
		--dist dist/debug/ \
		$<
.PHONY: serve-dev

## Serve webapp without automatic reloading
##
## This is a more accurate representation of what the app will look like when published.
serve-rel: dist/release/index.html
	cd $(<D) \
	&& python -m http.server 8000
.PHONY: serve-rel


## Nouns
## =====

./bin/mkhelp.py:
	mkhelp print_script > $@
	chmod +x $@

constraints.txt: requirements.txt
	pip-compile --allow-unsafe --no-header --output-file $@ --quiet $^

# Build webapp for serving locally
dist/release/index.html: crates/webapp/index.html
	rm -r $(@D)||:
	mkdir -p $(@D)
	trunk build \
		--dist $(@D) \
		--release \
		$<

## Build webapp for publication
docs/index.html: crates/webapp/index.html
	rm -r $(@D)||:
	mkdir -p $(@D)
	trunk build \
		--dist $(@D) \
		--public-url aocoracle \
		--release \
		$<

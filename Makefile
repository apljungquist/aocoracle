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

## Run all other checks
check_all: check_format check_lint check_tests check_tests_duration
.PHONY: check_all

## _
check_format:
	isort bin/*.py --check
	black bin/*.py --check
	cargo fmt --check
.PHONY: check_format

## _
check_lint:
	cargo clippy --tests
.PHONY: check_lint

## _
check_tests:
	cargo test
.PHONY: check_tests

## _
check_tests_duration:
	cargo +nightly test --release -- -Z unstable-options --report-time
.PHONY: check_tests_duration

## Fixes
## -----

## _
fix_format:
	isort bin/*.py
	black bin/*.py
	cargo fmt
.PHONY: fix_format

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

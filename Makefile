# Configuration
# =============

# Have zero effect by default to prevent accidental changes.
.DEFAULT_GOAL := none

# Delete targets that fail to prevent subsequent attempts incorrectly assuming
# the target is up to date.
.DELETE_ON_ERROR: ;

# Prevent pesky default rules from creating unexpected dependency graphs.
.SUFFIXES: ;


# Verbs
# =====

.PHONY: none

none:
	@echo No target specified

.PHONY: check_format
check_format:
	isort bin/*.py --check
	black bin/*.py --check
	cargo fmt --check

.PHONY: check_lint
check_lint:
	cargo clippy --tests

.PHONY: check_tests
check_tests:
	cargo test

.PHONY: fix_format
fix_format:
	isort bin/*.py
	black bin/*.py
	cargo fmt

# Nouns
# =====

constraints.txt: requirements.txt
	pip-compile --allow-unsafe --no-header --output-file $@ $^

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

.PHONY: check_all
check_all: check_format check_lint check_tests check_tests_duration

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

.PHONY: check_tests_duration
check_tests_duration:
	cargo +nightly test --release -- -Z unstable-options --report-time

.PHONY: fix_format
fix_format:
	isort bin/*.py
	black bin/*.py
	cargo fmt

.PHONY: serve-dev

serve-dev: crates/webapp/index.html
	mkdir -p dist/debug/
	trunk serve \
		--dist dist/debug/ \
		$<

serve-rel: dist/release/index.html
	cd $(<D) \
	&& python -m http.server 8000


# Nouns
# =====

constraints.txt: requirements.txt
	pip-compile --allow-unsafe --no-header --output-file $@ $^

dist/release/index.html: crates/webapp/index.html
	rm -r $(@D)||:
	mkdir -p $(@D)
	trunk build \
		--dist $(@D) \
		--release \
		$<

docs/index.html: crates/webapp/index.html
	rm -r $(@D)||:
	mkdir -p $(@D)
	trunk build \
		--dist $(@D) \
		--public-url aocoracle \
		--release \
		$<

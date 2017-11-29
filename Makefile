# Makefile for installing syntect plugin on macOS and Linux

xi-editor/rust/target/release/xi-core:
	git submodule init && git submodule update
	cd xi-editor/rust && cargo build --release

plugins/syntect/bin/xi-syntect-plugin: export XI_PLUGIN_DIR = $(shell pwd)/plugins
plugins/syntect/bin/xi-syntect-plugin:
	cd xi-editor/rust/syntect-plugin && $(MAKE) install

setup: xi-editor/rust/target/release/xi-core plugins/syntect/bin/xi-syntect-plugin

.PHONY: setup

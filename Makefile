BINDGEN = bindgen
BINDGENFLAGS = --use-core
CARGO = cargo

SOURCES := $(wildcard lib/*/src/*.rs lib/*/src/*/*.rs lib/*/src/*/*/*.rs)
VERSION := $(shell cat VERSION)

all: Cargo.toml $(SOURCES)
	$(CARGO) build

bindgen: lib/asimov-sys/src/bindgen.rs

lib/asimov-sys/src/bindgen.rs: etc/bindgen/allowlist.txt etc/bindgen/options.txt
	$(BINDGEN) -o $@ ../c/src/asimov.h $(BINDGENFLAGS) $(shell cat $^)

check: Cargo.toml $(SOURCES)
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

distclean: clean

mostlyclean: clean

maintainer-clean: clean

.PHONY: all bindgen check
.PHONY: clean distclean mostlyclean maintainer-clean
.SECONDARY:
.SUFFIXES:

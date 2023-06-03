INSTALL_PATH		:= $(HOME)/usr/bin/
PLIST_DEBUG_BIN		:=target/debug/plist
PLIST_RELEASE_BIN	:=target/release/plist
PLIST_BIN		:=$(PLIST_DEBUG_BIN)

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(PLIST_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(PLIST_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

.PHONY: all clean cls release debug fix fmt check build test


release: check fix | $(PLIST_RELEASE_BIN)
	install $(PLIST_RELEASE_BIN) $(INSTALL_PATH)

debug: check fix | $(PLIST_DEBUG_BIN)
	install $(PLIST_DEBUG_BIN) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fix:
	cargo fix --allow-dirty --allow-staged

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@

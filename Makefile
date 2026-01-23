# Makefile for emoji-picker: build, test, and package (Flatpak, RPM)

# Project version (auto-detected from Cargo.toml)
VERSION ?= $(shell grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)

# Paths
CARGO = cargo
TARGET = $(CURDIR)/target/release/emoji-picker
BUILD_DIR = $(CURDIR)/.build
FLATPAK_STAGE = $(BUILD_DIR)/flatpak
FLATPAK_MANIFEST = packaging/flatpak/nl.dibitat.emoji_picker.json
FLATPAK_METAINFO = packaging/nl.dibitat.emoji_picker.metainfo.xml
RPM_SPEC = packaging/rpm/emoji-picker.spec

.PHONY: all build test clean flatpak rpm version-inject bundle dist

all: build

build:
	$(CARGO) build --release

test:
	$(CARGO) test

clean:
	$(CARGO) clean
	rm -rf $(BUILD_DIR)


# Packaging-only targets (skip build)
flatpak-package:
	@if [ ! -f $(TARGET) ]; then \
	  echo "Error: $(TARGET) not found. Run 'make build' first."; \
	  exit 1; \
	fi
	rm -rf $(FLATPAK_STAGE)
	mkdir -p $(FLATPAK_STAGE)
	cp $(TARGET) $(FLATPAK_STAGE)/emoji-picker
	cp packaging/emoji-picker.desktop $(FLATPAK_STAGE)/emoji-picker.desktop
	cp packaging/emoji-picker.svg $(FLATPAK_STAGE)/emoji-picker.svg
	cp packaging/nl.dibitat.emoji_picker.metainfo.xml $(FLATPAK_STAGE)/nl.dibitat.emoji_picker.metainfo.xml
	cp data/style.css $(FLATPAK_STAGE)/style.css
	cp packaging/flatpak/nl.dibitat.emoji_picker.json $(FLATPAK_STAGE)/nl.dibitat.emoji_picker.json
	@echo "Injecting version $(VERSION) into staged manifests..."
	@sed -i 's/"version": ".*"/"version": "$(VERSION)"/' $(FLATPAK_STAGE)/nl.dibitat.emoji_picker.json
	@sed -i 's/<release version="[^"]*"/<release version="$(VERSION)"/' $(FLATPAK_STAGE)/nl.dibitat.emoji_picker.metainfo.xml
	cd $(FLATPAK_STAGE) && flatpak-builder --force-clean --disable-rofiles-fuse ../build-dir nl.dibitat.emoji_picker.json

rpm-package: version-inject
	@if [ ! -f $(TARGET) ]; then \
	  echo "Error: $(TARGET) not found. Run 'make build' first."; \
	  exit 1; \
	fi
	@echo "RPM build would run here (see $(RPM_SPEC))"
	# Example: rpmbuild -ba $(RPM_SPEC)

# Full build + package targets
flatpak: build flatpak-package


rpm: build version-inject
	@echo "RPM build would run here (see $(RPM_SPEC))"
	# Example: rpmbuild -ba $(RPM_SPEC)

version-inject:
	@echo "Injecting version $(VERSION) into manifests..."
	@sed -i 's/"version": ".*"/"version": "$(VERSION)"/' $(FLATPAK_MANIFEST)
	@sed -i 's/<release version="[^"]*"/<release version="$(VERSION)"/' $(FLATPAK_METAINFO)
	@sed -i 's/^Version:.*/Version:        $(VERSION)/' $(RPM_SPEC)

bundle: flatpak rpm

dist: clean build version-inject bundle
	@echo "Distribution artifacts are ready."
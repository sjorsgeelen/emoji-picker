#!/usr/bin/env bash
# ci-build.sh: Build, test, and package the Emoji Picker app for Flatpak
#
# This script is intended for local development and CI environments.
# It builds the Rust binary, runs tests, and packages the app as a Flatpak bundle.
#
# Steps:
# 1. Build the Rust binary (release mode)
# 2. Run Rust tests (release mode)
# 3. Build Flatpak package (using prebuilt binary)
# 4. (Optional) Run the Flatpak app in the build sandbox for a smoke test
# 5. Create a distributable .flatpak bundle
set -e

# 1. Build Rust binary
cargo build --release

# 2. Run tests
cargo test --release

# 3. Build Flatpak package and export to repo
flatpak-builder --force-clean --repo=repo build-dir packaging/nl.dibitat.emoji_picker.json

# 4. (Optional) Run Flatpak app in sandbox for smoke test
# Uncomment the next line to enable
# flatpak-builder --run build-dir packaging/nl.dibitat.emoji_picker.json emoji-picker

# 5. Create distributable bundle
flatpak build-bundle repo emoji-picker.flatpak nl.dibitat.emoji_picker

# End of script

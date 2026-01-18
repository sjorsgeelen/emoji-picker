# Flatpak Packaging for Emoji Picker

## Approach

This project uses a two-stage build and packaging process for Flatpak:

1. **Rust Build Outside Flatpak**
   - The Rust application is built and tested outside the Flatpak sandbox using `cargo build --release` and `cargo test --release`.
   - This avoids network and toolchain issues inside the Flatpak build environment and ensures fast, reliable builds.

2. **Flatpak Packaging**
   - The prebuilt binary and resources (desktop file, icon) are packaged using Flatpak.
   - The Flatpak manifest (`packaging/nl.dibitat.emoji_picker.json`) is located in the `packaging/` directory for project cleanliness.
   - The manifest's `build-commands` only copy/install the prebuilt binary and resources into the Flatpak image.
   - The build script (`scripts/ci-build.sh`) automates the process: build, test, package, and bundle.

## Build Steps

1. Build and test the Rust app:
   ```sh
   cargo build --release
   cargo test --release
   ```
2. Build the Flatpak package and export to a repo:
   ```sh
   flatpak-builder --force-clean --repo=repo build-dir packaging/nl.dibitat.emoji_picker.json
   ```
3. (Optional) Run the app in the Flatpak sandbox:
   ```sh
   flatpak-builder --run build-dir packaging/nl.dibitat.emoji_picker.json emoji-picker
   ```
4. Create a distributable bundle:
   ```sh
   flatpak build-bundle repo emoji-picker.flatpak nl.dibitat.emoji_picker
   ```

## Notes
- All packaging files (manifest, desktop file, icon) are in `packaging/`.
- All build scripts are in `scripts/`.
- Build artifacts and scripts are ignored by git via `.gitignore`.
- This approach is portable, CI-friendly, and avoids Flatpak sandbox network/toolchain issues.

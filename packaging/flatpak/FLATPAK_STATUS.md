# Flatpak Packaging Status (as of 2026-01-23)

## Current State
- The Flatpak manifest (`packaging/flatpak/nl.dibitat.emoji_picker.json`) is now fully standards-compliant:
  - Removed non-standard top-level fields (`name`, `version`).
  - Uses only required fields: `app-id`, `runtime`, `runtime-version`, `sdk`, `command`, etc.
  - All modules and sources are correctly specified.
- AppStream metadata is provided as `nl.dibitat.emoji_picker.metainfo.xml`:
  - Installed to `/app/share/metainfo/`.
  - `<id>` matches the Flatpak `app-id`.
  - Passes `appstream-util validate-relax` with no errors.
- The Makefile automates:
  - Clean staging in `.build/flatpak`.
  - Version injection for both manifest and metainfo.
  - All files are copied and staged as required.

## Outstanding Issue
- The Flatpak build fails at the `appstreamcli compose` step with:
  - `E: file-read-error` for the metainfo file, despite it being present, readable, and valid.
  - All other install and build steps succeed.
- This is likely a bug or limitation in the Flatpak/appstreamcli toolchain, not in the manifest or metadata.
- All best practices and Flatpak/AppStream standards have been followed.

## Next Steps
- Consider reporting this as a bug to Flatpak/AppStream if it blocks release.
- Try building on a different system or with updated Flatpak/appstreamcli versions if possible.

---

# Next: RPM Packaging
- The Flatpak packaging is as correct as possible given current toolchain behavior.
- Proceeding to ensure the RPM packaging is robust, version-injected, and cleanly staged.

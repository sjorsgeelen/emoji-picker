# Flatpak Build Instructions for Emoji Picker

## Prerequisites
- Flatpak and flatpak-builder installed
- GNOME SDK and Platform runtime (version 45)

## Build and Install

1. Build the Flatpak bundle:

   flatpak-builder --force-clean build-dir dev.emoji_picker.EmojiPicker.json

2. Run the app from the build directory:

   flatpak-builder --run build-dir dev.emoji_picker.EmojiPicker.json emoji-picker

3. (Optional) Install locally:

   flatpak-builder --user --install --force-clean build-dir dev.emoji_picker.EmojiPicker.json

4. (Optional) Create a .flatpak bundle for distribution:

   flatpak build-bundle build-dir emoji-picker.flatpak dev.emoji_picker.EmojiPicker

## Notes
- Make sure the desktop file and icon are present in the repo for proper integration.
- The app-id is `dev.emoji_picker.EmojiPicker`.
- Update the manifest if you add dependencies or change the build process.

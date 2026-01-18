# emoji-picker

## CI/Build Process

This project uses a portable build script (`ci-build.sh`) for local development and CI:

- Builds the Rust binary (`cargo build --release`)
- Runs Rust tests (`cargo test --release`)
- Packages the app as a Flatpak using the prebuilt binary
- (Optional) Runs the Flatpak app in the build sandbox for a smoke test
- (Optional) Creates a distributable `.flatpak` bundle

You can run the script with:

    ./ci-build.sh

This approach is compatible with local development, CI, and can be translated to Forgejo Actions or GitHub Actions for full automation.

---

## ✨ Features

- ⚡ **Instant startup** – Launches and is ready to use in a flash
- 🔎 **Fuzzy search** – Search emoji by name or keywords (supports multiple locales)
- ⌨️ **Full keyboard navigation** – Browse, search, and select emoji without touching the mouse
- 🖱️ **Mouse support** – Click any emoji to copy it
- 📋 **Copy to clipboard** – Press Enter or click to copy the selected emoji
- 🎨 **Modern GTK4 UI** – Clean, responsive, and follows GNOME conventions
- 🟩 **Visual feedback** – Highlight animation when emoji is copied
- 🗂️ **Category browsing** – Quickly jump between emoji categories
- 🔍 **Live search results** – See results as you type
- 🏳️‍🌈 **Unicode 15+ emoji support** – Always up to date
- 🌓 **System theme aware** – Follows light/dark mode (if supported by system)
- 🏃 **Fast exit** – Press Escape to instantly close the picker


The goal of this project is to (vibe)code a fast emoji-picker for GNOME. A common use case is that a user presses 'super'+'.', fuzzy searches the desired emoji based on the name and/or keywords in English or another localized form, and when the user presses 'return', the selected emoji (or Unicode character) is copied to the clipboard or inserted automatically.

I want the app to be focused on keyboard UX by making it superfast in start up, search and pasting. It should also get easily out of the way of the user. 


## Roadmap / Backlog
- [ ] Add a tab for recently used emoji
	- Track emoji usage and display most recent in a dedicated tab or category
- [ ] User-configurable settings
	- Allow users to configure preferences (see below)
- [ ] Light, dark, and system theme support
	- Picker should follow system theme or allow user override
- [ ] Configure locale-specific emoji matching
	- User can select preferred language/locale for emoji search and keywords
- [ ] Insert copied emoji directly at previous cursor position
	- If picker is launched from a text field, insert emoji at the original cursor location
	- Note: Due to Flatpak sandboxing, direct insertion into another app's text field is not possible. This feature will only be available in native (non-sandboxed) builds. For Flatpak, the emoji will be copied to the clipboard and the user can paste manually.
- [ ] Package and easy installation
	- Provide distribution packages (Flatpak, .deb, etc.) and simple install instructions
- [ ] Tooltip on long selection
	- When an emoji is selected for more than 2 seconds, show a tooltip with its name and keywords.

Recently used emojis are suggested in a recently used category.

The app should be able to follow (dark/light) system themes.

## Goal

I want to build a fast emoji-picker for GNOME.

## Development & Contribution Guidelines

Please refer to [.instructions.md](.instructions.md) for Rust best practices, project conventions, and contribution guidelines. Following these instructions helps maintain code quality and consistency.

## repo structure

```
emoji-picker/
├── Cargo.toml
├── Cargo.lock
├── data/
│   ├── emojis.json          # Raw emoji data (source of truth)
│   └── build.rs             # Preprocess emojis → compact binary
├── src/
│   ├── main.rs              # App entry point (VERY small)
│   ├── app.rs               # GTK application wiring
│   ├── ui/
│   │   ├── mod.rs
│   │   └── window.rs        # Main picker window
│   ├── emoji/
│   │   ├── mod.rs
│   │   ├── db.rs            # Static emoji database
│   │   └── search.rs        # Fast search logic
│   └── clipboard.rs
└── README.md
```

## Release Recipe

```
flatpak-builder --force-clean --disable-rofiles-fuse build-dir nl.dibitat.emoji_picker.json
```
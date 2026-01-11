
# emoji-picker

The goal of this project is to (vibe)code a fast emoji-picker for GNOME. A common use case is that a user presses 'super'+'.', fuzzy searches the desired emoji based on the name and/or keywords in English or another localized form, and when the user presses 'return', the selected emoji (or Unicode character) is copied to the clipboard or inserted automatically.

I want the app to be focused on keyboard UX by making it superfast in start up, search and pasting. It should also get easily out of the way of the user. 

Recently used emojis are suggested in a recently used category.

The app should be able to follow (dark/light) system themes.

## Goal

I want to build a fast emoji-picker for GNOME.

## Development & Contribution Guidelines

Please refer to [RUST_INSTRUCTIONS.md](RUST_INSTRUCTIONS.md) for Rust best practices, project conventions, and contribution guidelines. Following these instructions helps maintain code quality and consistency.

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
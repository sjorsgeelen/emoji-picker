# emoji-picker
An emoji picker for GNOME

## Goal

I want to build a fast emoji-picker for GNOME.

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
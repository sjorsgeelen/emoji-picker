use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Emoji {
    pub ch: &'static str,
    pub name: &'static str,
    pub keywords: &'static [&'static str],
}

static EMOJIS: &[Emoji] = &[
    Emoji {
        ch: "😀",
        name: "grinning face",
        keywords: &["grin", "smile", "happy"],
    },
    Emoji {
        ch: "🚀",
        name: "rocket",
        keywords: &["launch", "ship", "fast"],
    },
];

pub static EMOJI_DB: Lazy<&'static [Emoji]> = Lazy::new(|| EMOJIS);
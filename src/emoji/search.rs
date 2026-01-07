use crate::emoji::db::{Emoji, EMOJI_DB};

pub fn search(query: &str) -> Vec<&'static Emoji> {
    if query.is_empty() {
        return EMOJI_DB.iter().take(40).collect();
    }

    let q = query.to_lowercase();

    EMOJI_DB
        .iter()
        .filter(|e| {
            e.name.contains(&q)
                || e.keywords.iter().any(|k| k.contains(&q))
        })
        .take(40)
        .collect()
}


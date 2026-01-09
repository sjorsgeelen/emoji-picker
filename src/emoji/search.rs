use crate::emoji::emoji_data::{Emoji, EMOJIS};

pub fn search(query: &str) -> Vec<&'static Emoji> {
    if query.is_empty() {
        return EMOJIS.iter().take(40).collect();
    }

    let q = query.to_lowercase();

    EMOJIS
        .iter()
        .filter(|e| {
            e.name_en.to_lowercase().contains(&q)
                || e.keywords_en.iter().any(|k| k.to_lowercase().contains(&q))
        })
        .take(40)
        .collect()
}


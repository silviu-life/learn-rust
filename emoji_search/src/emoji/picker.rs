use super::emoji::{Emoji, EMOJIS};

/// Searches and returns relevant emojis based on query terms.
pub struct EmojiPicker;

/// Query to the emoji picker.
pub enum Query{
    /// Search for emojis containing tag.
    ExactTag {
        /// Tag to search for.
        tag: String
    }
}

impl EmojiPicker {
    /// Searches for an emoji in the database.
    pub fn search(query: &Query) -> Vec<&Emoji> {
        match query {
            Query::ExactTag { tag } => search_exact_tag(tag)
        }
    }
}

fn search_exact_tag(tag: &String) -> Vec<&Emoji> {
    EMOJIS.iter().filter(|emoji| emoji.tags.iter().any(|&t| t == tag)).collect()
}
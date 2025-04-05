use std::{collections::HashSet, str::Matches};

use crate::emoji;

use super::emoji::{EMOJIS, Emoji};

/// Searches and returns relevant emojis based on query terms.
pub struct EmojiPicker;

/// Query to the emoji picker.
pub enum Query {
    /// Search for emojis containing tag.
    ExactTag {
        /// Tag to search for.
        tag: String,
    },
    /// Search for emojis with a specific alias.
    ExactAlias {
        /// Exact alias.
        alias: String,
    },
    /// Search for emojis with a specific alias via fuzzy search.
    FuzzyAlias {
        /// Alias to search for.
        alias: String,
    },
}

impl EmojiPicker {
    /// Searches for an emoji in the database.
    pub fn search(query: &Query) -> Vec<&Emoji> {
        match query {
            Query::ExactTag { tag } => search_exact_tag(tag),
            Query::ExactAlias { alias } => search_exact_alias(alias),
            Query::FuzzyAlias { alias } => search_fuzzy_alias(alias),
        }
    }
}

/// Searches an exact tag search.
fn search_exact_tag(tag: &String) -> Vec<&Emoji> {
    EMOJIS
        .iter()
        .filter(|emoji| emoji.tags.iter().any(|&t| t == tag))
        .collect()
}

/// Searches an exact alias search.
fn search_exact_alias(alias: &String) -> Vec<&Emoji> {
    EMOJIS
        .iter()
        .filter(|emoji| emoji.aliases.iter().any(|&t| t == alias))
        .collect()
}

/// Searches an alias via fuzzy search.
fn search_fuzzy_alias(alias: &String) -> Vec<&Emoji> {
    use rust_fuzzy_search::fuzzy_search_threshold;
    let threshold : f32 = 0.4f32;

    // 1. We first construct the list of all aliases we have available in our list of emojis.
    let mut all_aliases = HashSet::new();
    for emoji in EMOJIS {
        emoji.aliases.iter().for_each(|&alias| {
            all_aliases.insert(alias);
        });
    }
    let all_aliases: Vec<&str> = all_aliases.iter().cloned().collect();

    // 2. Then we get all the matches from fuzzy search.
    let matches = fuzzy_search_threshold(alias, &all_aliases, threshold);
    let matches: HashSet<&str> = matches.iter().map(|&(m, _)| m).collect();

    // 3. Finally, we reparse the emoji list to look for the matches.
    EMOJIS
        .iter()
        .filter(|emoji| emoji.aliases.iter().any(|&t| matches.contains(t)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_by_exact_tag_success() {
        let query = Query::ExactTag {
            tag: "happy".to_string(),
        };

        let result: Vec<&str> = EmojiPicker::search(&query)
            .iter()
            .map(|emoji| emoji.emoji)
            .collect();

        assert_eq!(result, vec!["😀", "😃", "😄", "😆"]);
    }

    #[test]
    fn search_by_exact_alias_success() {
        let query = Query::ExactAlias {
            alias: "joy".to_string(),
        };

        let result: Vec<&str> = EmojiPicker::search(&query)
            .iter()
            .map(|emoji| emoji.emoji)
            .collect();

        assert_eq!(result, vec!["😂"]);
    }

    #[test]
    fn search_by_fuzzy_alias_success() {
        let query = Query::FuzzyAlias {
            alias: "joy".to_string(),
        };

        let result: Vec<&str> = EmojiPicker::search(&query)
            .iter()
            .map(|emoji| emoji.emoji)
            .collect();

        assert_eq!(result, vec!["😂", "😹", "🕹️", "🇯🇴"]);
    }
}

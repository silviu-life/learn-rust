// &#127999; 🏿 Dark skin tone
// &#127998; 🏾 Medium Dark skin tone
// &#127997; 🏽 Medium skin tone
// &#127996; 🏼 Medium Light skin tone
// &#127995; 🏻 Light skin tone

/// Skin tone to apply to emoji.
enum SkinTone {
    /// No skin tone. Yellowish color.
    ///
    /// e.g. 👍
    None,

    /// Light skin tone.
    ///
    /// e.g. 👍🏻
    Light,

    /// Medium light skin tone.
    ///
    /// e.g. 👍🏼
    MediumLight,

    /// Medium skin tone.
    ///
    /// e.g. 👍🏽
    Medium,

    /// Medium dark skin tone.
    ///
    /// e.g. 👍🏾
    MediumDark,

    /// Dark skin tone.
    ///
    /// e.g. 👍🏿
    Dark,
}

impl SkinTone {
    /// Applies skin tone to the emoji.
    pub fn apply(&self, emoji: &str) -> String {
        match self.to_modifier() {
            Some(modifier) => format!("{}{}", emoji,  modifier),
            None => emoji.to_string(),
        }
    }

    fn to_modifier(&self) -> Option<char> {
        match self {
            SkinTone::None => None,
            SkinTone::Light => Some('\u{1F3FB}'),
            SkinTone::MediumLight => Some('\u{1F3FC}'),
            SkinTone::Medium => Some('\u{1F3FD}'),
            SkinTone::MediumDark => Some('\u{1F3FE}'),
            SkinTone::Dark => Some('\u{1F3FF}'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skin_tones_success() {
        assert_eq!(SkinTone::None.apply("👍"), "👍");
        assert_eq!(SkinTone::Light.apply("👍"), "👍🏻");
        assert_eq!(SkinTone::MediumLight.apply("👍"), "👍🏼");
        assert_eq!(SkinTone::Medium.apply("👍"), "👍🏽");
        assert_eq!(SkinTone::MediumDark.apply("👍"), "👍🏾");
        assert_eq!(SkinTone::Dark.apply("👍"), "👍🏿");
    }
}

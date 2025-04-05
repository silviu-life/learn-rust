import json;

def main():
    with open('data/emoji.json', 'r') as f:
        content = f.read()
        data = json.loads(content)
        print("""/// Represents an emoji with its metadata
#[derive(Debug, Clone)]
pub struct Emoji {
    /// The emoji character
    pub emoji: &'static str,
    /// Short description of the emoji
    pub description: &'static str,
    /// Category the emoji belongs to (e.g., "Smileys & Emotion")
    pub category: &'static str,
    /// All aliases for the emoji
    pub aliases: &'static [&'static str],
    /// Tags associated with the emoji for searching/grouping
    pub tags: &'static [&'static str],
    /// Unicode version when the emoji was introduced
    pub unicode_version: &'static str,
    /// iOS version when the emoji was supported
    pub ios_version: &'static str,
    /// Whether the emoji supports skin tones
    pub skin_tones: bool,
}
""")
        print('/// Full list of emojis. Kept as a constant list for speed 🚅.')
        print('pub static EMOJIS: &[Emoji] = &[')
        for emoji_data in data:
            print("    Emoji {")
            print('        emoji: "' +  emoji_data["emoji"] + '\",')
            print('        description: "' +  emoji_data["description"] + '\",')
            print('        category: "' +  emoji_data["category"] + '\",')
            print('        aliases: &[' +  ",".join(['"' + emoji + '"' for emoji in emoji_data["aliases"]]) + '],')
            print('        tags: &[' +  ",".join(['"' + tag + '"' for tag in emoji_data["tags"]]) + '],')
            print('        unicode_version: "' +  emoji_data["unicode_version"] + '\",')
            print('        ios_version: "' +  emoji_data["ios_version"] + '\",')
            print('        skin_tones: ' +  str(emoji_data.get("skin_tones", False)).lower() + ',')
            print("    },")
        print('];')
    pass

if __name__ == "__main__":
    main()
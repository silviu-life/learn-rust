use emoji::{picker::EmojiPicker, picker::Query};

mod emoji;

fn main() {
    for emoji in EmojiPicker::search(&Query::ExactTag { tag: "haha".to_string() }){
        print!("{}", emoji.emoji)
    }
    println!()
}

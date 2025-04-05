use emoji::{picker::EmojiPicker, picker::Query};

mod emoji;

fn main() {
    for emoji in EmojiPicker::search(&Query::ExactTag { tag: "happy".to_string() }){
        print!("{}", emoji.emoji);
        dbg!(emoji);
    }
    println!()
}

use emoji::emoji::EMOJIS;

mod emoji;

fn main() {
    for emoji in EMOJIS {
        print!("{}", emoji.emoji)
    }
    println!()
}

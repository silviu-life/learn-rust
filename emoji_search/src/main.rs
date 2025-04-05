use ui::app::App;

mod emoji;
mod ui;

fn main() {
    let app = App::new();
    app.run();
    app.close();
}

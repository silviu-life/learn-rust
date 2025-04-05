use std::{marker::PhantomData, thread::sleep, time::Duration};

/// App not initialized state.
pub struct New;

/// App initialized state.
pub struct Initialized;

/// Tui interface application.
pub struct App<State = New> {
    state : PhantomData<State>
}

impl App {
    /// Create a new app.
    pub fn new() -> App<Initialized> {
        let _ = ratatui::init();
        App { state: PhantomData }
    }
}

impl App<Initialized> {
    /// Run the app loop.
    pub fn run(&self) {
        // TODO(silviu): do something here.
        sleep(Duration::from_secs(2));
    }

    /// Close the app.
    pub fn close(self) {
        ratatui::restore();
    }
}
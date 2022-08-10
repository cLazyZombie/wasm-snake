use log;

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        log::debug!("draw");
    }
}

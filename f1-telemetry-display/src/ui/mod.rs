use f1_telemetry::Stream;

use crate::ui::{gtk::GTKUi, nc::NCUi};

mod gtk;
mod nc;

pub trait Ui {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&mut self, stream: Stream);
    fn destroy(&self);
}

pub fn get_ui(ui: &str) -> Box<dyn Ui> {
    match ui {
        "gtk" => Box::new(GTKUi::new()),
        "ncurses" => Box::new(NCUi::new()),
        _ => panic!("Invalid ui: {}", ui),
    }
}

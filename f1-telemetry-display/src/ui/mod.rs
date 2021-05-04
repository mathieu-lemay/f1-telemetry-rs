use f1_telemetry::Stream;

use crate::ui::{gtk::GtkUi, nc::NcursesUi};

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
        "gtk" => Box::new(GtkUi::new()),
        "ncurses" => Box::new(NcursesUi::new()),
        _ => panic!("Invalid ui: {}", ui),
    }
}

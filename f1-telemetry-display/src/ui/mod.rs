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

pub fn get_ui(gui: bool) -> Box<dyn Ui> {
    if gui {
        Box::new(GTKUi::new())
    } else {
        Box::new(NCUi::new())
    }
}

use f1_telemetry::Stream;

use crate::ui::{gtk::GTKUi, nc::NCUi};

mod gtk;
mod nc;

pub trait Ui {
    fn new(stream: Stream) -> Self
    where
        Self: Sized;
    fn run(&mut self);
    fn destroy(&self);
}

pub fn get_ui(stream: Stream, gui: bool) -> Box<dyn Ui> {
    if gui {
        Box::new(GTKUi::new(stream))
    } else {
        Box::new(NCUi::new(stream))
    }
}

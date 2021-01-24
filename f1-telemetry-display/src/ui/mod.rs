use async_trait::async_trait;

use crate::ui::{gtk::GtkUi, nc::NcursesUi};

mod gtk;
mod nc;

#[async_trait]
pub trait Ui {
    fn new() -> Self
    where
        Self: Sized;
    async fn run(&mut self);
    fn destroy(&self);
}

pub fn get_ui(ui: &str) -> Box<dyn Ui> {
    match ui {
        "gtk" => Box::new(GtkUi::new()),
        "ncurses" => Box::new(NcursesUi::new()),
        _ => panic!("Invalid ui: {}", ui),
    }
}

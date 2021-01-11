use crate::models::GameState;
use crate::ui::nc::NCUi;
use f1_telemetry::packet::Packet;

mod nc;

pub trait Ui {
    fn new() -> Self;
    fn destroy(&self);
    fn render(&mut self, game_state: &GameState, packet: &Packet);
    fn process_input(&mut self) -> bool;
}

pub fn get_ui() -> impl Ui {
    NCUi::new()
}

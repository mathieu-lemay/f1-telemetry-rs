use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

use crate::fmt;
use crate::models::GameState;

pub(super) struct HeaderView {
    container: gtk::Box,
    session_name: gtk::Label,
    lap_info: gtk::Label,
    session_time: gtk::Label,
}

impl HeaderView {
    pub(super) fn new() -> Self {
        let container = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        let session_name = create_label("session_name");
        let lap_info = create_label("lap_info");
        let session_time = create_label("session_time");

        container.pack_start(&session_name, false, false, 0);
        container.pack_start(&lap_info, false, false, 0);
        container.pack_start(&session_time, false, false, 0);

        // Dummy Data
        session_name.set_label("Race - Red Bull Ring");
        lap_info.set_label("Lap 1 of 5");
        session_time.set_label("00:01:03 / 02:00:00");

        Self {
            container,
            session_name,
            lap_info,
            session_time,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        let sinfo = &game_state.session_info;

        self.session_name.set_label(&fmt::get_session_name(sinfo));
        self.lap_info.set_label(&fmt::get_lap_count(sinfo));
        self.session_time.set_label(&fmt::get_session_time(sinfo));
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn create_label(name: &str) -> gtk::Label {
    gtk::Label::builder()
        .name(name)
        .halign(Align::Center)
        .build()
}

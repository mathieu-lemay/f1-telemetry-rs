use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

use crate::fmt;
use crate::models::GameState;

pub(super) struct ThrottleView {
    container: gtk::Grid,
    throttle_bar: gtk::LevelBar,
    brake_bar: gtk::LevelBar,
    speed_lbl: gtk::Label,
    gear_lbl: gtk::Label,
}

impl ThrottleView {
    pub(super) fn new() -> Self {
        let throttle_lbl = create_pedal_lbl("Throttle");
        let throttle_bar = create_pedal_bar("throttle");

        let brake_lbl = create_pedal_lbl("Brake");
        let brake_bar = create_pedal_bar("brake");

        let gear_speed_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();
        let speed_lbl = gtk::Label::builder()
            .name("speed")
            .halign(Align::Center)
            .build();
        let gear_lbl = gtk::Label::builder()
            .name("gear")
            .halign(Align::Center)
            .build();

        let container = gtk::Grid::builder()
            .row_spacing(12)
            .column_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.style_context().add_class("pedal_input");
        container.attach(&throttle_lbl, 0, 0, 1, 1);
        container.attach(&throttle_bar, 1, 0, 1, 1);
        container.attach(&brake_lbl, 0, 1, 1, 1);
        container.attach(&brake_bar, 1, 1, 1, 1);
        container.attach(&gear_speed_box, 2, 0, 1, 2);

        gear_speed_box.pack_start(&speed_lbl, false, false, 0);
        gear_speed_box.pack_start(&gear_lbl, false, false, 0);

        // Dummy data
        throttle_bar.set_value(0.75);
        brake_bar.set_value(0.25);
        speed_lbl.set_text(&fmt::format_speed(420));
        gear_lbl.set_text(&fmt::format_gear(8));

        Self {
            container,
            throttle_bar,
            brake_bar,
            speed_lbl,
            gear_lbl,
        }
    }

    pub(super) fn update(&self, games_state: &GameState) {
        let throttle = games_state.telemetry_info.throttle;
        self.throttle_bar.set_value(throttle as f64);

        let brake = games_state.telemetry_info.brake;
        self.brake_bar.set_value(brake as f64);

        self.speed_lbl
            .set_text(&fmt::format_speed(games_state.telemetry_info.speed));
        self.gear_lbl
            .set_text(&fmt::format_gear(games_state.telemetry_info.gear));
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn create_pedal_lbl(label: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(label)
        .halign(Align::Start)
        .build()
}

fn create_pedal_bar(name: &str) -> gtk::LevelBar {
    gtk::LevelBar::builder()
        .name(name)
        .min_value(0.0)
        .max_value(1.0)
        .hexpand(true)
        .build()
}

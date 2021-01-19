use crate::fmt::format_gear;
use crate::models::GameState;
use gtk::prelude::*;
use gtk::Align;

pub(super) struct ThrottleView {
    pub(super) container: gtk::Grid,
    throttle_bar: gtk::LevelBar,
    brake_bar: gtk::LevelBar,
    gear_value_lbl: gtk::Label,
}

impl ThrottleView {
    pub(super) fn new() -> Self {
        let throttle_lbl = create_pedal_lbl("Throttle");
        let throttle_bar = create_pedal_bar("throttle");

        let brake_lbl = create_pedal_lbl("Brake");
        let brake_bar = create_pedal_bar("brake");

        let gear_lbl = create_pedal_lbl("Gear");
        let gear_value_lbl = create_pedal_lbl("N");

        let container = gtk::GridBuilder::new()
            .row_spacing(12)
            .column_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.get_style_context().add_class("pedal_input");
        container.attach(&throttle_lbl, 0, 0, 1, 1);
        container.attach(&throttle_bar, 1, 0, 1, 1);
        container.attach(&brake_lbl, 0, 1, 1, 1);
        container.attach(&brake_bar, 1, 1, 1, 1);
        container.attach(&gear_lbl, 0, 2, 1, 1);
        container.attach(&gear_value_lbl, 1, 2, 1, 1);

        // Dummy data
        throttle_bar.set_value(0.75);
        brake_bar.set_value(0.25);

        Self {
            container,
            throttle_bar,
            brake_bar,
            gear_value_lbl,
        }
    }

    pub(super) fn update(&self, games_state: &GameState) {
        let throttle = games_state.telemetry_info.throttle;
        self.throttle_bar.set_value(throttle as f64);

        let brake = games_state.telemetry_info.brake;
        self.brake_bar.set_value(brake as f64);

        let gear = games_state.telemetry_info.gear;
        self.gear_value_lbl.set_text(&format_gear(gear))
    }
}

fn create_pedal_lbl(label: &str) -> gtk::Label {
    gtk::LabelBuilder::new()
        .label(label)
        .halign(Align::Start)
        .build()
}

fn create_pedal_bar(name: &str) -> gtk::LevelBar {
    gtk::LevelBarBuilder::new()
        .name(name)
        .min_value(0.0)
        .max_value(1.0)
        .hexpand(true)
        .build()
}

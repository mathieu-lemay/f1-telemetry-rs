use gtk::prelude::*;
use gtk::{Align, Widget};

use crate::fmt::{AsPercentage, AsWeight};
use crate::models::GameState;

pub(super) struct RaceDataView {
    container: gtk::Grid,
    average_tyre_wear: gtk::Label,
    average_fuel: gtk::Label,
}

impl RaceDataView {
    pub(super) fn new() -> Self {
        let tyre_wear_label = create_data_label("Avg Tyre Wear");
        let fuel_usage_label = create_data_label("Avg Fuel Usage");

        let average_tyre_wear = gtk::Label::builder()
            .name("wear")
            .halign(Align::Center)
            .build();
        let average_fuel = gtk::Label::builder()
            .name("fuel")
            .halign(Align::Center)
            .build();

        let container = gtk::Grid::builder()
            .row_spacing(12)
            .column_spacing(15)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.style_context().add_class("race_data");
        container.attach(&tyre_wear_label, 0, 0, 1, 1);
        container.attach(&average_tyre_wear, 1, 0, 1, 1);
        container.attach(&fuel_usage_label, 0, 1, 1, 1);
        container.attach(&average_fuel, 1, 1, 1, 1);

        // Dummy data
        average_tyre_wear.set_text(&0.0.as_percentage());
        average_fuel.set_text(&99.99.as_weight());

        Self {
            container,
            average_tyre_wear,
            average_fuel,
        }
    }

    pub fn update(&self, gamestate: &GameState) {
        let hrd = &gamestate.historical_race_data;
        if hrd.fuel_in_tank.len() > 1 {
            let fuel_usage: f32 = hrd
                .fuel_in_tank
                .windows(2)
                .map(|t| t[0].fuel_remaining - t[1].fuel_remaining)
                .sum::<f32>()
                / (hrd.fuel_in_tank.len() - 1) as f32;
            self.average_fuel.set_text(&fuel_usage.as_weight())
        }
        if hrd.tyre_damage.len() > 1 {
            let avg_tyre_wear: f32 = hrd
                .tyre_damage
                .windows(2)
                .map(|t| (t[1].sum()) as f32 - t[0].sum() as f32)
                .sum::<f32>()
                / ((hrd.tyre_damage.len() - 1) * 4) as f32;
            self.average_tyre_wear
                .set_text(&avg_tyre_wear.as_percentage())
            // self.average_tyre_wear
            //     .set_text(&(hrd.tyre_damage.len() as f32).as_percentage())
        } else {
            self.average_tyre_wear.set_text(&0.0f32.as_percentage())
        }
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn create_data_label(label: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(label)
        .halign(Align::Center)
        .build()
}

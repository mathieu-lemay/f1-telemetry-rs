use crate::models::GameState;
use gtk::{GridExt, ProgressBarExt, WidgetExt};

pub(super) struct ThrottleView {
    pub(super) container: gtk::Grid,
    throttle_bar: gtk::ProgressBar,
    brake_bar: gtk::ProgressBar,
}

impl ThrottleView {
    pub(super) fn new() -> Self {
        let throttle = gtk::ProgressBar::new();
        throttle.set_text(Some("Throttle"));
        throttle.set_widget_name("throttle");
        throttle.set_show_text(true);
        throttle.set_hexpand(true);

        let brake = gtk::ProgressBar::new();
        brake.set_text(Some("Brake"));
        brake.set_widget_name("brake");
        brake.set_show_text(true);
        brake.set_hexpand(true);

        let container = gtk::Grid::new();
        container.attach(&throttle, 0, 0, 1, 1);
        container.attach(&brake, 0, 1, 1, 1);
        container.set_row_spacing(12);
        container.set_vexpand(true);
        container.set_hexpand(true);

        // Dummy data
        throttle.set_fraction(0.75);
        brake.set_fraction(0.25);

        Self {
            container,
            throttle_bar: throttle,
            brake_bar: brake,
        }
    }

    pub(super) fn update(&self, games_state: &GameState) {
        let throttle = games_state.telemetry_info.throttle;
        self.throttle_bar.set_fraction(throttle as f64);

        let brake = games_state.telemetry_info.brake;
        self.brake_bar.set_fraction(brake as f64);
    }
}

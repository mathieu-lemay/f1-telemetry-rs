use crate::models::GameState;
use gtk::{GridExt, ProgressBarExt, StateFlags, WidgetExt};

pub(super) struct ThrottleView {
    pub(crate) container: gtk::Grid,
    _throttle_bar: gtk::ProgressBar,
    _brake_bar: gtk::ProgressBar,
}

impl ThrottleView {
    pub(super) fn new(_parent: &gtk::ApplicationWindow) -> Self {
        let throttle = gtk::ProgressBar::new();
        throttle.set_widget_name("throttle");
        throttle.set_text(Some("Throttle"));
        throttle.set_show_text(true);
        throttle.override_color(StateFlags::NORMAL, Some(&gdk::RGBA::green()));
        throttle.set_hexpand(true);

        let brake = gtk::ProgressBar::new();
        brake.set_text(Some("Brake"));
        brake.set_widget_name("brake");
        brake.set_show_text(true);
        brake.override_color(StateFlags::NORMAL, Some(&gdk::RGBA::red()));
        brake.set_hexpand(true);

        let container = gtk::Grid::new();
        container.attach(&throttle, 0, 0, 1, 1);
        container.attach(&brake, 0, 1, 1, 1);
        container.set_row_spacing(12);
        container.set_vexpand(true);
        container.set_hexpand(true);

        // parent.add(&container);
        Self {
            container,
            _throttle_bar: throttle,
            _brake_bar: brake,
        }
    }

    pub(super) fn update(&self, games_state: &GameState) {
        let throttle = games_state.telemetry_info.throttle;
        self._throttle_bar.set_fraction(throttle as f64);

        let brake = games_state.telemetry_info.brake;
        self._brake_bar.set_fraction(brake as f64);
    }
}

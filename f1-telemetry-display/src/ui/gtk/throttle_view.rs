use crate::models::GameState;
use gtk::{ContainerExt, ProgressBarExt};

pub(super) struct ThrottleView {
    _progress_bar: gtk::ProgressBar,
}

impl ThrottleView {
    pub(super) fn new(_parent: &gtk::ApplicationWindow) -> Self {
        let throttle = gtk::ProgressBar::new();
        throttle.set_text(Some("Throttle"));
        throttle.set_show_text(true);

        // parent.add(&throttle);
        Self {
            _progress_bar: throttle,
        }
    }

    pub(super) fn update(&self, games_state: &GameState) {
        let throttle = games_state.telemetry_info.throttle;
        self._progress_bar.set_fraction(throttle as f64)
    }
}

use gtk::{Align, BoxExt, Orientation, WidgetExt};

use crate::models::GameState;
use crate::ui::gtk::car;
use cairo::{Context, Format, ImageSurface};

pub struct CarView {
    pub(crate) container: gtk::Box,
    _car: Context,
}

impl CarView {
    pub fn new() -> Self {
        let car_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 350, 550).expect("Can't create surface");
        let cr = Context::new(&surface);

        init_car(&cr);

        let car = gtk::ImageBuilder::new().surface(&surface).build();
        car_box.pack_start(&car, false, false, 0);

        Self {
            container: car_box,
            _car: cr,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        let cs = &game_state.car_status;
        let front_left = cs.tyres_damage.front_left();
        let front_right = cs.tyres_damage.front_right();
        let rear_left = cs.tyres_damage.rear_left();
        let rear_right = cs.tyres_damage.rear_right();

        update_car(
            &self._car,
            front_left as f64,
            front_right as f64,
            rear_left as f64,
            rear_right as f64,
        );
        self.container.queue_draw();
    }
}

fn init_car(ctx: &Context) {
    car::draw_tyres(ctx, 0.0, 0.0, 0.0, 0.0);
}

fn update_car(ctx: &Context, fl_damage: f64, fr_damage: f64, rl_damage: f64, rr_damage: f64) {
    car::draw_tyres(ctx, fl_damage, fr_damage, rl_damage, rr_damage);
}

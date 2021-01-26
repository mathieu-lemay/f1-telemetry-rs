use cairo::{Context, Format, ImageSurface};
use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

use crate::models::GameState;
use crate::ui::gtk::car;

pub struct CarView {
    container: gtk::Box,
    _car: Context,
}

impl CarView {
    pub fn new() -> Self {
        let car_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 350, 600).expect("Can't create surface");
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
        let left_wing = cs.left_front_wing_damage;
        let right_wing = cs.right_front_wing_damage;
        let player = game_state.player_index as usize;
        let team = &game_state.participants[player].team;

        let color = get_color_from_team(team);
        update_car(
            &self._car,
            front_left as f64,
            front_right as f64,
            rear_left as f64,
            rear_right as f64,
            left_wing as f64,
            right_wing as f64,
            color,
        );
        self.container.queue_draw();
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn get_color_from_team(team: &Team) -> (f64, f64, f64) {
    get_cairo_team_color(team)
}

fn init_car(ctx: &Context) {
    car::draw_full_body(ctx, None);
    car::draw_tyres(ctx, 0.0, 0.0, 0.0, 0.0);
    car::draw_right_wing(ctx, 0.0);
    car::draw_left_wing(ctx, 0.0);
}

fn update_car(
    ctx: &Context,
    fl_damage: f64,
    fr_damage: f64,
    rl_damage: f64,
    rr_damage: f64,
    left_wing_damage: f64,
    right_wing_damage: f64,
    team_colour: (f64, f64, f64),
) {
    car::draw_tyres(ctx, fl_damage, fr_damage, rl_damage, rr_damage);
    car::draw_body(ctx, Some(team_colour));
    car::draw_right_wing(ctx, right_wing_damage);
    car::draw_left_wing(ctx, left_wing_damage);
}

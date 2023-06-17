use cairo::{Context, Format, ImageSurface};
use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

use f1_telemetry::packet::generic::Team;

use crate::models::GameState;
use crate::ui::gtk::car;

pub struct CarView {
    container: gtk::Box,
    _car: Context,
}

impl CarView {
    pub fn new() -> Result<Self, cairo::Error> {
        let car_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 350, 600).expect("Can't create surface");
        let cr = Context::new(&surface).expect("Unable to create cairo context.");

        init_car(&cr)?;

        let car = gtk::Image::builder().surface(&surface).build();
        car_box.pack_start(&car, false, false, 0);

        Ok(Self {
            container: car_box,
            _car: cr,
        })
    }

    pub(super) fn update(&self, game_state: &GameState) -> Result<(), cairo::Error> {
        let cs = &game_state.car_status;
        let front_left = cs.tyres_damage.front_left;
        let front_right = cs.tyres_damage.front_right;
        let rear_left = cs.tyres_damage.rear_left;
        let rear_right = cs.tyres_damage.rear_right;
        let left_wing = cs.left_front_wing_damage;
        let right_wing = cs.right_front_wing_damage;
        let player = game_state.player_index as usize;
        let team = if let Some(p) = &game_state.participants.get(player) {
            &p.team
        } else {
            &Team::Unknown
        };

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
        )?;
        self.container.queue_draw();

        Ok(())
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn get_color_from_team(team: &Team) -> (f64, f64, f64) {
    let color = match team {
        Team::Mercedes => (0, 53, 48),
        Team::Ferrari => (56, 0, 0),
        Team::RedBullRacing => (15, 0, 65),
        Team::Williams => (0, 33, 65),
        Team::RacingPoint => (62, 38, 51),
        Team::Renault => (65, 62, 0),
        Team::ToroRosso => (18, 40, 65),
        Team::Haas => (30, 30, 30),
        Team::McLaren => (65, 34, 0),
        Team::AlfaRomeo => (40, 0, 0),
        Team::AlphaTauri => (65, 65, 65),
        Team::MyTeam => (0, 150, 0),
        _ => (255, 255, 255),
    };
    let r = color.0 as f64 / 255.0;
    let g = color.1 as f64 / 255.0;
    let b = color.2 as f64 / 255.0;
    (r, g, b)
}

fn init_car(ctx: &Context) -> Result<(), cairo::Error> {
    car::draw_full_body(ctx, None)?;
    car::draw_tyres(ctx, 0.0, 0.0, 0.0, 0.0)?;
    car::draw_right_wing(ctx, 0.0)?;
    car::draw_left_wing(ctx, 0.0)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn update_car(
    ctx: &Context,
    fl_damage: f64,
    fr_damage: f64,
    rl_damage: f64,
    rr_damage: f64,
    left_wing_damage: f64,
    right_wing_damage: f64,
    team_colour: (f64, f64, f64),
) -> Result<(), cairo::Error> {
    car::draw_tyres(ctx, fl_damage, fr_damage, rl_damage, rr_damage)?;
    car::draw_body(ctx, Some(team_colour))?;
    car::draw_right_wing(ctx, right_wing_damage)?;
    car::draw_left_wing(ctx, left_wing_damage)?;

    Ok(())
}

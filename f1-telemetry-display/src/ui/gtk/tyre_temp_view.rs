use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

use crate::models::GameState;
use crate::ui::gtk::tyre_temp;
use crate::ui::gtk::tyre_temp::TyreTemp;

use super::cairo::{Context, Format, ImageSurface};

pub(super) struct TyreTempView {
    container: gtk::Box,
    _tyres: Context,
}

impl TyreTempView {
    pub(super) fn new() -> Self {
        let tyre_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 400, 600).expect("Can't create surface");
        let ctx = Context::new(&surface).expect("Unable to create cairo context.");

        init_tyres(&ctx);

        let car = gtk::Image::builder().surface(&surface).build();
        tyre_box.pack_start(&car, false, false, 0);

        Self {
            container: tyre_box,
            _tyres: ctx,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        let ti = &game_state.telemetry_info;
        let front_left = TyreTemp::from_surface_inner(
            ti.tyre_surface_temperature.front_left,
            ti.tyre_inner_temperature.front_left,
        );
        let front_right = TyreTemp::from_surface_inner(
            ti.tyre_surface_temperature.front_right,
            ti.tyre_inner_temperature.front_right,
        );
        let rear_left = TyreTemp::from_surface_inner(
            ti.tyre_surface_temperature.rear_left,
            ti.tyre_inner_temperature.rear_left,
        );
        let rear_right = TyreTemp::from_surface_inner(
            ti.tyre_surface_temperature.rear_right,
            ti.tyre_inner_temperature.rear_right,
        );
        update_tyres(
            &self._tyres,
            &front_left,
            &front_right,
            &rear_left,
            &rear_right,
        );

        self.container.queue_draw();
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn init_tyres(ctx: &Context) {
    let initial_temp = TyreTemp::from_surface_inner(90, 100);
    let res = tyre_temp::draw_tyres(
        ctx,
        &initial_temp,
        &initial_temp,
        &initial_temp,
        &initial_temp,
    );

    if let Err(e) = res {
        error!("Error drawing tyres: {:?}", e);
    }
}

fn update_tyres(
    ctx: &Context,
    fl_temp: &TyreTemp,
    fr_temp: &TyreTemp,
    rl_temp: &TyreTemp,
    rr_temp: &TyreTemp,
) {
    let res = tyre_temp::draw_tyres(ctx, fl_temp, fr_temp, rl_temp, rr_temp);

    if let Err(e) = res {
        error!("Error drawing tyres: {:?}", e);
    }
}

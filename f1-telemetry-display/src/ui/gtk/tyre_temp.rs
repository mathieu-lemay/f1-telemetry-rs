use crate::ui::gtk::car::draw_rounded_rectangle;

use super::cairo::Context;

const TYRE_WIDTH: f64 = 100.0;
const TYRE_HEIGHT: f64 = 150.0;
const TYRE_RADIUS: f64 = 20.0;

const WHEELBASE: f64 = 250.0;
const WHEELTRACK: f64 = 150.0;

const LEFT_FRONT_TYRE_X: f64 = 20.0;
const LEFT_FRONT_TYRE_Y: f64 = 100.0;

const RIGHT_FRONT_TYRE_X: f64 = LEFT_FRONT_TYRE_X + WHEELTRACK;
const RIGHT_FRONT_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y;

const LEFT_REAR_TYRE_X: f64 = LEFT_FRONT_TYRE_X;
const LEFT_REAR_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y + WHEELBASE;

const RIGHT_REAR_TYRE_X: f64 = LEFT_FRONT_TYRE_X + WHEELTRACK;
const RIGHT_REAR_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y + WHEELBASE;

const INNER_TEMP_BLUE_THRESHOLD: u16 = 75;
const INNER_TEMP_GREEN_LOWER_THRESHOLD: u16 = 80;
const INNER_TEMP_GREEN_UPPER_THRESHOLD: u16 = 103;
const INNER_TEMP_RED_THRESHOLD: u16 = 110;

const SURFACE_TEMP_BLUE_THRESHOLD: u16 = 65;
const SURFACE_TEMP_GREEN_LOWER_THRESHOLD: u16 = 75;
const SURFACE_TEMP_GREEN_UPPER_THRESHOLD: u16 = 125;
const SURFACE_TEMP_RED_THRESHOLD: u16 = 144;

#[derive(Default, Copy, Clone)]
pub struct TyreTemp {
    pub surface: u16,
    pub inner: u16,
}

impl TyreTemp {
    pub(crate) fn from_surface_inner(surface: u16, inner: u16) -> Self {
        Self { surface, inner }
    }
}

pub(crate) fn draw_tyres(
    ctx: &Context,
    fl_temp: &TyreTemp,
    fr_temp: &TyreTemp,
    rl_temp: &TyreTemp,
    rr_temp: &TyreTemp,
) {
    draw_left_front_tyre(ctx, fl_temp);
    draw_right_front_tyre(ctx, fr_temp);
    draw_left_rear_tyre(ctx, rl_temp);
    draw_right_rear_tyre(ctx, rr_temp);
    ctx.stroke()
}

pub fn draw_left_front_tyre(ctx: &Context, temp: &TyreTemp) {
    draw_tyre(ctx, LEFT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y, temp)
}

pub fn draw_right_front_tyre(ctx: &Context, temp: &TyreTemp) {
    draw_tyre(ctx, RIGHT_FRONT_TYRE_X, RIGHT_FRONT_TYRE_Y, temp)
}

pub fn draw_left_rear_tyre(ctx: &Context, temp: &TyreTemp) {
    draw_tyre(ctx, LEFT_REAR_TYRE_X, LEFT_REAR_TYRE_Y, temp)
}

pub fn draw_right_rear_tyre(ctx: &Context, temp: &TyreTemp) {
    draw_tyre(ctx, RIGHT_REAR_TYRE_X, RIGHT_REAR_TYRE_Y, temp)
}

fn draw_tyre(ctx: &Context, x_: f64, y_: f64, temp: &TyreTemp) {
    let (r, g, b) = get_tyre_inner_temp_colour(temp.inner);
    ctx.set_source_rgb(r, g, b);

    draw_rounded_rectangle(ctx, x_, y_, TYRE_WIDTH, TYRE_HEIGHT, TYRE_RADIUS);

    ctx.fill_preserve();

    let (r, g, b) = get_tyre_surface_temp_colour(temp.surface);
    ctx.set_source_rgb(r, g, b);
    ctx.set_line_width(7.0);
    ctx.stroke();
}

fn get_tyre_inner_temp_colour(temp: u16) -> (f64, f64, f64) {
    get_tyre_temp_colour_from_boundaries(
        INNER_TEMP_BLUE_THRESHOLD,
        INNER_TEMP_GREEN_LOWER_THRESHOLD,
        INNER_TEMP_GREEN_UPPER_THRESHOLD,
        INNER_TEMP_RED_THRESHOLD,
        temp,
    )
}

fn get_tyre_surface_temp_colour(temp: u16) -> (f64, f64, f64) {
    get_tyre_temp_colour_from_boundaries(
        SURFACE_TEMP_BLUE_THRESHOLD,
        SURFACE_TEMP_GREEN_LOWER_THRESHOLD,
        SURFACE_TEMP_GREEN_UPPER_THRESHOLD,
        SURFACE_TEMP_RED_THRESHOLD,
        temp,
    )
}

fn get_tyre_temp_colour_from_boundaries(
    blue: u16,
    green_lower: u16,
    green_upper: u16,
    red: u16,
    temp: u16,
) -> (f64, f64, f64) {
    match temp {
        t if t < blue => (0.0, 0.0, 0.8),
        t if t < green_lower => scale_blue_green(blue, green_lower, t),
        t if t < green_upper => (0.0, 0.8, 0.0),
        t if t < red => scale_green_red(green_upper, red, t),
        _ => (0.8, 0.0, 0.0),
    }
}

fn scale_blue_green(blue: u16, green: u16, marker: u16) -> (f64, f64, f64) {
    let (b, g) = scale_colours(blue, green, marker);
    (0.0, g, b)
}

fn scale_green_red(green: u16, red: u16, marker: u16) -> (f64, f64, f64) {
    let (g, r) = scale_colours(green, red, marker);
    (r, g, 0.0)
}

fn scale_colours(a: u16, b: u16, marker: u16) -> (f64, f64) {
    let max = 0.8;

    let position: f64 = (marker - a) as f64 / (b - a) as f64;

    match marker - a {
        c if c <= b - marker => (max, max * position * 2.0),
        _ => (max * (1.0 - (position - 0.5) * 2.0), max),
    }
}

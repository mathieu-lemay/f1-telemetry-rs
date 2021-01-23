use super::cairo::Context;

const YELLOW_TYRE_DAMAGE: f64 = 30.0;
const RED_TYRE_DAMAGE: f64 = 100.0;
const GREEN_DAMAGE_COLOR: (f64, f64, f64) = (0.0, 1.0, 0.0);
const YELLOW_DAMAGE_COLOR: (f64, f64, f64) = (1.0, 1.0, 0.0);
// const RED_DAMAGE_COLOR: (f64, f64, f64) = (1.0, 0.0, 0.0);

const TYRE_WIDTH: f64 = 50.0;
const TYRE_HEIGHT: f64 = 75.0;
const TYRE_RADIUS: f64 = 10.0;

const LEFT_FRONT_TYRE_X: f64 = 20.0;
const LEFT_FRONT_TYRE_Y: f64 = 50.0;

const RIGHT_FRONT_TYRE_X: f64 = 220.0;
const RIGHT_FRONT_TYRE_Y: f64 = 50.0;

const LEFT_REAR_TYRE_X: f64 = 20.0;
const LEFT_REAR_TYRE_Y: f64 = 450.0;

const RIGHT_REAR_TYRE_X: f64 = 220.0;
const RIGHT_REAR_TYRE_Y: f64 = 450.0;

pub(crate) fn draw_rounded_rectangle(
    ctx: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: f64,
) {
    ctx.new_sub_path();
    let angle_90 = 90.0_f64.to_radians();
    let angle_180 = 180.0_f64.to_radians();
    let angle_270 = 270.0_f64.to_radians();
    ctx.arc(x + width - radius, y + radius, radius, -angle_90, 0.0);
    ctx.arc(
        x + width - radius,
        y + height - radius,
        radius,
        0.0,
        angle_90,
    );
    ctx.arc(x + radius, y + height - radius, radius, angle_90, angle_180);
    ctx.arc(x + radius, y + radius, radius, angle_180, angle_270);
    ctx.close_path();
}

pub(crate) fn draw_left_front_tyre(ctx: &Context, damage: f64) {
    draw_tyre(ctx, LEFT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y, damage);
}

pub(crate) fn draw_right_front_tyre(ctx: &Context, damage: f64) {
    draw_tyre(ctx, RIGHT_FRONT_TYRE_X, RIGHT_FRONT_TYRE_Y, damage)
}

pub(crate) fn draw_left_rear_tyre(ctx: &Context, damage: f64) {
    draw_tyre(ctx, LEFT_REAR_TYRE_X, LEFT_REAR_TYRE_Y, damage)
}

pub(crate) fn draw_right_rear_tyre(ctx: &Context, damage: f64) {
    draw_tyre(ctx, RIGHT_REAR_TYRE_X, RIGHT_REAR_TYRE_Y, damage)
}

pub fn draw_tyres(ctx: &Context, fl_damage: f64, fr_damage: f64, rl_damage: f64, rr_damage: f64) {
    draw_left_front_tyre(ctx, fl_damage);
    draw_right_front_tyre(ctx, fr_damage);
    draw_left_rear_tyre(ctx, rl_damage);
    draw_right_rear_tyre(ctx, rr_damage);
    ctx.stroke()
}

fn draw_tyre(ctx: &Context, x_: f64, y_: f64, damage: f64) {
    let (r, g, b) = get_tyre_damage_colour(damage);
    ctx.set_source_rgb(r, g, b);

    draw_rounded_rectangle(ctx, x_, y_, TYRE_WIDTH, TYRE_HEIGHT, TYRE_RADIUS);

    ctx.fill()
}

fn get_tyre_damage_colour(damage: f64) -> (f64, f64, f64) {
    let r: f64;
    let mut g: f64 = GREEN_DAMAGE_COLOR.1;
    let b = 0.0;

    if damage <= YELLOW_TYRE_DAMAGE {
        let marker = damage / YELLOW_TYRE_DAMAGE;
        r = YELLOW_DAMAGE_COLOR.0 * marker;
    } else {
        let marker = (damage - YELLOW_TYRE_DAMAGE) / (RED_TYRE_DAMAGE - YELLOW_TYRE_DAMAGE);
        r = YELLOW_DAMAGE_COLOR.0;
        g = YELLOW_DAMAGE_COLOR.1 * (1.0 - marker);
    }
    (r, g, b)
}

#[cfg(test)]
mod test_get_tyre_damage_colour {
    use crate::ui::gtk::car::{
        get_tyre_damage_colour, GREEN_DAMAGE_COLOR, YELLOW_DAMAGE_COLOR, YELLOW_TYRE_DAMAGE,
    };

    #[test]
    fn test_0_damage_green() {
        let d = get_tyre_damage_colour(0.0);
        assert_eq!(d, GREEN_DAMAGE_COLOR)
    }

    #[test]
    fn test_yellow_damage_yellow_color() {
        let d = get_tyre_damage_colour(YELLOW_TYRE_DAMAGE);
        assert_eq!(d, YELLOW_DAMAGE_COLOR)
    }

    #[test]
    fn test_15_damage_returns_greeny_yelllow_color() {
        let d = get_tyre_damage_colour(15.0);
        assert_eq!(d, (0.5, 1.0, 0.0))
    }

    #[test]
    fn test_65_damage_returns_yellowy_red_color() {
        let d = get_tyre_damage_colour(65.0);
        assert_eq!(d, (1.0, 0.5, 0.0))
    }
}

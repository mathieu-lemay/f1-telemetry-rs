use super::cairo::Context;

const YELLOW_TYRE_DAMAGE: f64 = 30.0;
const RED_TYRE_DAMAGE: f64 = 100.0;
const GREEN_DAMAGE_COLOR: (f64, f64, f64) = (0.0, 0.8, 0.0);
const YELLOW_DAMAGE_COLOR: (f64, f64, f64) = (0.8, 0.8, 0.0);
// const RED_DAMAGE_COLOR: (f64, f64, f64) = (1.0, 0.0, 0.0);

const TYRE_WIDTH: f64 = 50.0;
const TYRE_HEIGHT: f64 = 75.0;
const TYRE_RADIUS: f64 = 10.0;

const WHEELBASE: f64 = 400.0;
const WHEELTRACK: f64 = 200.0;

const LEFT_FRONT_TYRE_X: f64 = 20.0;
const LEFT_FRONT_TYRE_Y: f64 = 100.0;

const RIGHT_FRONT_TYRE_X: f64 = LEFT_FRONT_TYRE_X + WHEELTRACK;
const RIGHT_FRONT_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y;

const LEFT_REAR_TYRE_X: f64 = LEFT_FRONT_TYRE_X;
const LEFT_REAR_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y + WHEELBASE;

const RIGHT_REAR_TYRE_X: f64 = LEFT_FRONT_TYRE_X + WHEELTRACK;
const RIGHT_REAR_TYRE_Y: f64 = LEFT_FRONT_TYRE_Y + WHEELBASE;

const LEFT_WING_HEIGHT: f64 = 50.0;
const LEFT_WING_WIDTH: f64 = (WHEELTRACK + TYRE_WIDTH) / 2.0;
const LEFT_WING_X: f64 = LEFT_FRONT_TYRE_X;
const LEFT_WING_Y: f64 = LEFT_FRONT_TYRE_Y - LEFT_WING_HEIGHT - 10.0;
const LEFT_WING_NOSE_RADIUS: f64 = 15.0;

const YELLOW_WING_DAMAGE: f64 = 30.0;

fn draw_left_wing_shape(ctx: &Context) {
    ctx.new_sub_path();
    let angle_90 = 90.0_f64.to_radians();
    let angle_180 = 180.0_f64.to_radians();

    ctx.move_to(LEFT_WING_X, LEFT_WING_Y);
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS,
        LEFT_WING_Y,
    );
    ctx.arc(
        LEFT_WING_X + LEFT_WING_WIDTH,
        LEFT_WING_Y,
        LEFT_WING_NOSE_RADIUS,
        angle_180,
        -angle_90,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH,
        LEFT_WING_Y + LEFT_WING_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH * 0.8,
        LEFT_WING_Y + LEFT_WING_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH * 0.4,
        LEFT_WING_Y + LEFT_WING_HEIGHT,
    );
    ctx.line_to(LEFT_WING_X, LEFT_WING_Y + LEFT_WING_HEIGHT);
    ctx.line_to(LEFT_WING_X, LEFT_WING_Y);
    ctx.close_path();
}

fn draw_right_wing_shape(ctx: &Context) {
    ctx.new_sub_path();
    let angle_180 = 180.0_f64.to_radians();

    ctx.move_to(LEFT_WING_X + 2.0 * LEFT_WING_WIDTH, LEFT_WING_Y);
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS,
        LEFT_WING_Y,
    );
    ctx.arc(
        LEFT_WING_X + LEFT_WING_WIDTH,
        LEFT_WING_Y,
        LEFT_WING_NOSE_RADIUS,
        angle_180,
        0.0,
    );
    ctx.move_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS,
        LEFT_WING_Y,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH,
        LEFT_WING_Y + LEFT_WING_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH * 1.2,
        LEFT_WING_Y + LEFT_WING_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH * 1.6,
        LEFT_WING_Y + LEFT_WING_HEIGHT,
    );
    ctx.line_to(
        LEFT_WING_X + 2.0 * LEFT_WING_WIDTH,
        LEFT_WING_Y + LEFT_WING_HEIGHT,
    );
    ctx.line_to(LEFT_WING_X + 2.0 * LEFT_WING_WIDTH, LEFT_WING_Y);

    ctx.close_path();
}

pub(crate) fn draw_left_wing(ctx: &Context, damage: f64) {
    let (r, g, b) = get_wing_damage_colour(damage);
    ctx.set_source_rgb(r, g, b);

    draw_left_wing_shape(ctx);

    ctx.fill()
}
pub(crate) fn draw_right_wing(ctx: &Context, damage: f64) {
    let (r, g, b) = get_wing_damage_colour(damage);
    ctx.set_source_rgb(r, g, b);

    draw_right_wing_shape(ctx);

    ctx.fill()
}

pub(crate) fn draw_full_body(ctx: &Context, team_colour: Option<(f64, f64, f64)>) {
    draw_suspension(ctx);
    draw_body(ctx, team_colour);
}

pub(crate) fn draw_body(ctx: &Context, team_colour: Option<(f64, f64, f64)>) {
    let (r, g, b) = (1.0, 1.0, 1.0);
    ctx.set_source_rgb(r, g, b);
    draw_body_shape(ctx);

    ctx.fill_preserve();
    let (r, g, b) = team_colour.unwrap_or((0.8, 0.8, 0.8));
    ctx.set_source_rgb(r * 3.0, g * 3.0, b * 3.0);
    ctx.set_line_width(5.0);
    ctx.stroke();
}

fn draw_suspension(ctx: &Context) {
    ctx.set_source_rgb(0.8, 0.8, 0.8);
    draw_suspension_shape(ctx);
    ctx.fill();
}

fn draw_suspension_shape(ctx: &Context) {
    //TOP SUS BAR
    ctx.new_sub_path();
    ctx.move_to(LEFT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4);
    ctx.rel_line_to(LEFT_WING_WIDTH, -TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(-LEFT_WING_WIDTH, TYRE_HEIGHT * 0.2);
    ctx.close_path();

    ctx.new_sub_path();
    ctx.move_to(
        RIGHT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4,
    );
    ctx.rel_line_to(-LEFT_WING_WIDTH, -TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(LEFT_WING_WIDTH, TYRE_HEIGHT * 0.2);
    ctx.close_path();

    //BOTTOM SUS BAR
    ctx.new_sub_path();
    ctx.move_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4,
    );
    ctx.line_to(
        LEFT_FRONT_TYRE_X + LEFT_WING_WIDTH / 2.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.8,
    );
    ctx.rel_line_to(LEFT_WING_WIDTH / 2.0, 0.0);
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(-LEFT_WING_WIDTH / 2.0 - 2.0, 0.0);
    ctx.line_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.6,
    );

    ctx.close_path();

    ctx.new_sub_path();
    ctx.move_to(RIGHT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4);
    ctx.line_to(
        LEFT_FRONT_TYRE_X + LEFT_WING_WIDTH * 3.0 / 2.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.8,
    );
    ctx.rel_line_to(-LEFT_WING_WIDTH / 2.0, 0.0);
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.rel_line_to(LEFT_WING_WIDTH / 2.0 + 2.0, 0.0);
    ctx.line_to(RIGHT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.6);

    ctx.close_path();

    //MID SUS BAR

    ctx.new_sub_path();
    ctx.move_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4,
    );
    ctx.line_to(
        LEFT_FRONT_TYRE_X + LEFT_WING_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.8,
    );
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.line_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.6,
    );

    ctx.close_path();

    ctx.new_sub_path();
    ctx.move_to(RIGHT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.4);
    ctx.line_to(
        LEFT_FRONT_TYRE_X + LEFT_WING_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.8,
    );
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.line_to(RIGHT_FRONT_TYRE_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 0.6);

    ctx.close_path();

    //REAR SUS BAR

    ctx.new_sub_path();
    ctx.move_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_REAR_TYRE_Y + TYRE_HEIGHT * 0.4,
    );
    ctx.line_to(RIGHT_REAR_TYRE_X, LEFT_REAR_TYRE_Y + TYRE_HEIGHT * 0.4);
    ctx.rel_line_to(0.0, TYRE_HEIGHT * 0.2);
    ctx.line_to(
        LEFT_FRONT_TYRE_X + TYRE_WIDTH,
        LEFT_REAR_TYRE_Y + TYRE_HEIGHT * 0.6,
    );
    ctx.close_path()
}

fn draw_body_shape(ctx: &Context) {
    ctx.new_sub_path();
    //LEFT NOSE
    ctx.move_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS,
        LEFT_WING_Y + 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS - 3.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH - LEFT_WING_NOSE_RADIUS - 10.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 1.5,
    );

    //LEFT BARGEBOARD
    ctx.line_to(LEFT_WING_X, LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 2.0);
    ctx.line_to(LEFT_WING_X, LEFT_REAR_TYRE_Y - TYRE_HEIGHT * 0.5);

    //LEFT REAR
    ctx.line_to(
        LEFT_WING_X + TYRE_WIDTH * 1.5,
        LEFT_REAR_TYRE_Y - TYRE_HEIGHT * 0.5,
    );
    ctx.line_to(
        LEFT_WING_X + TYRE_WIDTH * 1.5,
        LEFT_REAR_TYRE_Y + TYRE_HEIGHT * 0.75,
    );

    //REAR
    ctx.line_to(
        RIGHT_REAR_TYRE_X - TYRE_WIDTH * 0.5,
        LEFT_REAR_TYRE_Y + TYRE_HEIGHT * 0.75,
    );

    //RIGHT REAR
    ctx.line_to(
        RIGHT_REAR_TYRE_X - TYRE_WIDTH * 0.5,
        LEFT_REAR_TYRE_Y - TYRE_HEIGHT * 0.5,
    );
    ctx.line_to(
        RIGHT_REAR_TYRE_X + TYRE_WIDTH,
        LEFT_REAR_TYRE_Y - TYRE_HEIGHT * 0.5,
    );

    //RIGHT BARGEBOARD
    ctx.line_to(
        RIGHT_REAR_TYRE_X + TYRE_WIDTH,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 2.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH + LEFT_WING_NOSE_RADIUS + 10.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT * 1.5,
    );

    //RIGHT NOSE
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH + LEFT_WING_NOSE_RADIUS + 3.0,
        LEFT_FRONT_TYRE_Y + TYRE_HEIGHT / 3.0,
    );
    ctx.line_to(
        LEFT_WING_X + LEFT_WING_WIDTH + LEFT_WING_NOSE_RADIUS,
        LEFT_WING_Y + 3.0,
    );

    ctx.close_path();
}

fn get_wing_damage_colour(damage: f64) -> (f64, f64, f64) {
    get_damage_color(damage, YELLOW_WING_DAMAGE, 100.0)
}

fn get_damage_color(
    damage: f64,
    yellow_damage_level: f64,
    red_damage_level: f64,
) -> (f64, f64, f64) {
    let r: f64;
    let mut g: f64 = GREEN_DAMAGE_COLOR.1;
    let b = 0.0;

    if damage <= yellow_damage_level {
        let marker = damage / yellow_damage_level;
        r = YELLOW_DAMAGE_COLOR.0 * marker;
    } else {
        let marker = (damage - yellow_damage_level) / (red_damage_level - yellow_damage_level);
        r = YELLOW_DAMAGE_COLOR.0;
        g = YELLOW_DAMAGE_COLOR.1 * (1.0 - marker);
    }
    (r, g, b)
}

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
    get_damage_color(damage, YELLOW_TYRE_DAMAGE, RED_TYRE_DAMAGE)
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
        assert_eq!(d, (0.4, 0.8, 0.0))
    }

    #[test]
    fn test_65_damage_returns_yellowy_red_color() {
        let d = get_tyre_damage_colour(65.0);
        assert_eq!(d, (0.8, 0.4, 0.0))
    }
}

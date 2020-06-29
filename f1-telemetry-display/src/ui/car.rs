use super::fmt;
use crate::models::CarStatus;
use ncurses::*;

const LEFT_FRONT_WING: &str = "\
___________/--
||||||||||||||
|_________/--|
|            |";

const RIGHT_FRONT_WING: &str = "\
--\\___________
||||||||||||||
|--\\_________|
|            |";

const TYRE: &str = " _____
|     |
|     |
|     |
|_____|";

const BODY: &str = "      /  \\
     /    \\
----/      \\----
---/        \\---
  /          \\
 /            \\
 |            |
 |            |
 |            |
 \\            /
  \\          /
---|        |---
---|        |---
   |        |   ";

const REAR_WING: &str = "\
|------------------|
|------------------|
|------------------|";

pub fn render_car(w: WINDOW, car_status: &CarStatus, y: i32, x: i32) {
    render_component(
        w,
        LEFT_FRONT_WING,
        car_status.left_front_wing_damage,
        y,
        x + 1,
    );
    render_component(
        w,
        RIGHT_FRONT_WING,
        car_status.right_front_wing_damage,
        y,
        x + 15,
    );
    render_component(w, BODY, 0, y + 4, x + 7);
    render_component(w, REAR_WING, car_status.rear_wing_damage, y + 18, x + 5);

    render_component(w, TYRE, car_status.tyres_damage.front_left(), y + 4, x);
    render_component(
        w,
        TYRE,
        car_status.tyres_damage.front_right(),
        y + 4,
        x + 22,
    );
    render_component(w, TYRE, car_status.tyres_damage.rear_left(), y + 13, x);
    render_component(
        w,
        TYRE,
        car_status.tyres_damage.rear_right(),
        y + 13,
        x + 22,
    );
}

fn render_component(w: WINDOW, component: &str, damage: u8, y: i32, x: i32) {
    fmt::set_damage_color(Some(w), damage);

    for (i, l) in component.split('\n').enumerate() {
        mvwaddstr(w, y + i as i32, x, l);
    }

    fmt::wreset(w);
}

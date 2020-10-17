use super::car::{render_component, TYRE};
use crate::models::{MotionInfo, TelemetryInfo};
use crate::ui::fmt;
use f1_telemetry::packet::generic::WheelData;
use f1_telemetry::packet::motion::MotionData;
use ncurses::{mvwaddstr, werase, WINDOW};

const SUSP_RIGHT_NORMAL: &str = "
   O
    \\
    \\ \\
     \\/\\
      \\/\\
       \\/\\
        \\ \\
          \\
           O


    ";
const SUSP_RIGHT_EXTENDED: &str = "
   O
    \\
    \\ \\
     \\/\\
      \\ \\
       \\/\\
        \\ \\
         \\/\\
          \\ \\
            \\
             O
    ";
const SUSP_RIGHT_COMPRESSED: &str = "
   O
    \\
    \\/\\
     \\/\\
      \\/\\
        \\
         O




    ";
const SUSP_LEFT_NORMAL: &str = "
           O
          /
        / /
       /\\/
      /\\/
     /\\/
    / /
    /
   O


    ";
const SUSP_LEFT_EXTENDED: &str = "
           O
          /
        / /
       /\\/
      / /
     /\\/
    / /
   /\\/
  / /
  /
 O
    ";
const SUSP_LEFT_COMPRESSED: &str = "
           O
          /
        /\\/
       /\\/
      /\\/
      /
     O




    ";

pub(crate) const TEMPTYRE: &str = " _____
|     |
|     |
|     |
|_____|";

pub(crate) fn render_suspension(
    w: WINDOW,
    motion_info: &MotionInfo,
    telemetry_info: &TelemetryInfo,
) {
    werase(w);
    let left_front_susp = get_left_component(motion_info.suspension_position.front_left());
    let left_rear_susp = get_left_component(motion_info.suspension_position.rear_left());
    let right_front_susp = get_right_component(motion_info.suspension_position.front_right());
    let right_rear_susp = get_right_component(motion_info.suspension_position.rear_right());

    let front_left_temp = telemetry_info.tyre_surface_temperature.front_left();
    let front_right_temp = telemetry_info.tyre_surface_temperature.front_right();
    let rear_left_temp = telemetry_info.tyre_surface_temperature.rear_left();
    let rear_right_temp = telemetry_info.tyre_surface_temperature.rear_right();
    let temp_offsety = 6;
    let temp_offsetx = 1;

    render_component(w, TEMPTYRE, front_left_temp as u8, 3, 1);

    let front_left_str = format!("T: {:01.1}", front_left_temp);
    mvwaddstr(w, 3 + temp_offsety, 1 + temp_offsetx, &front_left_str);

    render_component(w, &left_front_susp, 0, 1, 10);
    render_component(w, &right_front_susp, 0, 1, 31);

    render_component(w, TEMPTYRE, front_right_temp as u8, 3, 50);

    let front_right_str = format!("T: {:01.1}", front_right_temp);
    mvwaddstr(w, 3 + temp_offsety, 50 + temp_offsetx, &front_right_str);

    let offset = 12;
    render_component(w, TEMPTYRE, rear_left_temp as u8, 3 + offset, 1);

    let rear_left_str = format!("T: {:01.1}", rear_left_temp);
    mvwaddstr(
        w,
        3 + offset + temp_offsety,
        1 + temp_offsetx,
        &rear_left_str,
    );

    render_component(w, &left_rear_susp, 0, 1 + offset, 10);
    render_component(w, &right_rear_susp, 0, 1 + offset, 31);

    render_component(w, TEMPTYRE, rear_right_temp as u8, 3 + offset, 50);

    let rear_right_str = format!("T: {:01.1}", rear_right_temp);
    mvwaddstr(
        w,
        3 + offset + temp_offsety,
        50 + temp_offsetx,
        &rear_right_str,
    );
}

fn get_left_component(suspension_position: f32) -> String {
    let component = match suspension_position {
        s if s < -4.0 => SUSP_LEFT_EXTENDED,
        s if s >= 10.0 => SUSP_LEFT_COMPRESSED,
        s if s >= -4.0 => SUSP_LEFT_NORMAL,
        _ => SUSP_LEFT_NORMAL,
    };
    component.to_owned()
}

fn get_right_component(suspension_position: f32) -> String {
    let component = match suspension_position {
        s if s < -4.0 => SUSP_RIGHT_EXTENDED,
        s if s >= 10.0 => SUSP_RIGHT_COMPRESSED,
        s if s >= -4.0 => SUSP_RIGHT_NORMAL,
        _ => SUSP_RIGHT_NORMAL,
    };
    component.to_owned()
}

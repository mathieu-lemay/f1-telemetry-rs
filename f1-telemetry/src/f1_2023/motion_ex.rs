use std::io::BufRead;

use serde::Deserialize;

use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::motion_ex::PacketMotionExData;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// The motion packet gives physics data for all the cars being driven. There is additional data for
/// the car being driven with the goal of being able to drive a motion platform setup.
///
/// Frequency: Rate as specified in menus
/// Size: 217 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                     Header
/// suspension_position:        Note: All wheel arrays have the following order:
/// suspension_velocity:        RL, RR, FL, FR
/// suspension_acceleration:    RL, RR, FL, FR
/// wheel_speed:                Speed of each wheel
/// wheel_slip_ratio:           Slip ratio for each wheel
/// wheel_slip_angle:           Slip angle for each wheel
/// wheel_lat_force:            Lateral forces for each wheel
/// wheel_long_force:           Longitudinal forces for each wheel
/// height_of_cog_above_ground: Height of centre of gravity above ground
/// local_velocity_x:           Velocity in local space (m/s)
/// local_velocity_y:           Velocity in local space (m/s)
/// local_velocity_z:           Velocity in local space (m/s)
/// angular_velocity_x:         Angular velocity x-component (radians/s)
/// angular_velocity_y:         Angular velocity y-component (radians/s)
/// angular_velocity_z:         Angular velocity z-component (radians/s)
/// angular_acceleration_x:     Angular acceleration x-component (radians/s)
/// angular_acceleration_y:     Angular acceleration y-component (radians/s)
/// angular_acceleration_z:     Angular acceleration z-component (radians/s)
/// front_wheels_angle:         Current front wheels angle in radians
/// wheel_vertical_force:       Vertical forces for each wheel
/// ```
#[derive(Deserialize)]
struct RawMotionExData {
    suspension_position: WheelData<f32>,
    suspension_velocity: WheelData<f32>,
    suspension_acceleration: WheelData<f32>,
    wheel_speed: WheelData<f32>,
    wheel_slip_ratio: WheelData<f32>,
    wheel_slip_angle: WheelData<f32>,
    wheel_lat_force: WheelData<f32>,
    wheel_long_force: WheelData<f32>,
    height_of_cog_above_ground: f32,
    local_velocity_x: f32,
    local_velocity_y: f32,
    local_velocity_z: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    angular_acceleration_x: f32,
    angular_acceleration_y: f32,
    angular_acceleration_z: f32,
    front_wheels_angle: f32,
    wheel_vertical_force: WheelData<f32>,
}

pub(crate) fn parse_motion_ex_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionExData, UnpackError> {
    assert_packet_size(size, MOTION_EX_PACKET_SIZE)?;

    let motion_data: RawMotionExData = bincode::deserialize_from(reader)?;

    Ok(PacketMotionExData {
        header,
        suspension_position: motion_data.suspension_position,
        suspension_velocity: motion_data.suspension_velocity,
        suspension_acceleration: motion_data.suspension_acceleration,
        wheel_speed: motion_data.wheel_speed,
        wheel_slip_ratio: motion_data.wheel_slip_ratio,
        wheel_slip_angle: motion_data.wheel_slip_angle,
        wheel_lat_force: motion_data.wheel_lat_force,
        wheel_long_force: motion_data.wheel_long_force,
        height_of_center_of_gravity: motion_data.height_of_cog_above_ground,
        local_velocity_x: motion_data.local_velocity_x,
        local_velocity_y: motion_data.local_velocity_y,
        local_velocity_z: motion_data.local_velocity_z,
        angular_velocity_x: motion_data.angular_velocity_x,
        angular_velocity_y: motion_data.angular_velocity_y,
        angular_velocity_z: motion_data.angular_velocity_z,
        angular_acceleration_x: motion_data.angular_acceleration_x,
        angular_acceleration_y: motion_data.angular_acceleration_y,
        angular_acceleration_z: motion_data.angular_acceleration_z,
        front_wheels_angle: motion_data.front_wheels_angle,
        wheel_vertical_force: motion_data.wheel_vertical_force,
    })
}

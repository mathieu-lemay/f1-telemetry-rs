use std::io::BufRead;

use serde::Deserialize;

use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::motion::{CarMotionData, PacketMotionData, PlayerCarData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// The motion packet gives physics data for all the cars being driven. There is additional data for
/// the car being driven with the goal of being able to drive a motion platform setup.
///
/// Frequency: Rate as specified in menus
/// Size: 1464 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                 Header
/// motion_data:            List of motion data (22)
///
/// # Extra player car ONLY data
/// suspension_position:     Note: All wheel arrays have the following order:
/// suspension_velocity:     RL, RR, FL, FR
/// suspension_acceleration: RL, RR, FL, FR
/// wheel_speed:             Speed of each wheel
/// wheel_slip:              Slip ratio for each wheel
/// local_velocity_x:        Velocity in local space
/// local_velocity_y:        Velocity in local space
/// local_velocity_z:        Velocity in local space
/// angular_velocity_x:      Angular velocity x-component
/// angular_velocity_y:      Angular velocity y-component
/// angular_velocity_z:      Angular velocity z-component
/// angular_acceleration_x:  Angular acceleration x-component
/// angular_acceleration_y:  Angular acceleration y-component
/// angular_acceleration_z:  Angular acceleration z-component
/// front_wheels_angle:      Current front wheels angle in radians
/// ```
#[derive(Deserialize)]
struct RawMotionData {
    car_motion: [RawCarMotion; NUMBER_CARS],
    suspension_position: WheelData<f32>,
    suspension_velocity: WheelData<f32>,
    suspension_acceleration: WheelData<f32>,
    wheel_speed: WheelData<f32>,
    wheel_slip: WheelData<f32>,
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
}

/// ## Specification
/// ```text
/// world_position_x:     World space X position
/// world_position_y:     World space Y position
/// world_position_z:     World space Z position
/// world_velocity_x:     Velocity in world space X
/// world_velocity_y:     Velocity in world space Y
/// world_velocity_z:     Velocity in world space Z
/// world_forward_dir_x:  World space forward X direction (normalised)
/// world_forward_dir_y:  World space forward Y direction (normalised)
/// world_forward_dir_z:  World space forward Z direction (normalised)
/// world_right_dir_x:    World space right X direction (normalised)
/// world_right_dir_y:    World space right Y direction (normalised)
/// world_right_dir_z:    World space right Z direction (normalised)
/// g_force_lateral:      Lateral G-Force component
/// g_force_longitudinal: Longitudinal G-Force component
/// g_force_vertical:     Vertical G-Force component
/// yaw:                  Yaw angle in radians
/// pitch:                Pitch angle in radians
/// roll:                 Roll angle in radians
/// ```
#[derive(Deserialize)]
struct RawCarMotion {
    world_position_x: f32,
    world_position_y: f32,
    world_position_z: f32,
    world_velocity_x: f32,
    world_velocity_y: f32,
    world_velocity_z: f32,
    world_forward_dir_x: i16,
    world_forward_dir_y: i16,
    world_forward_dir_z: i16,
    world_right_dir_x: i16,
    world_right_dir_y: i16,
    world_right_dir_z: i16,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl From<&RawCarMotion> for CarMotionData {
    fn from(car_motion: &RawCarMotion) -> Self {
        Self {
            world_position_x: car_motion.world_position_x,
            world_position_y: car_motion.world_position_y,
            world_position_z: car_motion.world_position_z,
            world_velocity_x: car_motion.world_velocity_x,
            world_velocity_y: car_motion.world_velocity_y,
            world_velocity_z: car_motion.world_velocity_z,
            world_forward_dir_x: car_motion.world_forward_dir_x,
            world_forward_dir_y: car_motion.world_forward_dir_y,
            world_forward_dir_z: car_motion.world_forward_dir_z,
            world_right_dir_x: car_motion.world_right_dir_x,
            world_right_dir_y: car_motion.world_right_dir_y,
            world_right_dir_z: car_motion.world_right_dir_z,
            g_force_lateral: car_motion.g_force_lateral,
            g_force_longitudinal: car_motion.g_force_longitudinal,
            g_force_vertical: car_motion.g_force_vertical,
            yaw: car_motion.yaw,
            pitch: car_motion.pitch,
            roll: car_motion.roll,
        }
    }
}

impl From<RawMotionData> for PlayerCarData {
    fn from(motion_data: RawMotionData) -> Self {
        Self {
            suspension_position: motion_data.suspension_position,
            suspension_velocity: motion_data.suspension_velocity,
            suspension_acceleration: motion_data.suspension_acceleration,
            wheel_speed: motion_data.wheel_speed,
            wheel_slip: motion_data.wheel_slip,
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
        }
    }
}

pub(crate) fn parse_motion_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionData, UnpackError> {
    assert_packet_size(size, MOTION_PACKET_SIZE)?;

    let motion_data: RawMotionData = bincode::deserialize_from(reader)?;

    let car_motion = motion_data.car_motion.iter().map(|cm| cm.into()).collect();

    Ok(PacketMotionData {
        header,
        motion_data: car_motion,
        player_car_data: Some(motion_data.into()),
    })
}

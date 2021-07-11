use std::io::BufRead;

use serde::Deserialize;

use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::motion::{CarMotionData, PacketMotionData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

/// The motion packet gives physics data for all the cars being driven. There is additional data for
/// the car being driven with the goal of being able to drive a motion platform setup.
///
/// Frequency: Rate as specified in menus
/// Size: 1343 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                 Header
/// motion_data:            List of motion data (20)
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
pub(super) struct RawMotionData {
    car_motion: [RawCarMotion; 20],
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

impl PacketMotionData {
    pub(super) fn from(header: PacketHeader, motion_data: RawMotionData) -> Self {
        Self::new(
            header,
            motion_data
                .car_motion
                .iter()
                .map(CarMotionData::from)
                .collect(),
            motion_data.suspension_position,
            motion_data.suspension_velocity,
            motion_data.suspension_acceleration,
            motion_data.wheel_speed,
            motion_data.wheel_slip,
            motion_data.local_velocity_x,
            motion_data.local_velocity_y,
            motion_data.local_velocity_z,
            motion_data.angular_velocity_x,
            motion_data.angular_velocity_y,
            motion_data.angular_velocity_z,
            motion_data.angular_acceleration_x,
            motion_data.angular_acceleration_y,
            motion_data.angular_acceleration_z,
            motion_data.front_wheels_angle,
        )
    }
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

impl CarMotionData {
    fn from(car_motion: &RawCarMotion) -> Self {
        Self::new(
            car_motion.world_position_x,
            car_motion.world_position_y,
            car_motion.world_position_z,
            car_motion.world_velocity_x,
            car_motion.world_velocity_y,
            car_motion.world_velocity_z,
            car_motion.world_forward_dir_x,
            car_motion.world_forward_dir_y,
            car_motion.world_forward_dir_z,
            car_motion.world_right_dir_x,
            car_motion.world_right_dir_y,
            car_motion.world_right_dir_z,
            car_motion.g_force_lateral,
            car_motion.g_force_longitudinal,
            car_motion.g_force_vertical,
            car_motion.yaw,
            car_motion.pitch,
            car_motion.roll,
        )
    }
}
pub(crate) fn parse_motion_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionData, UnpackError> {
    assert_packet_size(size, MOTION_PACKET_SIZE)?;

    let motion_data: RawMotionData = bincode::deserialize_from(reader)?;

    Ok(PacketMotionData::from(header, motion_data))
}

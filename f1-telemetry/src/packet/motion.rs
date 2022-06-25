use serde::Serialize;

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the `car_motion_data` array of the [`PacketMotionData`] type.
///
/// N.B. For the normalised vectors below, to convert to float values divide by 32767.0f – 16-bit
/// signed values are used to pack the data and on the assumption that direction values are always
/// between -1.0f and 1.0f.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CarMotionData {
    /// World space X position
    pub world_position_x: f32,
    /// World space Y position
    pub world_position_y: f32,
    /// World space Z position
    pub world_position_z: f32,
    /// Velocity in world space X
    pub world_velocity_x: f32,
    /// Velocity in world space Y
    pub world_velocity_y: f32,
    /// Velocity in world space Z
    pub world_velocity_z: f32,
    /// World space forward X direction (normalised)
    pub world_forward_dir_x: i16,
    /// World space forward Y direction (normalised)
    pub world_forward_dir_y: i16,
    /// World space forward Z direction (normalised)
    pub world_forward_dir_z: i16,
    /// World space right X direction (normalised)
    pub world_right_dir_x: i16,
    /// World space right Y direction (normalised)
    pub world_right_dir_y: i16,
    /// World space right Z direction (normalised)
    pub world_right_dir_z: i16,
    /// Lateral G-Force component
    pub g_force_lateral: f32,
    /// Longitudinal G-Force component
    pub g_force_longitudinal: f32,
    /// Vertical G-Force component
    pub g_force_vertical: f32,
    /// Yaw angle in radians
    pub yaw: f32,
    /// Pitch angle in radians
    pub pitch: f32,
    /// Roll angle in radians
    pub roll: f32,
}

/// Data specific to the car being driven. This data is provided with the goal of being able to
/// drive a motion platform setup
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlayerCarData {
    /// Position of the suspension
    pub suspension_position: WheelData<f32>,
    /// Velocity of the suspension
    pub suspension_velocity: WheelData<f32>,
    /// Acceleration of the suspension
    pub suspension_acceleration: WheelData<f32>,
    /// Speed of each wheel
    pub wheel_speed: WheelData<f32>,
    /// Slip ratio for each wheel
    pub wheel_slip: WheelData<f32>,
    /// Velocity in local space
    pub local_velocity_x: f32,
    /// Velocity in local space
    pub local_velocity_y: f32,
    /// Velocity in local space
    pub local_velocity_z: f32,
    /// Angular velocity x-component
    pub angular_velocity_x: f32,
    /// Angular velocity y-component
    pub angular_velocity_y: f32,
    /// Angular velocity z-component
    pub angular_velocity_z: f32,
    /// Angular acceleration x-component
    pub angular_acceleration_x: f32,
    /// Angular acceleration y-component
    pub angular_acceleration_y: f32,
    /// Angular acceleration z-component
    pub angular_acceleration_z: f32,
    /// Current front wheels angle in radians
    pub front_wheels_angle: f32,
}

/// The motion packet gives physics data for all the cars being driven. There is additional data for
/// the car being driven with the goal of being able to drive a motion platform setup.
///
/// Frequency: Rate as specified in menus
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PacketMotionData {
    /// Packet Header
    pub header: PacketHeader,
    /// List of motion data
    pub motion_data: Vec<CarMotionData>,
    /// Extra data specific to the player's car
    pub player_car_data: PlayerCarData,
}

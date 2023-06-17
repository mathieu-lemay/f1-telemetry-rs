use serde::Serialize;

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// The motion packet gives extended data for the car being driven with the goal of
/// being able to drive a motion platform setup.
/// This packet was introduced in F1 23. It replaces
/// [`PacketMotionData::player_car_data`](super::motion::PacketMotionData::player_car_data).
///
/// Frequency: Rate as specified in menus
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketMotionExData {
    /// Packet Header
    pub header: PacketHeader,
    /// Position of the suspension
    pub suspension_position: WheelData<f32>,
    /// Velocity of the suspension
    pub suspension_velocity: WheelData<f32>,
    /// Acceleration of the suspension
    pub suspension_acceleration: WheelData<f32>,
    /// Speed of each wheel
    pub wheel_speed: WheelData<f32>,
    /// Slip ratio for each wheel
    pub wheel_slip_ratio: WheelData<f32>,
    /// Slip angle for each wheel
    pub wheel_slip_angle: WheelData<f32>,
    /// Lateral forces for each wheel
    pub wheel_lat_force: WheelData<f32>,
    /// Longitudinal forces for each wheel
    pub wheel_long_force: WheelData<f32>,
    /// Height of centre of gravity above ground
    pub height_of_center_of_gravity: f32,
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
    /// Vertical forces for each wheel
    pub wheel_vertical_force: WheelData<f32>,
}

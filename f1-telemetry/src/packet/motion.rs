use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the `car_motion_data` array of the [`PacketMotionData`] type.
///
/// N.B. For the normalised vectors below, to convert to float values divide by 32767.0f – 16-bit
/// signed values are used to pack the data and on the assumption that direction values are always
/// between -1.0f and 1.0f.
///
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
#[derive(Debug, PartialEq)]
pub struct CarMotionData {
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,
    pub world_velocity_x: f32,
    pub world_velocity_y: f32,
    pub world_velocity_z: f32,
    pub world_forward_dir_x: i16,
    pub world_forward_dir_y: i16,
    pub world_forward_dir_z: i16,
    pub world_right_dir_x: i16,
    pub world_right_dir_y: i16,
    pub world_right_dir_z: i16,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

/// The motion packet gives physics data for all the cars being driven. There is additional data for
/// the car being driven with the goal of being able to drive a motion platform setup.
///
/// Frequency: Rate as specified in menus
///
/// ## Specification
/// ```text
/// header:                 Header
/// motion_data:            List of motion data
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
#[derive(Debug, PartialEq)]
pub struct PacketMotionData {
    pub header: PacketHeader,
    pub motion_data: Vec<CarMotionData>,

    // Extra player car ONLY data
    pub suspension_position: WheelData<f32>,
    pub suspension_velocity: WheelData<f32>,
    pub suspension_acceleration: WheelData<f32>,
    pub wheel_speed: WheelData<f32>,
    pub wheel_slip: WheelData<f32>,
    pub local_velocity_x: f32,
    pub local_velocity_y: f32,
    pub local_velocity_z: f32,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub angular_acceleration_x: f32,
    pub angular_acceleration_y: f32,
    pub angular_acceleration_z: f32,
    pub front_wheels_angle: f32,
}

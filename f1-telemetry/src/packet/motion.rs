use getset::{CopyGetters, Getters};

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

/// This type is used for the 20-element `motion_data` array of the [`PacketMotionData`] type.
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
///
/// [`PacketMotionData`]: ./struct.MotionData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct MotionData {
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

impl MotionData {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
    ) -> MotionData {
        MotionData {
            world_position_x,
            world_position_y,
            world_position_z,
            world_velocity_x,
            world_velocity_y,
            world_velocity_z,
            world_forward_dir_x,
            world_forward_dir_y,
            world_forward_dir_z,
            world_right_dir_x,
            world_right_dir_y,
            world_right_dir_z,
            g_force_lateral,
            g_force_longitudinal,
            g_force_vertical,
            yaw,
            pitch,
            roll,
        }
    }
}

/// The motion packet gives physics data for all the cars being driven.
///
/// There is additional data for the car being driven with the goal of being able to drive a motion platform setup.
///
/// N.B. For the normalised vectors below, to convert to float values divide by 32767.0f – 16-bit signed values are
/// used to pack the data and on the assumption that direction values are always between -1.0f and 1.0f.
///
/// Frequency: Rate as specified in menus
///
/// Size: 1343 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:          Header
/// motion_data: List of motion data (20)
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
#[derive(Debug, CopyGetters, Getters)]
pub struct PacketMotionData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get = "pub")]
    motion_data: Vec<MotionData>,

    // Extra player car ONLY data
    #[getset(get_copy = "pub")]
    suspension_position: WheelData<f32>,
    #[getset(get_copy = "pub")]
    suspension_velocity: WheelData<f32>,
    #[getset(get_copy = "pub")]
    suspension_acceleration: WheelData<f32>,
    #[getset(get_copy = "pub")]
    wheel_speed: WheelData<f32>,
    #[getset(get_copy = "pub")]
    wheel_slip: WheelData<f32>,
    #[getset(get_copy = "pub")]
    local_velocity_x: f32,
    #[getset(get_copy = "pub")]
    local_velocity_y: f32,
    #[getset(get_copy = "pub")]
    local_velocity_z: f32,
    #[getset(get_copy = "pub")]
    angular_velocity_x: f32,
    #[getset(get_copy = "pub")]
    angular_velocity_y: f32,
    #[getset(get_copy = "pub")]
    angular_velocity_z: f32,
    #[getset(get_copy = "pub")]
    angular_acceleration_x: f32,
    #[getset(get_copy = "pub")]
    angular_acceleration_y: f32,
    #[getset(get_copy = "pub")]
    angular_acceleration_z: f32,
    #[getset(get_copy = "pub")]
    front_wheels_angle: f32,
}

impl PacketMotionData {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        header: PacketHeader,
        motion_data: Vec<MotionData>,
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
    ) -> PacketMotionData {
        PacketMotionData {
            header,
            motion_data,
            suspension_position,
            suspension_velocity,
            suspension_acceleration,
            wheel_speed,
            wheel_slip,
            local_velocity_x,
            local_velocity_y,
            local_velocity_z,
            angular_velocity_x,
            angular_velocity_y,
            angular_velocity_z,
            angular_acceleration_x,
            angular_acceleration_y,
            angular_acceleration_z,
            front_wheels_angle,
        }
    }
}

use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::motion::{MotionData, PacketMotionData};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn parse_motion<T: BufRead>(reader: &mut T) -> Result<MotionData, UnpackError> {
    let world_position_x = reader.read_f32::<LittleEndian>().unwrap();
    let world_position_y = reader.read_f32::<LittleEndian>().unwrap();
    let world_position_z = reader.read_f32::<LittleEndian>().unwrap();
    let world_velocity_x = reader.read_f32::<LittleEndian>().unwrap();
    let world_velocity_y = reader.read_f32::<LittleEndian>().unwrap();
    let world_velocity_z = reader.read_f32::<LittleEndian>().unwrap();
    let world_forward_dir_x = reader.read_i16::<LittleEndian>().unwrap();
    let world_forward_dir_y = reader.read_i16::<LittleEndian>().unwrap();
    let world_forward_dir_z = reader.read_i16::<LittleEndian>().unwrap();
    let world_right_dir_x = reader.read_i16::<LittleEndian>().unwrap();
    let world_right_dir_y = reader.read_i16::<LittleEndian>().unwrap();
    let world_right_dir_z = reader.read_i16::<LittleEndian>().unwrap();
    let g_force_lateral = reader.read_f32::<LittleEndian>().unwrap();
    let g_force_longitudinal = reader.read_f32::<LittleEndian>().unwrap();
    let g_force_vertical = reader.read_f32::<LittleEndian>().unwrap();
    let yaw = reader.read_f32::<LittleEndian>().unwrap();
    let pitch = reader.read_f32::<LittleEndian>().unwrap();
    let roll = reader.read_f32::<LittleEndian>().unwrap();

    Ok(MotionData::new(
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
    ))
}

pub(crate) fn parse_motion_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionData, UnpackError> {
    assert_packet_size(size, MOTION_PACKET_SIZE)?;

    let mut motion_data = Vec::with_capacity(NUMBER_CARS);
    for _ in 0..NUMBER_CARS {
        let md = parse_motion(&mut reader)?;
        motion_data.push(md);
    }

    let suspension_position = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );

    let suspension_velocity = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );

    let suspension_acceleration = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );

    let wheel_speed = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );

    let wheel_slip = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );

    let local_velocity_x = reader.read_f32::<LittleEndian>().unwrap();
    let local_velocity_y = reader.read_f32::<LittleEndian>().unwrap();
    let local_velocity_z = reader.read_f32::<LittleEndian>().unwrap();
    let angular_velocity_x = reader.read_f32::<LittleEndian>().unwrap();
    let angular_velocity_y = reader.read_f32::<LittleEndian>().unwrap();
    let angular_velocity_z = reader.read_f32::<LittleEndian>().unwrap();
    let angular_acceleration_x = reader.read_f32::<LittleEndian>().unwrap();
    let angular_acceleration_y = reader.read_f32::<LittleEndian>().unwrap();
    let angular_acceleration_z = reader.read_f32::<LittleEndian>().unwrap();
    let front_wheels_angle = reader.read_f32::<LittleEndian>().unwrap();

    Ok(PacketMotionData::new(
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
    ))
}

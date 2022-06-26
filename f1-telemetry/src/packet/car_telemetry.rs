use serde::Serialize;

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum SurfaceType {
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
    Unknown,
}

impl Default for SurfaceType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// This type is used for the `car_telemetry_data` array of the [`PacketCarTelemetryData`] type.
///
/// ## Specification
/// ```text
/// speed                     Speed of car in kilometres per hour
/// throttle                  Amount of throttle applied (0.0 to 1.0)
/// steer                     Steering (-1.0 (full lock left) to 1.0 (full lock right))
/// brake                     Amount of brake applied (0 to 1.0)
/// clutch                    Amount of clutch applied (0 to 100)
/// gear                      Gear selected (1-8, N=0, R=-1)
/// engine_rpm                Engine RPM
/// drs                       DRS activated
/// rev_lights_percent        Rev lights indicator (percentage)
/// brakes_temperature        Brakes temperature (celsius)
/// tyres_surface_temperature Tyres surface temperature (celsius)
/// tyres_inner_temperature   Tyres inner temperature (celsius)
/// engine_temperature        Engine temperature (celsius)
/// tyre_pressures            Tyres pressure (PSI)
/// surface_type              Driving surface, see [`SurfaceType`]
/// ```
///
/// See also [`SurfaceType`]
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub rev_lights_bit_value: Option<u16>,
    pub brakes_temperature: WheelData<u16>,
    pub tyres_surface_temperature: WheelData<u16>,
    pub tyres_inner_temperature: WheelData<u16>,
    pub engine_temperature: u16,
    pub tyre_pressures: WheelData<f32>,
    pub surface_types: WheelData<SurfaceType>,
}

/// Bit-mask values for the `button_status` field in [`PacketCarTelemetryData`]
///
/// These flags are used in the telemetry packet to determine if any buttons are being held on the
/// controlling device. If the value below logical ANDed with the button status is set then the
/// corresponding button is being held.
///
/// ## Specification
/// ```text
/// Bit Flag            Button
/// 0x0001              Cross or A
/// 0x0002              Triangle or Y
/// 0x0004              Circle or B
/// 0x0008              Square or X
/// 0x0010              D-pad Left
/// 0x0020              D-pad Right
/// 0x0040              D-pad Up
/// 0x0080              D-pad Down
/// 0x0100              Options or Menu
/// 0x0200              L1 or LB
/// 0x0400              R1 or RB
/// 0x0800              L2 or LT
/// 0x1000              R2 or RT
/// 0x2000              Left Stick Click
/// 0x4000              Right Stick Click
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum ButtonFlag {
    Cross = 0x0001,
    Triangle = 0x0002,
    Circle = 0x0004,
    Square = 0x0008,
    DPadLeft = 0x0010,
    DPadRight = 0x0020,
    DPadUp = 0x0040,
    DPadDown = 0x0080,
    Options = 0x0100,
    L1 = 0x0200,
    R1 = 0x0400,
    L2 = 0x0800,
    R2 = 0x1000,
    LeftStickClick = 0x2000,
    RightStickClick = 0x4000,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum MFDPanel {
    CarSetup,
    Pits,
    Damage,
    Engine,
    Temperatures,
    Closed,
    NotSet,
}

/// This packet details telemetry for all the cars in the race.
///
/// It details various values that would be recorded on the car such as speed, throttle application,
/// DRS etc.
///
/// Frequency: Rate as specified in menus
///
/// ## Specification
/// ```text
/// header:             Header
/// car_telemetry_data: List of car telemetry
/// button_status:      Bit flags specifying which buttons are being pressed currently.
///                     See [`ButtonFlag`]
///                     Available in F1 2019 and F1 2020 only. For F1 2021 and above,
///                     this information is available as an event packet.
/// ```
/// See also [`ButtonFlag`]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,
    pub car_telemetry_data: Vec<CarTelemetryData>,
    pub button_status: Option<u32>,
    pub mfd_panel: MFDPanel,
    pub secondary_player_mfd_panel: MFDPanel,
    pub suggested_gear: Option<i8>,
}

impl PacketCarTelemetryData {
    pub fn get_pressed_buttons(&self) -> Vec<ButtonFlag> {
        let mask = self.button_status.unwrap_or_default();
        let mut buttons = Vec::new();

        if mask == 0 {
            return buttons;
        }

        if mask & (ButtonFlag::Cross as u32) > 0 {
            buttons.push(ButtonFlag::Cross);
        }
        if mask & (ButtonFlag::Triangle as u32) > 0 {
            buttons.push(ButtonFlag::Triangle);
        }
        if mask & (ButtonFlag::Circle as u32) > 0 {
            buttons.push(ButtonFlag::Circle);
        }
        if mask & (ButtonFlag::Square as u32) > 0 {
            buttons.push(ButtonFlag::Square);
        }
        if mask & (ButtonFlag::DPadLeft as u32) > 0 {
            buttons.push(ButtonFlag::DPadLeft);
        }
        if mask & (ButtonFlag::DPadRight as u32) > 0 {
            buttons.push(ButtonFlag::DPadRight);
        }
        if mask & (ButtonFlag::DPadUp as u32) > 0 {
            buttons.push(ButtonFlag::DPadUp);
        }
        if mask & (ButtonFlag::DPadDown as u32) > 0 {
            buttons.push(ButtonFlag::DPadDown);
        }
        if mask & (ButtonFlag::Options as u32) > 0 {
            buttons.push(ButtonFlag::Options);
        }
        if mask & (ButtonFlag::L1 as u32) > 0 {
            buttons.push(ButtonFlag::L1);
        }
        if mask & (ButtonFlag::R1 as u32) > 0 {
            buttons.push(ButtonFlag::R1);
        }
        if mask & (ButtonFlag::L2 as u32) > 0 {
            buttons.push(ButtonFlag::L2);
        }
        if mask & (ButtonFlag::R2 as u32) > 0 {
            buttons.push(ButtonFlag::R2);
        }
        if mask & (ButtonFlag::LeftStickClick as u32) > 0 {
            buttons.push(ButtonFlag::LeftStickClick);
        }
        if mask & (ButtonFlag::RightStickClick as u32) > 0 {
            buttons.push(ButtonFlag::RightStickClick);
        }

        buttons
    }
}

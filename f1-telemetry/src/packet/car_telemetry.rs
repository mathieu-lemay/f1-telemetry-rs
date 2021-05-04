use getset::{CopyGetters, Getters};

use crate::packet::generic::WheelData;

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

/// This type is used for the 20-element `car_telemetry_data` array of the [`PacketCarTelemetryData`] type.
///
/// ## Specification
/// speed                     Speed of car in kilometres per hour
/// throttle                  Amount of throttle applied (0.0 to 1.0)
/// steer                     Steering (-1.0 (full lock left) to 1.0 (full lock right))
/// brake                     Amount of brake applied (0 to 1.0)
/// clutch                    Amount of clutch applied (0 to 100)
/// gear                      Gear selected (1-8, N=0, R=-1)
/// engine_rpm                Engine RPM
/// drs                       0 = off, 1 = on
/// rev_lights_percent        Rev lights indicator (percentage)
/// brakes_temperature        Brakes temperature (celsius)
/// tyres_surface_temperature Tyres surface temperature (celsius)
/// tyres_inner_temperature   Tyres inner temperature (celsius)
/// engine_temperature        Engine temperature (celsius)
/// tyre_pressures            Tyres pressure (PSI)
/// surface_type              Driving surface, see appendices
///
/// [`PacketCarTelemetryData`]: ./struct.CarTelemetryData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8,
    engine_rpm: u16,
    drs: bool,
    rev_lights_percent: u8,
    brakes_temperature: WheelData<u16>,
    tyres_surface_temperature: WheelData<u16>,
    tyres_inner_temperature: WheelData<u16>,
    engine_temperature: u16,
    tyre_pressures: WheelData<f32>,
    surface_types: WheelData<SurfaceType>,
}

impl CarTelemetryData {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        speed: u16,
        throttle: f32,
        steer: f32,
        brake: f32,
        clutch: u8,
        gear: i8,
        engine_rpm: u16,
        drs: bool,
        rev_lights_percent: u8,
        brakes_temperature: WheelData<u16>,
        tyres_surface_temperature: WheelData<u16>,
        tyres_inner_temperature: WheelData<u16>,
        engine_temperature: u16,
        tyre_pressures: WheelData<f32>,
        surface_types: WheelData<SurfaceType>,
    ) -> CarTelemetryData {
        CarTelemetryData {
            speed,
            throttle,
            steer,
            brake,
            clutch,
            gear,
            engine_rpm,
            drs,
            rev_lights_percent,
            brakes_temperature,
            tyres_surface_temperature,
            tyres_inner_temperature,
            engine_temperature,
            tyre_pressures,
            surface_types,
        }
    }
}

/// Bit-mask values for the `button_status` field in [`PacketCarTelemetryData`]
///
/// [`PacketCarTelemetryData`]: ./struct.CarTelemetryData.html
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
#[derive(Debug, Copy, Clone)]
pub enum MFDPanel {
    CarSetup,
    Pits,
    Damage,
    Engine,
    Temperatures,
    Closed,
}

/// This packet details telemetry for all the cars in the race.
///
/// It details various values that would be recorded on the car such as speed, throttle application, DRS etc.
///
/// Frequency: Rate as specified in menus
///
/// Size: 1347 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:             Header
/// car_telemetry_data: List of car telemetry (20)
/// button_status:      Bit flags specifying which buttons are being
///                     pressed currently - see appendices
/// ```
#[derive(Debug, CopyGetters, Getters)]
pub struct PacketCarTelemetryData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get = "pub")]
    car_telemetry_data: Vec<CarTelemetryData>,
    #[getset(get_copy = "pub")]
    button_status: u32,
    #[getset(get_copy = "pub")]
    mfd_panel: MFDPanel,
    #[getset(get_copy = "pub")]
    secondary_player_mfd_panel: MFDPanel,
    #[getset(get_copy = "pub")]
    suggested_gear: i8,
}

impl PacketCarTelemetryData {
    pub(crate) fn from_2019(
        header: PacketHeader,
        car_telemetry_data: Vec<CarTelemetryData>,
        button_status: u32,
    ) -> PacketCarTelemetryData {
        PacketCarTelemetryData {
            header,
            car_telemetry_data,
            button_status,
            mfd_panel: MFDPanel::Closed,
            secondary_player_mfd_panel: MFDPanel::Closed,
            suggested_gear: 0,
        }
    }
    pub(crate) fn from_2020(
        header: PacketHeader,
        car_telemetry_data: Vec<CarTelemetryData>,
        button_status: u32,
        mfd_panel: MFDPanel,
        secondary_player_mfd_panel: MFDPanel,
        suggested_gear: i8,
    ) -> PacketCarTelemetryData {
        PacketCarTelemetryData {
            header,
            car_telemetry_data,
            button_status,
            mfd_panel,
            secondary_player_mfd_panel,
            suggested_gear,
        }
    }

    pub fn get_pressed_buttons(&self) -> Vec<ButtonFlag> {
        let mask = self.button_status;
        let mut buttons = Vec::new();

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

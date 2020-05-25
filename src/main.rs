use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{BufRead, Cursor};
use std::mem;
use std::net::UdpSocket;

//struct UnpackError(&'static str);
#[derive(Debug)]
struct UnpackError(String);

enum Packet {
    Session(PacketSessionData),
}

#[derive(Debug)]
enum PacketID {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
}

impl TryFrom<u8> for PacketID {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketID::Motion),
            1 => Ok(PacketID::Session),
            2 => Ok(PacketID::LapData),
            3 => Ok(PacketID::Event),
            4 => Ok(PacketID::Participants),
            5 => Ok(PacketID::CarSetups),
            6 => Ok(PacketID::CarTelemetry),
            7 => Ok(PacketID::CarStatus),
            _ => Err(UnpackError(format!("Invalid PacketID: {}", value))),
        }
    }
}

/// The header for each of the UDP telemetry packets.
///
/// ## Specification
/// ```text
/// packet_format:      2019
/// game_major_version: game major version - "x.00"
/// game_minor_version: game minor version - "1.xX"
/// packet_version:     version of this packet type, all start from 1
/// packet_id:          identifier for the packet type, see [`PacketID`]
/// session_uid:        unique identifier for the session
/// session_time:       session timestamp
/// frame_identifier:   identifier for the frame the data was retrieved on
/// player_car_index:   index of player's car in the array
/// ```
///
/// [`PacketID`]: ./enum.PacketID.html
#[derive(Debug)]
struct PacketHeader {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    player_car_index: u8,
}

impl PacketHeader {
    pub fn new<T: BufRead>(reader: &mut T) -> PacketHeader {
        let packet_format = reader.read_u16::<LittleEndian>().unwrap();
        let game_major_version = reader.read_u8().unwrap();
        let game_minor_version = reader.read_u8().unwrap();
        let packet_version = reader.read_u8().unwrap();
        let packet_id = reader.read_u8().unwrap();
        let session_uid = reader.read_u64::<LittleEndian>().unwrap();
        let session_time = reader.read_f32::<LittleEndian>().unwrap();
        let frame_identifier = reader.read_u32::<LittleEndian>().unwrap();
        let player_car_index = reader.read_u8().unwrap();

        PacketHeader {
            packet_format,
            game_major_version,
            game_minor_version,
            packet_version,
            packet_id,
            session_uid,
            session_time,
            frame_identifier,
            player_car_index,
        }
    }
}

#[derive(Debug)]
enum Flag {
    None,
    Green,
    Blue,
    Yellow,
    Red,
    Invalid,
}

impl TryFrom<i8> for Flag {
    type Error = UnpackError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Flag::None),
            1 => Ok(Flag::Green),
            2 => Ok(Flag::Blue),
            3 => Ok(Flag::Yellow),
            4 => Ok(Flag::Red),
            -1 => Ok(Flag::Invalid),
            _ => Err(UnpackError(format!("Invalid Flag value: {}", value))),
        }
    }
}

/// This type is used for the 21-element 'marshalZones' array of the [`PacketSessionData`] type, defined below.
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts
/// zone_flag:  -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
/// ```
/// [`PacketSessionData`]: ./struct.PacketSessionData.html
#[derive(Debug)]
struct MarshalZone {
    zone_start: f32,
    zone_flag: Flag,
}

impl MarshalZone {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<MarshalZone, UnpackError> {
        let zone_start = reader.read_f32::<LittleEndian>().unwrap();
        let zone_flag = Flag::try_from(reader.read_i8().unwrap())?;

        Ok(MarshalZone {
            zone_start,
            zone_flag,
        })
    }
}

#[derive(Debug)]
enum Weather {
    Clear,
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

impl TryFrom<u8> for Weather {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Weather::Clear),
            1 => Ok(Weather::LightCloud),
            2 => Ok(Weather::Overcast),
            3 => Ok(Weather::LightRain),
            4 => Ok(Weather::HeavyRain),
            5 => Ok(Weather::Storm),
            _ => Err(UnpackError(format!("Invalid Weather value: {}", value))),
        }
    }
}

#[derive(Debug)]
enum SessionType {
    Unknown,
    Practice1,
    Practice2,
    Practice3,
    PracticeShort,
    Qualifying1,
    Qualifying2,
    Qualifying3,
    QualifyingShort,
    OneShotQualifying,
    Race,
    Race2,
    TimeTrial,
}

impl TryFrom<u8> for SessionType {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SessionType::Unknown),
            1 => Ok(SessionType::Practice1),
            2 => Ok(SessionType::Practice2),
            3 => Ok(SessionType::Practice3),
            4 => Ok(SessionType::PracticeShort),
            5 => Ok(SessionType::Qualifying1),
            6 => Ok(SessionType::Qualifying2),
            7 => Ok(SessionType::Qualifying3),
            8 => Ok(SessionType::QualifyingShort),
            9 => Ok(SessionType::OneShotQualifying),
            10 => Ok(SessionType::Race),
            11 => Ok(SessionType::Race2),
            12 => Ok(SessionType::TimeTrial),
            _ => Err(UnpackError(format!("Invalid SessionType value: {}", value))),
        }
    }
}

#[derive(Debug)]
enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
}

impl TryFrom<u8> for Formula {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Formula::F1Modern),
            1 => Ok(Formula::F1Classic),
            2 => Ok(Formula::F2),
            3 => Ok(Formula::F1Generic),
            _ => Err(UnpackError(format!("Invalid Formula value: {}", value))),
        }
    }
}

#[derive(Debug)]
enum SafetyCar {
    None,
    Full,
    Virtual,
}

impl TryFrom<u8> for SafetyCar {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SafetyCar::None),
            1 => Ok(SafetyCar::Full),
            2 => Ok(SafetyCar::Virtual),
            _ => Err(UnpackError(format!("Invalid SafetyCar value: {}", value))),
        }
    }
}

/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second
///
/// Size: 149 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                 Header
/// weather:                Weather - 0 = clear, 1 = light cloud, 2 = overcast
///                         3 = light rain, 4 = heavy rain, 5 = storm
/// track_temperature:      Track temp. in degrees celsius
/// air_temperature:        Air temp. in degrees celsius
/// total_laps:             Total number of laps in this race
/// track_length:           Track length in metres
/// session_type:           0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
///                         5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
///                         10 = R, 11 = R2, 12 = Time Trial
/// track_id:               -1 for unknown, 0-21 for tracks, see appendix
/// formula:                Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
///                         3 = F1 Generic
/// session_time_left:      Time left in session in seconds
/// session_duration:       Session duration in seconds
/// pit_speed_limit:        Pit speed limit in kilometres per hour
/// game_paused:            Whether the game is paused
/// is_spectating:          Whether the player is spectating
/// spectator_car_index:    Index of the car being spectated
/// sli_pro_native_support: SLI Pro support, 0 = inactive, 1 = active
/// numMarshal_zones:       Number of marshal zones to follow
/// marshal_zones:          List of marshal zones – max 21
/// safety_car_status:      0 = no safety car, 1 = full safety car
///                         2 = virtual safety car
/// network_game:           0 = offline, 1 = online
/// ```
#[derive(Debug)]
struct PacketSessionData {
    header: PacketHeader,
    weather: Weather,
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: SessionType,
    track_id: i8,
    formula: Formula,
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: bool,
    num_marshal_zones: u8,
    marshal_zones: [MarshalZone; 21],
    safety_car_status: SafetyCar,
    network_game: bool,
}

impl PacketSessionData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketSessionData, UnpackError> {
        let weather = Weather::try_from(reader.read_u8().unwrap())?;
        let track_temperature = reader.read_i8().unwrap();
        let air_temperature = reader.read_i8().unwrap();
        let total_laps = reader.read_u8().unwrap();
        let track_length = reader.read_u16::<LittleEndian>().unwrap();
        let session_type = SessionType::try_from(reader.read_u8().unwrap())?;
        let track_id = reader.read_i8().unwrap();
        let formula = Formula::try_from(reader.read_u8().unwrap())?;
        let session_time_left = reader.read_u16::<LittleEndian>().unwrap();
        let session_duration = reader.read_u16::<LittleEndian>().unwrap();
        let pit_speed_limit = reader.read_u8().unwrap();
        let game_paused = reader.read_u8().unwrap();
        let is_spectating = reader.read_u8().unwrap();
        let spectator_car_index = reader.read_u8().unwrap();
        let sli_pro_native_support = reader.read_u8().unwrap() == 1;
        let num_marshal_zones = reader.read_u8().unwrap();

        // Ugly but avoids double initialization
        let marshal_zones = [
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
            MarshalZone::new(&mut reader)?,
        ];

        let safety_car_status = SafetyCar::try_from(reader.read_u8().unwrap())?;
        let network_game = reader.read_u8().unwrap() == 1;

        Ok(PacketSessionData {
            header,
            weather,
            track_temperature,
            air_temperature,
            total_laps,
            track_length,
            session_type,
            track_id,
            formula,
            session_time_left,
            session_duration,
            pit_speed_limit,
            game_paused,
            is_spectating,
            spectator_car_index,
            sli_pro_native_support,
            num_marshal_zones,
            marshal_zones,
            safety_car_status,
            network_game,
        })
    }
}

fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let header_size = mem::size_of::<PacketHeader>();

    if size < header_size {
        return Err(UnpackError(format!(
            "Invalid packet: too small ({} bytes)",
            size
        )));
    }

    let mut cursor = Cursor::new(packet);
    let header = PacketHeader::new(&mut cursor);

    let packet_id: PacketID = PacketID::try_from(header.packet_id)?;

    match packet_id {
        //PacketID::Motion => {}
        PacketID::Session => {
            let session_data = PacketSessionData::new(&mut cursor, header)?;

            return Ok(Packet::Session(session_data));
        }
        //PacketID::LapData => {}
        //PacketID::Event => {}
        //PacketID::Participants => {}
        //PacketID::CarSetups => {}
        //PacketID::CarTelemetry => {}
        //PacketID::CarStatus => {}
        _ => Err(UnpackError(format!(
            "Unpacking not implemented for {:?}",
            packet_id
        ))),
    }
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", socket.local_addr().unwrap());

    let mut buf = [0; 2048]; // All packets fit in 2048 bytes

    for _ in 0..20 {
        match socket.recv(&mut buf) {
            Ok(len) => match parse_packet(len, &buf) {
                Ok(p) => match p {
                    Packet::Session(s) => println!("{:?}", s),
                },
                Err(e) => println!("{:?}", e),
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

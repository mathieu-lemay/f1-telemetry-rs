use serde::Deserialize;
use std::convert::TryFrom;
use std::mem;
use std::net::UdpSocket;

//struct UnpackError(&'static str);
#[derive(Debug)]
struct UnpackError(String);

#[derive(Copy, Clone, Debug, Deserialize)]
enum PacketID {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
}

impl TryFrom<u8> for PacketID {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == PacketID::Motion as u8 => Ok(PacketID::Motion),
            x if x == PacketID::Session as u8 => Ok(PacketID::Session),
            x if x == PacketID::LapData as u8 => Ok(PacketID::LapData),
            x if x == PacketID::Event as u8 => Ok(PacketID::Event),
            x if x == PacketID::Participants as u8 => Ok(PacketID::Participants),
            x if x == PacketID::CarSetups as u8 => Ok(PacketID::CarSetups),
            x if x == PacketID::CarTelemetry as u8 => Ok(PacketID::CarTelemetry),
            x if x == PacketID::CarStatus as u8 => Ok(PacketID::CarStatus),
            _ => Err(UnpackError(format!("Invalid PacketID: {}", value))),
        }
    }
}

enum Packet {
    Session(PacketSessionData),
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
#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(packed)]
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

/// This type is used for the 21-element 'marshalZones' array of the [`PacketSessionData`] type, defined below.
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts
/// zone_flag:  -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
/// ```
/// [`PacketSessionData`]: ./struct.PacketSessionData.html
#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(packed)]
struct MarshalZone {
    zone_start: f32,
    zone_flag: i8,
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
///                         3 = light:rain, 4 = heavy rain, 5 = storm
/// track_temperature:      Track temp. in degrees celsius
/// air_temperature:        Air temp. in degrees celsius
/// total_laps:             Total number of laps in this race
/// track_length:           Track length in metres
/// session_type:           0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
///                         5 = Q1, 6 = Q2, 7 = Q3, 8 = Short:Q, 9 = OSQ
///                         10 = R, 11 = R2, 12 = Time:Trial
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
///                         2 = virtual:safety car
/// network_game:           0 = offline, 1 = online
/// ```
#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(packed)]
struct PacketSessionData {
    header: PacketHeader,
    weather: u8,
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: u8,
    track_id: i8,
    formula: u8,
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: u8,
    num_marshal_zones: u8,
    marshal_zones: [MarshalZone; 21],
    safety_car_status: u8,
    network_game: u8,
}

fn parse_packet(size: usize, packet: &[u8; 2048]) -> Result<Packet, UnpackError> {
    let header_size = mem::size_of::<PacketHeader>();

    if size < header_size {
        return Err(UnpackError(format!(
            "Invalid packet: too small ({} bytes)",
            size
        )));
    }

    let header: PacketHeader = bincode::deserialize(&packet[..header_size]).unwrap();

    let packet_id: PacketID = PacketID::try_from(header.packet_id)?;

    match packet_id {
        //PacketID::Motion => {}
        PacketID::Session => {
            let packet: PacketSessionData = bincode::deserialize(&packet[..]).unwrap();

            return Ok(Packet::Session(packet));
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
                    Packet::Session(s) => println!(
                        "[Session] SessionTimeLeft: {}, SessionDuration: {}",
                        s.session_time_left, s.session_duration
                    ),
                },
                Err(e) => println!("{:?}", e),
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

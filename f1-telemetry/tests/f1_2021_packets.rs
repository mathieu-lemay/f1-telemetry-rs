use serial_test::serial;

use f1_telemetry::packet::generic::Flag;
use f1_telemetry::packet::session::{
    BrakingAssist, DrivingAssists, DynamicRacingLine, DynamicRacingLineType, ForecastAccuracy,
    Formula, GearboxAssist, MarshalZone, PacketSessionData, SafetyCar, SessionType,
    TemperatureChange, Track, Weather, WeatherForecastSample,
};
use f1_telemetry::packet::Packet;

mod utils;

#[test]
#[serial]
fn test_parse_2021_session_packet() {
    let stream = utils::get_stream();

    utils::send_raw_data(&stream, "e507011201012e324e2ac5eb38ad10e997421b06000013ff001d14c813110906000000f000500000ff000f000000000081aeba3d00756a0b3e00f3c5473e0041d8723e00699d8b3e00a5d2a23e00cd64c63e0086dddf3e00b77dfb3e009f55153f00fd7e263f00d8923c3f00fbd35b3f003ce06b3f000000000000000000000000000000000000000000000000000000000000000000060900001d021402020905001d02140202090a001d021402020a00001d021402040a05001d021402040a0a001d0214020400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f3028fbaf3028fbaf3028fbaf000000000003010100000101");

    let p = utils::get_packet(&stream).unwrap().unwrap();

    let actual = match p {
        Packet::Session(s) => s,
        _ => panic!("Invalid packet. Expected Session, got {:?}", &p),
    };

    assert_eq!(actual.header.packet_format, 2021);

    let expected = PacketSessionData {
        header: actual.header.clone(),
        weather: Weather::Clear,
        track_temperature: 29,
        air_temperature: 20,
        total_laps: 200,
        track_length: 4371,
        session_type: SessionType::OneShotQualifying,
        track: Track::Montreal,
        formula: Formula::F1Modern,
        session_time_left: 0,
        session_duration: 240,
        pit_speed_limit: 80,
        game_paused: false,
        is_spectating: false,
        spectator_car_index: 255,
        sli_pro_native_support: false,
        num_marshal_zones: 15,
        marshal_zones: vec![
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.09115315,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.13614829,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.19509105,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.23715307,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.27268532,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.31801334,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.3874878,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.43723696,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.4911935,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.5833377,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.6503752,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.73661566,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.8587033,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.9213903,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
            MarshalZone {
                zone_start: 0.0,
                zone_flag: Flag::None,
            },
        ],
        safety_car_status: SafetyCar::None,
        network_game: false,
        num_weather_forecast_samples: 6,
        weather_forecast_samples: vec![
            WeatherForecastSample {
                session_type: SessionType::OneShotQualifying,
                time_offset: 0,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 2,
            },
            WeatherForecastSample {
                session_type: SessionType::OneShotQualifying,
                time_offset: 5,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 2,
            },
            WeatherForecastSample {
                session_type: SessionType::OneShotQualifying,
                time_offset: 10,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 2,
            },
            WeatherForecastSample {
                session_type: SessionType::Race,
                time_offset: 0,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 4,
            },
            WeatherForecastSample {
                session_type: SessionType::Race,
                time_offset: 5,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 4,
            },
            WeatherForecastSample {
                session_type: SessionType::Race,
                time_offset: 10,
                weather: Weather::Clear,
                track_temperature: 29,
                track_temperature_change: TemperatureChange::NoChange,
                air_temperature: 20,
                air_temperature_change: TemperatureChange::NoChange,
                rain_percentage: 4,
            },
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
            WeatherForecastSample::default(),
        ],
        forecast_accuracy: Some(ForecastAccuracy::Perfect),
        ai_difficulty: Some(31),
        season_identifier: Some(2952472624),
        weekend_identifier: Some(2952472624),
        session_identifier: Some(2952472624),
        pit_stop_window_ideal_lap: Some(0),
        pit_stop_window_latest_lap: Some(0),
        pit_stop_rejoin_position: Some(0),
        driving_assists: Some(DrivingAssists {
            steering_assist: false,
            braking_assist: BrakingAssist::Off,
            gearbox_assist: GearboxAssist::Automatic,
            pit_assist: true,
            pit_relase_assist: true,
            ers_assist: false,
            drs_assist: false,
            dynamic_racing_line: DynamicRacingLine::CornersOnly,
            dynamic_racing_line_type: DynamicRacingLineType::ThreeDimensions,
        }),
    };

    assert_eq!(actual, expected);
}

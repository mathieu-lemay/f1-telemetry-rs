use log::LevelFilter;
use simplelog::{format_description, Config, ConfigBuilder, LevelPadding};

pub fn get_log_config() -> Config {
    ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_target_level(LevelFilter::Error)
        .set_location_level(LevelFilter::Debug)
        .set_thread_level(LevelFilter::Off)
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .set_time_offset_to_local()
        .unwrap_or_else(|e| {
            println!("Error finding local time offset. Using UTC.");
            e
        })
        .build()
}

use std::fs::OpenOptions;
use std::io::Error as IOError;
use std::path::Path;

use log::{LevelFilter, SetLoggerError};
use simplelog::{
    format_description, ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelPadding,
    SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

fn get_log_config() -> Config {
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

#[derive(Default)]
pub struct LogBuilder {
    config: Config,
    loggers: Vec<Box<dyn SharedLogger>>,
}

impl LogBuilder {
    pub fn new() -> Self {
        let config = get_log_config();

        LogBuilder {
            config,
            ..Default::default()
        }
    }

    pub fn with_term_logger(
        mut self,
        log_level: LevelFilter,
        mode: TerminalMode,
        color_choice: ColorChoice,
    ) -> Self {
        self.loggers.push(TermLogger::new(
            log_level,
            self.config.clone(),
            mode,
            color_choice,
        ));
        self
    }

    pub fn with_file_logger<P: AsRef<Path>>(
        mut self,
        log_level: LevelFilter,
        path: P,
    ) -> Result<Self, IOError> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        self.loggers
            .push(WriteLogger::new(log_level, self.config.clone(), file));

        Ok(self)
    }

    pub fn build(self) -> Result<(), SetLoggerError> {
        CombinedLogger::init(self.loggers)
    }
}

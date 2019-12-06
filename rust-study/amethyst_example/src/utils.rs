pub fn init() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Off,
        ..LoggerConfig::default()
    });
}

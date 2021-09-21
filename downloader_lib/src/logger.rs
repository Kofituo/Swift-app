use android_logger::Config;
use log::Level;
use rust_interface_file_generator::gen_attributes_interface_generator::*;

pub struct Logger;

impl Logger {
    ///Set up logging
    #[generate_interface]
    pub fn initialise_logging() {
        #[cfg(target_os = "android")]
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace)
                .with_tag("rust"),
        );
        log_panics::init();
        log::error!("initialised");
    }
}

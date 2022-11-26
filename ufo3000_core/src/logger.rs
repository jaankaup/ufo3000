use simple_logger::SimpleLogger;
use log::LevelFilter;

/// Initialize simple logger.
pub fn initialize_simple_logger(module_levels: &Vec<(String, LevelFilter)>) {

    #[cfg(not(target_arch = "wasm32"))]
    {

        let mut simple_logger = SimpleLogger::new();

        for (s, l) in module_levels.iter() {
            simple_logger = simple_logger.with_module_level(s, *l);
        }

        simple_logger.with_utc_timestamps().init().unwrap();
    }
}

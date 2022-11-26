//use log; 
use simple_logger::SimpleLogger;
use log::LevelFilter;

fn main() {

    SimpleLogger::new()
    .with_level(LevelFilter::Off)
    .with_module_level("dummy_example", LevelFilter::Info)
    .with_utc_timestamps()
    .init()
    .unwrap();

    log::info!("Hekotus from dummy_example.");
}

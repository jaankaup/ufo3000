//use log; 
use ufo3000::logger::initialize_simple_logger;
//use simple_logger::SimpleLogger;
use log::LevelFilter;

fn main() {


    initialize_simple_logger(&vec![("dummy_example".to_string(), LevelFilter::Info)]);

    //SimpleLogger::new()
    //.with_level(LevelFilter::Off)
    //.with_module_level("dummy_example", LevelFilter::Info)
    //.with_utc_timestamps()
    //.init()
    //.unwrap();

    log::info!("Hekotus from dummy_example.");
}

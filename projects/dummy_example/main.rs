use ufo3000::logger::initialize_simple_logger;
use ufo3000::template::*;
use log::LevelFilter;

fn main() {


    initialize_simple_logger(&vec![("dummy_example".to_string(), LevelFilter::Info)]);

    log::info!("Hekotus from dummy_example.");
}

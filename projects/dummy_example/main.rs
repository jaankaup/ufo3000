// use crate::features::*;
mod dummy_features;
use log::LevelFilter;
use ufo3000::logger::initialize_simple_logger;
use ufo3000::input::*;
use ufo3000::template::{
    WGPUFeatures,
    WGPUConfiguration,
    Application,
    BasicLoop,
    Spawner,
};


fn main() {

    initialize_simple_logger(&vec![("dummy_example".to_string(), LevelFilter::Info)]);
    let dummy_features = dummy_features::DummyExampleFeatures {};

    log::info!("Hekotus from dummy_example.");
}

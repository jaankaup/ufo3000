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
    run_loop,
};
use ufo3000::screen::ScreenTexture;

// TODO: drop renderpass if there is nothing to draw.

struct DummyExampleFeatures {
    //screen: ScreenTexture, 
}

// impl Application for FmmApp {
// 
//     /// Initialize application.
//     fn init(configuration: &WGPUConfiguration) -> Self {}
// 
//     /// Render application.
//     fn render(&mut self,
//               device: &wgpu::Device,
//               queue: &mut wgpu::Queue,
//               surface: &wgpu::Surface,
//               sc_desc: &wgpu::SurfaceConfiguration,
//               _spawner: &Spawner) {
// 
//         self.screen.acquire_screen_texture(
//             &device,
//             &sc_desc,
//             &surface
//             );
// 
//     }
// 
// }

fn main() {

    // Initialize logging.
    initialize_simple_logger(&vec![("dummy_example".to_string(), LevelFilter::Info)]);

    // Execute application.
    //run_loop::<DummyExampleApp, BasicLoop, DummyExampleFeatures>(); 

    log::info!("Hekotus from dummy_example.");
}

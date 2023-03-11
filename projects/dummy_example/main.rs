// use crate::features::*;
mod dummy_features;
use log::LevelFilter;
//use winit::dpi::PhysicalSize;
use ufo3000::logger::initialize_simple_logger;
use ufo3000::input::InputCache;
use ufo3000::template::{
    WGPUConfiguration,
    Application,
    BasicLoop,
    Spawner,
    run_loop,
};
use ufo3000::screen::ScreenTexture;
use ufo3000::camera::Camera;
use ufo3000::texture::Texture as ATexture;
// use ufo3000::render_object::*;

// TODO: drop renderpass if there is nothing to draw.

struct DummyExampleApp {
    screen: ScreenTexture, 
    camera: Camera,
    render: bool,
}

impl Application for DummyExampleApp {

    /// Initialize application.
    fn init(configuration: &WGPUConfiguration) -> Self {

        // Create camera.
        let mut camera = Camera::new(configuration.size.width as f32,
                                     configuration.size.height as f32,
                                     (180.0, 130.0, 480.0),
                                     -89.0,
                                     -4.0
        );
        camera.set_rotation_sensitivity(0.4);
        camera.set_movement_sensitivity(0.2);

        Self {
            screen: ScreenTexture::init(&configuration.device, &configuration.sc_desc, true),
            camera,
            render: false,
        }
    }

    /// Render application.
    fn render(&mut self,
              device: &wgpu::Device,
              _queue: &mut wgpu::Queue,
              surface: &wgpu::Surface,
              sc_desc: &wgpu::SurfaceConfiguration,
              _spawner: &Spawner) {

        if self.render {

            // Acquire screen.
            self.screen.acquire_screen_texture(
                device,
                sc_desc,
                surface
                );

            // Create view.
            let _view = self.screen.surface_texture.as_ref().unwrap().texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Prepare rendering.
            self.screen.prepare_for_rendering();

        } // render
    }

    /// Handle user input.
    fn input(&mut self, _queue: &wgpu::Queue, _input: &InputCache) {

    }

    /// Resize window.
    fn resize(&mut self, device: &wgpu::Device, sc_desc: &wgpu::SurfaceConfiguration, _new_size: winit::dpi::PhysicalSize<u32>) {

        // TODO: add this functionality to the Screen.
        self.screen.depth_texture = Some(ATexture::create_depth_texture(device, sc_desc, Some("depth-texture")));
        self.camera.resize(sc_desc.width as f32, sc_desc.height as f32);
    }

    /// Application update.
    fn update(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue, _input: &InputCache, _spawner: &Spawner) {

    }

    /// Exit.
    fn exit(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue, _input: &InputCache, _spawner: &Spawner) {
        log::info!("Exit.");
    }
}

fn main() {

    // Initialize logging.
    initialize_simple_logger(&vec![("dummy_example".to_string(), LevelFilter::Info)]);

    log::info!("Hekotus from dummy_example.");

    // Execute application.
    run_loop::<DummyExampleApp, BasicLoop, dummy_features::DummyExampleFeatures>(); 

}

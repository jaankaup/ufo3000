mod cube_features;
use log::LevelFilter;
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
use ufo3000::render_object::create_render_pass;

#[cfg(target_arch = "wasm32")]
use ufo3000::template::OffscreenCanvasSetup;

// TODO: drop renderpass if there is nothing to draw.

struct CubeApp {
    screen: ScreenTexture, 
    camera: Camera,
    render: bool,
}

impl Application for CubeApp {

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
            render: true,
        }
    }

    /// Render application.
    fn render(&mut self,
              device: &wgpu::Device,
              queue: &mut wgpu::Queue,
              surface: &wgpu::Surface,
              sc_desc: &wgpu::SurfaceConfiguration,
              #[cfg(target_arch = "wasm32")]
              offscreen_canvas_setup: &OffscreenCanvasSetup,
              _spawner: &Spawner) {

        if self.render {

            // Acquire screen.
            self.screen.acquire_screen_texture(
                device,
                sc_desc,
                surface
                );

            // Create view.
            let view = self.screen.surface_texture.as_ref().unwrap().texture.create_view(&wgpu::TextureViewDescriptor::default());

            // If there is nothing to draw, this must be executed.
            let mut cube_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Cube encoder") });
            {
                let _render_pass = create_render_pass(
                    &mut cube_encoder,
                    &view,
                    self.screen.depth_texture.as_ref().unwrap(),
                    true,
                    &Some(wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    })
                    );
            }
            queue.submit(Some(cube_encoder.finish()));

            // Prepare rendering.
            #[cfg(not(target_arch = "wasm32"))]
            self.screen.prepare_for_rendering();

            #[cfg(target_arch = "wasm32")]
            self.screen.prepare_for_rendering(offscreen_canvas_setup);

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
    run_loop::<CubeApp, BasicLoop, cube_features::CubeFeatures>(); 

}

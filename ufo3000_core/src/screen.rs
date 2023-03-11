use std::mem;
use crate::texture::Texture;
#[cfg(target_arch = "wasm32")]
use crate::template::OffscreenCanvasSetup;

/// A struct that owns the current wgpu::SurfaceTexture and the optional depth texture.
/// TODO: getter_functions for attributes
pub struct ScreenTexture {
    pub surface_texture: Option<wgpu::SurfaceTexture>,
    #[allow(dead_code)]
    pub depth_texture: Option<Texture>,
}

impl ScreenTexture {

    /// Create ScreenTexture without current wgpu::SurfaceTexture.
    /// A depth texture is created if the create_depth_texture parameter is true.
    pub fn init(
             device: &wgpu::Device,
             sc_desc: &wgpu::SurfaceConfiguration,
             create_depth_texture: bool) -> Self {

        log::info!("Screen::init.");

        let depth_texture = if create_depth_texture {
                Some(Texture::create_depth_texture(
                    device,
                    sc_desc,
                    Some("depth_texture")
                    )
                )
            } else { None };

        log::info!("Created depth_texture.");

        Self {
            surface_texture: None,
            depth_texture,
        }
    }

    /// Acquire the current screen texture.
    pub fn acquire_screen_texture(
            &mut self,
            device: &wgpu::Device,
            sc_desc: &wgpu::SurfaceConfiguration,
            surface: &wgpu::Surface) {

        let frame = match surface.get_current_texture() {
            Ok(frame) => {frame},
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => {
                surface.configure(device, sc_desc);
                
                surface.get_current_texture().expect("Failed to acquire next texture")
            },
            Err(wgpu::SurfaceError::Timeout) => panic!("Timeout occurred while acquiring the next frame texture."),
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("OutOfMemory occurred while acquiring the next frame texture."),
        };
        self.surface_texture = Some(frame);
    }

    /// This must be called so the texture can be actually rendered to the screen. Call this method
    /// after wgpu::Queue::submit.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn prepare_for_rendering(&mut self) {
        if self.surface_texture.is_none() {
            panic!("ScreenTexture doesn't have a surface_texture. Consider calling the ScreenTexture::acquire_screen_texture before this method.");
        }

        mem::take(&mut self.surface_texture).unwrap().present();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn prepare_for_rendering(&mut self, offscreen_canvas_setup: &OffscreenCanvasSetup) {

        if self.surface_texture.is_none() {
            panic!("ScreenTexture doesn't have a surface_texture. Consider calling the ScreenTexture::acquire_screen_texture before this method.");
        }

        mem::take(&mut self.surface_texture).unwrap().present();

        #[cfg(target_arch = "wasm32")]
        {
            let image_bitmap = offscreen_canvas_setup
                .offscreen_canvas
                .transfer_to_image_bitmap()
                .expect("couldn't transfer offscreen canvas to image bitmap.");
            offscreen_canvas_setup
                .bitmap_renderer
                .transfer_from_image_bitmap(&image_bitmap);

            log::info!("Transferring OffscreenCanvas to ImageBitmapRenderer");
        }
    }
}

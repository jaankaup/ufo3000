use ufo3000::template::{
    WGPUFeatures,
};

/// Features and limits for DummyExample application.
pub struct CubeFeatures {}

impl WGPUFeatures for CubeFeatures {

    fn optional_features() -> wgpu::Features {
        wgpu::Features::TIMESTAMP_QUERY
    }

    fn required_features() -> wgpu::Features {

        if cfg!(not(target_arch = "wasm32")) {
            wgpu::Features::empty()
            // wgpu::Features::PUSH_CONSTANTS |
            // wgpu::Features::WRITE_TIMESTAMP_INSIDE_PASSES
        }
        else {
            wgpu::Features::empty()
        }
    }

    fn required_limits() -> wgpu::Limits {
        let mut limits = wgpu::Limits::default();

        #[cfg(not(target_arch = "wasm32"))]
        {
        limits.max_compute_invocations_per_workgroup = 1024;
        limits.max_compute_workgroup_size_x = 1024;
        limits.max_push_constant_size = 4;
        limits.max_push_constant_size = 4;
        }

        limits.max_storage_buffers_per_shader_stage = 10;

        // limits.max_storage_buffer_binding_size = 396441600; 

        limits
    }
}

// use cgmath::Vector3;
use crate::misc::clamp;
use crate::input::{InputCache, InputState};
use crate::buffer::buffer_from_data;
use cgmath::{prelude::*, Vector3, Vector4, Point3};
use bytemuck::{Pod, Zeroable};

pub use winit::event::VirtualKeyCode as Key;
pub use winit::event::MouseButton as MouseButton;

/// Opengl to wgpu matrix
//#[cfg_attr(rustfmt, surtfmt_skip)]
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
);

/// Struct that represent camera uniform data in shader. The projection matrix and the position of
/// the camera.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CameraUniform {
    view_proj: cgmath::Matrix4<f32>,
    pos: cgmath::Vector4<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

/// Struct that represent ray tracing camera uniform data in shader.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RayCameraUniform {
    pos: [f32; 3],
    aperture_radius: f32,
    view: [f32; 3],
    focal_distance: f32,
    up: [f32; 3],
    padding: u32,
    fov: [f32; 2],
    padding2: [u32; 2],
}

// unsafe impl bytemuck::Zeroable for RayCameraUniform {}
// unsafe impl bytemuck::Pod for RayCameraUniform {}

/// A camera for basic rendering and ray tracing purposes.
pub struct Camera {
    pos: cgmath::Vector3<f32>,
    view: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov: cgmath::Vector2<f32>,
    znear: f32,
    zfar: f32,
    movement_sensitivity: f32,
    rotation_sensitivity: f32,
    pitch: f32,
    yaw: f32,
    aperture_radius: f32, // For ray tracer camera.
    focal_distance: f32, // For ray tracer camera.
    camera_buffer: Option<wgpu::Buffer>, // A buffer to basic camera uniform buffer.
    ray_camera_buffer: Option<wgpu::Buffer>, // A buffer to ray tracing camear uniform buffer.
    restriction_area: [cgmath::Vector3<f32> ; 2],
    restriction_area_enabled: bool,
}

impl Camera {

    pub fn set_movement_sensitivity(&mut self, sensitivity: f32) {

        assert!(sensitivity > 0.0, "Movement sensitivity must be > 0.");
        self.movement_sensitivity = sensitivity;
    }

    pub fn set_rotation_sensitivity(&mut self, sensitivity: f32) {
        assert!(sensitivity > 0.0, "Rotation sensitivity must be > 0.");
        self.rotation_sensitivity = sensitivity;
    }

    pub fn set_focal_distance(&mut self, value: f32, queue: &wgpu::Queue) {
        if value > 0.0 {
            self.focal_distance = value;
        }
        // self.update_camera(&queue);
        self.update_ray_camera(queue);
    }

    pub fn get_focal_distance(&self) -> f32 {
        self.focal_distance
    }

    pub fn get_view(&self) -> [f32 ; 3] {
        [self.view.x, self.view.y, self.view.z]
    }

    pub fn get_position(&self) -> [f32; 3] {
        [self.pos.x, self.pos.y, self.pos.z]
    }

    pub fn move_forward(&mut self, amount: f32, queue: &wgpu::Queue) {
        self.pos += self.view * amount;
        self.update_camera(queue);
        self.update_ray_camera(queue);
    }
    pub fn set_lookat(&mut self, at: [f32; 3], queue: &wgpu::Queue) {
        self.view = Vector3::new(
            at[0] - self.pos.x, // - at[0],
            at[1] - self.pos.y, // - at[1],
            at[2] - self.pos.z, // - at[2],
        ).normalize_to(1.0);
        self.update_camera(queue);
        self.update_ray_camera(queue);
    }

    /// Get a reference to camera uniform buffer. Creates the buffer is it doens't already exist.
    pub fn get_camera_uniform(&mut self, device: &wgpu::Device) -> &wgpu::Buffer {

        // The camera uniform buffer doesn't exist. Create camera buffer.
        if self.camera_buffer.is_none() {

            // Create camera uniform data.
            let camera_uniform = CameraUniform {
                view_proj: self.build_projection_matrix(),
                pos: Vector4::new(self.pos.x, self.pos.y, self.pos.z, 1.0),
            };

            self.camera_buffer = Some(buffer_from_data::<CameraUniform>(
                device,
                &[camera_uniform],
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                None)
            );
        }

        self.camera_buffer.as_ref().unwrap()
    }
    
    // TODO: update uniform?
    pub fn resize(&mut self, aspect_width: f32, aspect_height: f32) {
        self.aspect = aspect_width / aspect_height;
    }

    /// Get a reference to ray tracing camera uniform buffer. Creates the buffer is it doesn't already exist.
    /// TODO: create buffer on init().
    pub fn get_ray_camera_uniform(&mut self, device: &wgpu::Device) -> &wgpu::Buffer {

        // The ray camera uniform buffer doesn't exist. Create ray camera buffer.
        if self.ray_camera_buffer.is_none() {

            // println!("Creating ray camera uniform.");
            // Create ray camera uniform data.
            let ray_camera_uniform = RayCameraUniform {
                pos: [self.pos.x, self.pos.y, self.pos.z],
                aperture_radius: self.aperture_radius,
                view: [self.view.x, self.view.y, self.view.z],
                focal_distance: self.focal_distance,
                up: [self.up.x, self.up.y, self.up.z],
                padding: 0,
                fov: [self.fov.x, self.fov.y],
                padding2: [0, 0],
            };

            self.ray_camera_buffer = Some(buffer_from_data::<RayCameraUniform>(
                device,
                &[ray_camera_uniform],
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                None)
            );
        }

        self.ray_camera_buffer.as_ref().unwrap()
    }

    /// TODO: something better.
    pub fn new(aspect_width: f32, aspect_height: f32, start_position: (f32, f32, f32), yaw: f32, pitch: f32) -> Self {

        assert!(aspect_height > 0.0, "Height must be > 0.");
        assert!(aspect_width > 0.0, "Width must be > 0.");

        let pitch = clamp(
            pitch,
            -89.0,89.0);

        let view = Vector3::new(
            pitch.to_radians().cos() * yaw.to_radians().cos(),
            pitch.to_radians().sin(),
            pitch.to_radians().cos() * yaw.to_radians().sin()
        ).normalize_to(1.0);

        Self {
            pos: start_position.into(),
            view, //Vector3::new(0.0, 0.0, -1.0).normalize(),
            up: cgmath::Vector3::unit_y(),
            aspect: aspect_width / aspect_height,
            fov: (1.485387,0.785387).into(),
            //fov: (45.0,45.0).into(),
            znear: 0.01,
            zfar: 1000.0,
            movement_sensitivity: 0.003,
            rotation_sensitivity: 0.05,
            pitch, // -80.5,
            yaw, // -50.5,
            aperture_radius: 0.01,
            focal_distance: 1.0,
            camera_buffer: None,
            ray_camera_buffer: None,
            restriction_area: [cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0), cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0)],
            restriction_area_enabled: false,
        }
    }

    pub fn set_restriction_area(&mut self, min: [f32; 3], max: [f32; 3]) {
        // TODO: asserts
        self.restriction_area = [cgmath::Vector3::<f32>::new(min[0], min[1], min[2]), cgmath::Vector3::<f32>::new(max[0], max[1], max[2])];
    }

    pub fn enable_restriction_area(&mut self, enable: bool) {
        self.restriction_area_enabled = enable;
    }

    /// Update camera from user input. TODO: create a method for 
    /// Bezier-curvers and B-splines.
    pub fn update_from_input(&mut self, queue: &wgpu::Queue, input_cache: &InputCache) {

        // Get the keyboard state (camera movement).
        let state_forward = input_cache.key_state(&Key::W);
        let state_backward = input_cache.key_state(&Key::S);
        let state_right = input_cache.key_state(&Key::D);
        let state_left = input_cache.key_state(&Key::A);
        let state_up = input_cache.key_state(&Key::E);
        let state_down = input_cache.key_state(&Key::C);
        let left_mouse_button = input_cache.mouse_button_state(&MouseButton::Left);
        let left_shift = input_cache.key_state(&Key::LShift);

        // Get the delta time between previous and current tick.
        let time_delta_nanos = input_cache.get_time_delta();

        // Convert time delta to milli seconds.
        let time_delta_milli_f32 = time_delta_nanos as f32 / 1000000.0;

        // The right vector.
        let right = self.view.cross(self.up);

        let mut movement = cgmath::Vector3::new(0.0, 0.0, 0.0);

        // 1/10 speed if left shift is down.
        let mut movement_factor = 1.0;
        if left_shift.is_some() { movement_factor = 0.1; }

        // Calculate the amount of movement based on user input.
        if state_forward.is_some() { movement += movement_factor * time_delta_milli_f32 * self.view; }
        if state_backward.is_some() { movement -= movement_factor * time_delta_milli_f32 * self.view; }
        if state_right.is_some() { movement += movement_factor * time_delta_milli_f32 * right; }
        if state_left.is_some() { movement -= movement_factor * time_delta_milli_f32 * right; }
        if state_up.is_some() { movement += movement_factor * time_delta_milli_f32 * self.up; }
        if state_down.is_some() { movement -= movement_factor * time_delta_milli_f32 * self.up; }

        let new_pos = self.movement_sensitivity * movement + self.pos;
        // Update the camera position.
        if self.restriction_area_enabled &&
           new_pos.x >= self.restriction_area[0].x &&
           new_pos.y >= self.restriction_area[0].y &&
           new_pos.z >= self.restriction_area[0].z &&
           new_pos.x < self.restriction_area[1].x &&
           new_pos.y < self.restriction_area[1].y &&
           new_pos.z < self.restriction_area[1].z {

           self.pos = new_pos;
        }
        else if !self.restriction_area_enabled {
           self.pos = new_pos;
        }

        // Rotation.
          
        let md = input_cache.get_mouse_delta();

        // If left mouse is down update pitch, yaw and view.
        if let Some(InputState::Down(_,_)) = left_mouse_button {

            self.pitch = clamp(
                self.pitch + (self.rotation_sensitivity * (md.y * (-1.0)) as f32),
                -89.0,89.0);
            self.yaw += self.rotation_sensitivity * md.x as f32 ;

            self.view = Vector3::new(
                self.pitch.to_radians().cos() * self.yaw.to_radians().cos(),
                self.pitch.to_radians().sin(),
                self.pitch.to_radians().cos() * self.yaw.to_radians().sin()
            ).normalize_to(1.0);
        }

        // Update the camera uniform and the camera uniform buffer.
        // TODO: refactor.
        self.update_camera(queue);
        self.update_ray_camera(queue);
    }

    fn update_camera(&self, queue: &wgpu::Queue) {
        if self.camera_buffer.is_some() {

            // Create camera uniform data. TODO: refactor.
            let camera_uniform = CameraUniform {
                view_proj: self.build_projection_matrix(),
                pos: Vector4::new(self.pos.x, self.pos.y, self.pos.z, 1.0),
            };
            queue.write_buffer(
                self.camera_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[camera_uniform]));
        }
    }

    fn update_ray_camera(&self, queue: &wgpu::Queue) {
        if self.ray_camera_buffer.is_some() {

            // Create ray camera uniform data.
            let ray_camera_uniform = RayCameraUniform {
                pos: [self.pos.x, self.pos.y, self.pos.z],
                aperture_radius: self.aperture_radius,
                view: [self.view.x, self.view.y, self.view.z],
                focal_distance: self.focal_distance,
                up: [self.up.x, self.up.y, self.up.z],
                padding: 0,
                fov: [self.fov.x, self.fov.y],
                padding2: [0, 0],
            };

            queue.write_buffer(
                self.ray_camera_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[ray_camera_uniform]));
        }
    }

    /// Creates a pv matrix for wgpu.
    pub fn build_projection_matrix(&self) -> cgmath::Matrix4<f32> {

        let view = self.build_view_matrix();
        let proj = cgmath::perspective(cgmath::Rad(std::f32::consts::PI/2.0), self.aspect, self.znear, self.zfar);

        // Convert "opengl" matrix to wgpu matris.
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    /// Build view projection matrix.
    pub fn build_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let pos3 = Point3::new(self.pos.x, self.pos.y,self.pos.z);
        let view3 = Point3::new(self.view.x + pos3.x, self.view.y + pos3.y, self.view.z + pos3.z);
        
        cgmath::Matrix4::look_at_rh(pos3, view3, self.up)
    }

}


use bytemuck::{Pod, Zeroable};

/// A trait for types that can be copied from wgpu::buffer buffer to
/// a std::Vec. // TODO: check if there is already an implementation for this.
pub trait Convert2Vec where Self: std::marker::Sized {
    fn convert(data: &[u8]) -> Vec<Self>;  
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct OutputVertex {
    pub pos: [f32; 3],
    pub color_point_size: u32,
}

#[macro_export]
macro_rules! impl_convert {
  ($to_type:ty) => {
    impl Convert2Vec for $to_type {
      fn convert(data: &[u8]) -> Vec<Self> {
            let result = data
                .chunks_exact(std::mem::size_of::<Self>())
                .map(|b| *bytemuck::try_from_bytes::<Self>(b).unwrap())
                .collect();
            result
      }
    }
  }
}

impl_convert!{OutputVertex}
impl_convert!{u32}
impl_convert!{f32}

/// Take wgpu::VertexFormats as input and return (stride, Vec<wgpu::VertexBufferDescriptor>)
pub fn create_vb_descriptor(formats: &Vec<wgpu::VertexFormat>) -> (u64, Vec<wgpu::VertexAttribute>) { 

    let mut attribute_descriptors: Vec<wgpu::VertexAttribute> = Vec::new();
    let mut stride: u64 = 0;
    for (i, format) in formats.iter().enumerate() {
        let size = match format {
                wgpu::VertexFormat::Uint8x2 => 2 * std::mem::size_of::<u8>() as u64, 
                wgpu::VertexFormat::Uint8x4 => 4 * std::mem::size_of::<u8>() as u64,
                wgpu::VertexFormat::Sint8x2 => 2 * std::mem::size_of::<i8>() as u64,
                wgpu::VertexFormat::Sint8x4 => 4 * std::mem::size_of::<i8>() as u64,
                wgpu::VertexFormat::Unorm8x2 => 2 * std::mem::size_of::<u8>() as u64,
                wgpu::VertexFormat::Unorm8x4 => 4 * std::mem::size_of::<u8>() as u64,
                wgpu::VertexFormat::Snorm8x2 => 2 * std::mem::size_of::<u8>() as u64,
                wgpu::VertexFormat::Snorm8x4 => 4 * std::mem::size_of::<u8>() as u64,
                wgpu::VertexFormat::Uint16x2 => 2 * std::mem::size_of::<u16>() as u64,
                wgpu::VertexFormat::Uint16x4 => 4 * std::mem::size_of::<u16>() as u64,
                wgpu::VertexFormat::Sint16x2 => 2 * std::mem::size_of::<i16>() as u64,
                wgpu::VertexFormat::Sint16x4 => 4 * std::mem::size_of::<i16>() as u64,
                wgpu::VertexFormat::Unorm16x2 => 2 * std::mem::size_of::<u16>() as u64,
                wgpu::VertexFormat::Unorm16x4 => 4 * std::mem::size_of::<u16>() as u64,
                wgpu::VertexFormat::Snorm16x2 => 2 * std::mem::size_of::<i16>() as u64,
                wgpu::VertexFormat::Snorm16x4 => 4 * std::mem::size_of::<i16>() as u64,
                wgpu::VertexFormat::Float16x2 => unimplemented!(),
                wgpu::VertexFormat::Float16x4 => unimplemented!(),
                wgpu::VertexFormat::Float32 => std::mem::size_of::<f32>() as u64,
                wgpu::VertexFormat::Float32x2 => 2 * std::mem::size_of::<f32>() as u64,
                wgpu::VertexFormat::Float32x3 => 3 * std::mem::size_of::<f32>() as u64,
                wgpu::VertexFormat::Float32x4 => 4 * std::mem::size_of::<f32>() as u64,
                wgpu::VertexFormat::Uint32 => std::mem::size_of::<u32>() as u64,
                wgpu::VertexFormat::Uint32x2 => 2 * std::mem::size_of::<u32>() as u64,
                wgpu::VertexFormat::Uint32x3 => 3 * std::mem::size_of::<u32>() as u64,
                wgpu::VertexFormat::Uint32x4 => 4 * std::mem::size_of::<u32>() as u64,
                wgpu::VertexFormat::Sint32 => std::mem::size_of::<i32>() as u64,
                wgpu::VertexFormat::Sint32x2 => 2 * std::mem::size_of::<i32>() as u64,
                wgpu::VertexFormat::Sint32x3 => 3 * std::mem::size_of::<i32>() as u64,
                wgpu::VertexFormat::Sint32x4 => 4 * std::mem::size_of::<i32>() as u64,
                wgpu::VertexFormat::Float64
                | wgpu::VertexFormat::Float64x2
                | wgpu::VertexFormat::Float64x3
                | wgpu::VertexFormat::Float64x4
                => panic!("VERTEX_ATTRIBUTE_64BIT must be enabled to use Double formats")
        };
        attribute_descriptors.push(
            wgpu::VertexAttribute {
                format: *format,
                offset: stride,
                shader_location: i as u32, 
            }
        );
        stride += size;
    }

    (stride, attribute_descriptors)
}

/// Clamp function.
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    let result  = if val >= max { max } else { val };
    
    if result <= min { min } else { val }
}

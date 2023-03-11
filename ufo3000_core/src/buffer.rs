use crate::misc::Convert2Vec;
use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// Create wgpu::buffer from data.
pub fn buffer_from_data<T: Pod>(
    device: &wgpu::Device,
    t: &[T],
    usage: wgpu::BufferUsages,
    label: wgpu::Label)
    -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(t),
                usage,
            }
        )
}

//pub fn to_vec_explicit<T: Convert2Vec + std::clone::Clone + bytemuck::Pod + std::marker::Send>(
//        device: &wgpu::Device,
//        buffer: &wgpu::Buffer,
//    ) -> Vec<T> {
//
//    let res: Vec<T>;
//
//    let buffer_slice = buffer.slice(..);
//    buffer_slice.map_async(wgpu::MapMode::Read, move |_| ());
//    device.poll(wgpu::Maintain::Wait);
//
//    let data = buffer_slice.get_mapped_range().to_vec();
//    res = Convert2Vec::convert(&data);
//    drop(data);
//    buffer.unmap();
//
//    res
//}

/// Copy the content of the buffer into a vector.
pub fn to_vec<T: Convert2Vec + std::clone::Clone + bytemuck::Pod + std::marker::Send>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    _src_offset: wgpu::BufferAddress,
    copy_size: wgpu::BufferAddress,
    // _spawner: &Spawner,
    ) -> Vec<T> {

    // TODO: Recycle staging buffers.
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: copy_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(buffer, 0, &staging_buffer, 0, copy_size);
    queue.submit(Some(encoder.finish()));

    

    let buffer_slice = staging_buffer.slice(..);
    //++ let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    //++ let _ = buffer_slice.map_async(wgpu::MapMode::Read, true);
    // let _ = buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    buffer_slice.map_async(wgpu::MapMode::Read, move |_| ());
    device.poll(wgpu::Maintain::Wait);

    // Wasm version crashes: DOMException.getMappedRange: Buffer not mapped.
    let data = buffer_slice.get_mapped_range().to_vec();
    let res: Vec<T> = Convert2Vec::convert(&data);
    drop(data);
    staging_buffer.unmap();

    res
}

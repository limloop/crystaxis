use bevy::render::renderer::RenderDevice;
use bevy::render::render_resource::*;

/// GPU-буфер для хранения матриц частиц (инстансинг).
pub struct ParticleInstanceBuffer {
    pub buffer: Buffer,
}

impl ParticleInstanceBuffer {
    pub fn new(device: &RenderDevice, max_particles: usize) -> Self {
        let size = (max_particles * std::mem::size_of::<[[f32; 4]; 4]>()) as u64;
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Particle Instance Buffer"),
            size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { buffer }
    }

    pub fn update(&self, queue: &RenderQueue, transforms: &[Mat4]) {
        let data = bytemuck::cast_slice(transforms);
        queue.write_buffer(&self.buffer, 0, data);
    }
}
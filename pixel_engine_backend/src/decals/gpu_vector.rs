use wgpu::util::DeviceExt;

pub struct GpuVector<T: bytemuck::Pod> {
    buffer: wgpu::Buffer,
    usage: wgpu::BufferUsages,
    capacity: usize,
    len: usize,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T: bytemuck::Pod + std::fmt::Debug + std::cmp::PartialEq> GpuVector<T> {
    #[allow(unused)]
    /// Create a buffer with capacity 0
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gpu_vector"),
                size: 0,
                usage,
                mapped_at_creation: false,
            }),
            usage,
            len: 0,
            capacity: 0,
            marker: std::marker::PhantomData,
        }
    }

    pub fn with_capacity(
        capacity: usize,
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        Self {
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gpu_vector"),
                size: (capacity * std::mem::size_of::<T>()) as u64,
                usage,
                mapped_at_creation: false,
            }),
            capacity,
            len: 0,
            usage,
            marker: std::marker::PhantomData,
        }
    }

    pub fn sync(
        &mut self,
        device: &wgpu::Device,
        data: &[T],
    ) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("gpu_vector_encoder"),
        });
        if self.capacity < data.len() {
            self.len = data.len();
            self.capacity = (self.capacity * 3) / 2 + 1;
            if self.capacity < data.len() {
                // We tried to grow, but it wasn't enough, so just use the size of the input
                // slice
                self.capacity = data.len()
            }
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gpu_vector"),
                size: (self.capacity * std::mem::size_of::<T>()) as u64,
                usage: self.usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        let raw_data: &[u8] = bytemuck::cast_slice(&data);
        encoder.copy_buffer_to_buffer(
            &device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gpu_vector_temp_buffer"),
                contents: raw_data,
                usage: wgpu::BufferUsages::COPY_SRC,
            }),
            0,
            &self.buffer,
            0,
            raw_data.len() as u64,
        );
        self.len = data.len();

        encoder.finish()
    }

    pub fn iter_offsets(&self) -> impl Iterator<Item = std::ops::Range<wgpu::BufferAddress>> {
        (0..self.len)
            .map(|n| (n, n + 1))
            .map(|(start, end)| {
                (
                    start * std::mem::size_of::<T>(),
                    end * std::mem::size_of::<T>(),
                )
            })
            .map(|(start, end)| (start as u64, end as u64))
            .map(|(start, end)| start..end)
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

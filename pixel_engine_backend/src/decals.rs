use crate::Vertex;
use wgpu::util::DeviceExt;
pub type DecalTextureID = usize;

mod gpu_vector;

#[derive(Debug)]
pub struct DecalInstances {
    pub id: DecalTextureID,
    pub pos: [(f32, f32); 4],
    pub uv: [(f32, f32); 4],
    pub w: [f32; 4],
    pub tint: [f32; 4],
}

#[derive(Debug)]
pub struct Decal {
    id: DecalTextureID,
    pub size: (u32, u32),
    pub uv_scale: (f32, f32),
}

pub struct DecalContextManager {
    id_generator: DecalIDGenerator,
    decal_textures:
        std::collections::HashMap<DecalTextureID, (crate::texture::Texture, wgpu::BindGroup)>,
    pub decal_instances: Vec<DecalInstances>,
    vertex_vector: gpu_vector::GpuVector<[Vertex; 4]>,
    cpu_vertex_vector: Vec<[Vertex; 4]>,
    buffer_index: wgpu::Buffer,
}

impl DecalContextManager {
    #[must_use]
    pub fn new(device: &wgpu::Device) -> (Self, wgpu::CommandBuffer) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("decal"),
        });
        let vertex_vector =
            gpu_vector::GpuVector::with_capacity(128, device, wgpu::BufferUsages::VERTEX);
        let buffer_index = {
            let b = device.create_buffer(&wgpu::BufferDescriptor {
                mapped_at_creation: false,
                label: None,
                size: std::mem::size_of::<[u16; 6]>() as u64,
                usage: wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::INDEX
                    | wgpu::BufferUsages::COPY_SRC,
            });
            encoder.copy_buffer_to_buffer(
                &{
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(crate::INDICES),
                        usage: wgpu::BufferUsages::INDEX
                            | wgpu::BufferUsages::COPY_DST
                            | wgpu::BufferUsages::COPY_SRC,
                    })
                },
                0,
                &b,
                0,
                std::mem::size_of::<[u16; 6]>() as u64,
            );
            b
        };
        (
            Self {
                id_generator: DecalIDGenerator(0),
                buffer_index,
                vertex_vector,
                decal_textures: std::collections::HashMap::with_capacity(64),
                decal_instances: Vec::with_capacity(128),
                cpu_vertex_vector: Vec::with_capacity(128),
            },
            encoder.finish(),
        )
    }
    pub fn add_instance(&mut self, decal: DecalInstances) {
        self.decal_instances.push(decal);
    }
}

#[derive(Debug, Clone)]
struct DecalIDGenerator(DecalTextureID);
impl DecalIDGenerator {
    fn get(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

impl Decal {
    pub fn create(ctx: &mut crate::Context, sprite: (&[u8], (u32, u32))) -> Self {
        let id = ctx.dcm.id_generator.get();
        let tex = crate::texture::Texture::from_bytes(&ctx.device, &ctx.queue, sprite);
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &ctx.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tex.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&tex.sampler),
                },
            ],
            label: Some("decal_bindgroup"),
        });
        ctx.dcm.decal_textures.insert(id, (tex, bind_group));

        Self {
            id,
            size: sprite.1,
            uv_scale: (1.0 / (sprite.1).0 as f32, 1.0 / (sprite.1).1 as f32),
        }
    }

    pub fn destroy(self, ctx: &mut crate::Context) {
        ctx.dcm.decal_textures.remove(&self.id);
    }

    #[must_use]
    pub fn id(&self) -> DecalTextureID {
        self.id
    }
}

pub trait DrawDecals<'a, 'b>
where
    'b: 'a,
{
    fn draw_decals(
        &mut self,
        dcm: &'b mut DecalContextManager,
        device: &'b mut wgpu::Device,
        queue: &'b mut wgpu::Queue,
    );
}

impl<'a, 'b> DrawDecals<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_decals(
        &mut self,
        dcm: &'b mut DecalContextManager,
        device: &'b mut wgpu::Device,
        queue: &'b mut wgpu::Queue,
    ) {
        dcm.cpu_vertex_vector.clear();
        for decal_instance in dcm.decal_instances.iter() {
            dcm.cpu_vertex_vector.push([
                Vertex {
                    position: [decal_instance.pos[0].0, decal_instance.pos[0].1, 0.0],
                    tex_coords: [
                        decal_instance.uv[0].0,
                        decal_instance.uv[0].1,
                        decal_instance.w[0],
                    ],
                    tint: decal_instance.tint,
                },
                Vertex {
                    position: [decal_instance.pos[1].0, decal_instance.pos[1].1, 0.0],
                    tex_coords: [
                        decal_instance.uv[1].0,
                        decal_instance.uv[1].1,
                        decal_instance.w[1],
                    ],
                    tint: decal_instance.tint,
                },
                Vertex {
                    position: [decal_instance.pos[2].0, decal_instance.pos[2].1, 0.0],
                    tex_coords: [
                        decal_instance.uv[2].0,
                        decal_instance.uv[2].1,
                        decal_instance.w[2],
                    ],
                    tint: decal_instance.tint,
                },
                Vertex {
                    position: [decal_instance.pos[3].0, decal_instance.pos[3].1, 0.0],
                    tex_coords: [
                        decal_instance.uv[3].0,
                        decal_instance.uv[3].1,
                        decal_instance.w[3],
                    ],
                    tint: decal_instance.tint,
                },
            ]);
        }

        let command = dcm
            .vertex_vector
            .sync(device, dcm.cpu_vertex_vector.as_slice());
        let buffer = dcm.vertex_vector.buffer();
        queue.submit(std::iter::once(command));

        for (range, instance) in dcm
            .vertex_vector
            .iter_offsets()
            .zip(dcm.decal_instances.iter())
        {
            let texture = {
                let t = dcm.decal_textures.get(&instance.id);
                if t.is_none() {
                    dbg!("no texture");
                    continue;
                }
                t.unwrap()
            };

            // Update buffers

            self.set_bind_group(0, &texture.1, &[]);
            self.set_index_buffer(dcm.buffer_index.slice(..), wgpu::IndexFormat::Uint16);
            self.set_vertex_buffer(0, buffer.slice(range));
            self.draw_indexed(0..(crate::INDICES.len() as u32), 0, 0..1);
        }
        //std::thread::sleep(std::time::Duration::from_millis(100));
        //dcm.decal_instances.clear()
    }
}

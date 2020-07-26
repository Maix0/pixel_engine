use crate::Vertex;
pub type DecalTextureID = usize;

#[derive(Debug)]
pub struct DecalInstances {
    pub id: DecalTextureID,
    pub pos: [(f32, f32); 4],
    pub uv: [(f32, f32); 4],
    pub w: [f32; 4],
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
    buffer_vertex: wgpu::Buffer,
    buffer_index: wgpu::Buffer,
}

impl DecalContextManager {
    pub fn new(device: &wgpu::Device) -> (Self, wgpu::CommandBuffer) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let buffer_vertex = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: crate::VERTEX_BUFFER_SIZE,
            usage: wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::VERTEX
                | wgpu::BufferUsage::COPY_SRC,
        });
        let buffer_index = {
            let b = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<[u16; 6]>() as u64,
                usage: wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::INDEX
                    | wgpu::BufferUsage::COPY_SRC,
            });
            encoder.copy_buffer_to_buffer(
                &device.create_buffer_with_data(
                    bytemuck::cast_slice(crate::INDICES),
                    wgpu::BufferUsage::INDEX
                        | wgpu::BufferUsage::COPY_DST
                        | wgpu::BufferUsage::COPY_SRC,
                ),
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
                buffer_vertex,
                decal_textures: std::collections::HashMap::with_capacity(10),
                decal_instances: Vec::with_capacity(100),
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
    pub fn create(
        ctx: &mut crate::Context,
        sprite: (&[u8], (u32, u32)),
    ) -> (Self, wgpu::CommandBuffer) {
        let id = ctx.dcm.id_generator.get();
        let (tex, cmd) = crate::texture::Texture::from_bytes(&ctx.device, sprite);
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &ctx.bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tex.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&tex.sampler),
                },
            ],
            label: None,
        });
        ctx.dcm.decal_textures.insert(id, (tex, bind_group));
        (
            Self {
                id,
                size: sprite.1,
                uv_scale: (1.0 / (sprite.1).0 as f32, 1.0 / (sprite.1).1 as f32),
            },
            cmd,
        )
    }

    pub fn destroy(self, ctx: &mut crate::Context) {
        ctx.dcm.decal_textures.remove(&self.id);
    }

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
        for decal_instance in &dcm.decal_instances {
            let texture = {
                let t = dcm.decal_textures.get(&decal_instance.id);
                if t.is_none() {
                    return;
                }
                t.unwrap()
            };

            // Update buffers
            {
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                encoder.copy_buffer_to_buffer(
                    &device.create_buffer_with_data(
                        bytemuck::cast_slice(&[
                            Vertex {
                                position: [decal_instance.pos[0].0, decal_instance.pos[0].1, 0.0],
                                tex_coords: [
                                    decal_instance.uv[0].0,
                                    decal_instance.uv[0].1,
                                    decal_instance.w[0],
                                ],
                            },
                            Vertex {
                                position: [decal_instance.pos[1].0, decal_instance.pos[1].1, 0.0],
                                tex_coords: [
                                    decal_instance.uv[1].0,
                                    decal_instance.uv[1].1,
                                    decal_instance.w[1],
                                ],
                            },
                            Vertex {
                                position: [decal_instance.pos[2].0, decal_instance.pos[2].1, 0.0],
                                tex_coords: [
                                    decal_instance.uv[2].0,
                                    decal_instance.uv[2].1,
                                    decal_instance.w[2],
                                ],
                            },
                            Vertex {
                                position: [decal_instance.pos[3].0, decal_instance.pos[3].1, 0.0],
                                tex_coords: [
                                    decal_instance.uv[3].0,
                                    decal_instance.uv[3].1,
                                    decal_instance.w[3],
                                ],
                            },
                        ]),
                        wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
                    ),
                    0,
                    &dcm.buffer_vertex,
                    0,
                    crate::VERTEX_BUFFER_SIZE,
                );
                queue.submit(&[encoder.finish()]);
            }

            // Render things
            self.set_bind_group(0, &texture.1, &[]);
            self.set_index_buffer(&dcm.buffer_index, 0, 0);
            self.set_vertex_buffer(0, &dcm.buffer_vertex, 0, 0);
            self.draw_indexed(0..(crate::INDICES.len() as u32), 0, 0..1);
        }
        dcm.decal_instances.clear();
    }
}

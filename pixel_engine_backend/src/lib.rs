pub use winit;
use winit::window::Window;
pub mod decals;
mod texture;

pub trait VertexTrait {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    position: [f32; 3],
    // UV + q for warped Decal
    tex_coords: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl VertexTrait for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

pub const VERTEX_BUFFER_SIZE: u64 = std::mem::size_of::<[Vertex; 4]>() as u64;

const CORNER: f32 = 1f32;
#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-CORNER, CORNER, 0.0], tex_coords: [0.0, 0.0, 1.0] }, // A
    Vertex { position: [-CORNER,-CORNER, 0.0], tex_coords: [0.0, 1.0, 1.0] }, // B
    Vertex { position: [ CORNER,-CORNER, 0.0], tex_coords: [1.0, 1.0, 1.0] }, // C
    Vertex { position: [ CORNER, CORNER, 0.0], tex_coords: [1.0, 0.0, 1.0] }, // D
];

#[rustfmt::skip]
pub(crate) const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3,
];

#[allow(dead_code)]
pub struct Context {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    main_texture: texture::Texture,
    main_bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    dcm: decals::DecalContextManager,
}

impl Context {
    pub async fn new(window: &Window, px_size: (u32, u32, u32)) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY, // Vulkan + Metal + DX12 + Browser WebGPU
        )
        .await
        .expect("Error when requesting Adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let vs_raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/shader.vert.spv"
        ));
        let fs_raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/shader.frag.spv"
        ));
        let vs_data = wgpu::read_spirv(std::io::Cursor::new(&vs_raw[..]))
            .expect("Error when reading Vertex Shader");
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(&fs_raw[..]))
            .expect("Error when reading Fragment Shader");
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let (main_texture, cmd_buffer) = texture::Texture::from_bytes(
            &device,
            (
                &vec![0, 0, 0, 255].repeat((px_size.0 * px_size.1) as usize),
                (px_size.0, px_size.1),
            ),
        );
        queue.submit(&[cmd_buffer]);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let main_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&main_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&main_texture.sampler),
                },
            ],
            label: Some("main_bind_group"),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&texture_bind_group_layout],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let vertex_buffer = {
            let b = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: VERTEX_BUFFER_SIZE,
                usage: wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::VERTEX
                    | wgpu::BufferUsage::COPY_SRC,
            });
            encoder.copy_buffer_to_buffer(
                &device.create_buffer_with_data(
                    bytemuck::cast_slice(VERTICES),
                    wgpu::BufferUsage::VERTEX
                        | wgpu::BufferUsage::COPY_DST
                        | wgpu::BufferUsage::COPY_SRC,
                ),
                0,
                &b,
                0,
                VERTEX_BUFFER_SIZE,
            );
            b
        };
        let index_buffer = {
            let b = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<[u16; 6]>() as u64,
                usage: wgpu::BufferUsage::COPY_DST
                    | wgpu::BufferUsage::INDEX
                    | wgpu::BufferUsage::COPY_SRC,
            });
            encoder.copy_buffer_to_buffer(
                &device.create_buffer_with_data(
                    bytemuck::cast_slice(INDICES),
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
        queue.submit(&[encoder.finish()]);
        let num_indices = INDICES.len() as u32;
        let (dcm, cmd) = decals::DecalContextManager::new(&device);
        queue.submit(&[cmd]);
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            main_bind_group,
            main_texture,
            bind_group_layout: texture_bind_group_layout,
            dcm,
        }
    }
    pub fn render(&mut self, data: &[u8]) {
        self.queue
            .submit(&[self.main_texture.update(&self.device, data)]);
        if let Ok(frame) = self.swap_chain.get_next_texture() {
            //.expect("Timeout getting texture");

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                use decals::DrawDecals;

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.main_bind_group, &[]);
                render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
                render_pass.set_index_buffer(&self.index_buffer, 0, 0);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
                render_pass.draw_decals(&mut self.dcm, &mut self.device, &mut self.queue);
            }
            self.queue.submit(&[encoder.finish()]);
        } else {
            println!("Frame timeout !");
        }
    }

    pub fn create_decal(&mut self, spr: (&[u8], (u32, u32))) -> decals::Decal {
        let (d, cmd) = decals::Decal::create(self, spr);
        self.queue.submit(&[cmd]);
        d
    }
    pub fn draw_decal_instance(&mut self, decal_instance: decals::DecalInstances) {
        self.dcm.add_instance(decal_instance);
    }
}

/*
event_loop.run(move |event, _, control_flow| {
    let screen_buffer_slice = state.get_screen_slice();
   match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        },
        Event::RedrawRequested(_) => {
            state.render();
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    }
});
*/

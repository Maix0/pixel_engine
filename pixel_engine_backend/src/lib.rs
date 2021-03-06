use wgpu::util::DeviceExt;
pub use winit;
use winit::window::Window;
pub mod decals;
mod texture;

pub trait VertexTrait {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    position: [f32; 3],
    // UV + q for warped Decal
    tex_coords: [f32; 3],

    tint: [f32; 4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl VertexTrait for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub const VERTEX_BUFFER_SIZE: u64 = std::mem::size_of::<[Vertex; 4]>() as u64;

const CORNER: f32 = 1f32;
#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-CORNER, CORNER, 0.0], tex_coords: [0.0, 0.0, 1.0], tint: [1.0, 1.0, 1.0, 1.0] }, // A
    Vertex { position: [-CORNER,-CORNER, 0.0], tex_coords: [0.0, 1.0, 1.0], tint: [1.0, 1.0, 1.0, 1.0] }, // B
    Vertex { position: [ CORNER,-CORNER, 0.0], tex_coords: [1.0, 1.0, 1.0], tint: [1.0, 1.0, 1.0, 1.0] }, // C
    Vertex { position: [ CORNER, CORNER, 0.0], tex_coords: [1.0, 0.0, 1.0], tint: [1.0, 1.0, 1.0, 1.0] }, // D
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
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Error when requesting Adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                    label: Some("device_request"),
                },
                None,
            )
            .await
            .expect("Error when getting device and queue");

        device.on_uncaptured_error(|error| panic!("error: {}", error));

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            #[cfg(target_arch = "wasm32")]
            format: wgpu::TextureFormat::Rgba8Unorm,
            #[cfg(not(target_arch = "wasm32"))]
            format: wgpu::TextureFormat::Bgra8Unorm,
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
        let vs_data = wgpu::util::make_spirv(&vs_raw[..]);
        let fs_data = wgpu::util::make_spirv(&fs_raw[..]);

        let vs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("vs_module"),
            source: vs_data,
            flags: wgpu::ShaderFlags::empty(),
        });
        let fs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("fs_module"),
            source: fs_data,
            flags: wgpu::ShaderFlags::empty(),
        });

        let main_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            (
                &vec![0, 0, 0, 255].repeat((px_size.0 * px_size.1) as usize),
                (px_size.0, px_size.1),
            ),
        );

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: true,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        let main_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&main_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&main_texture.sampler),
                },
            ],
            label: Some("main_bind_group"),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    write_mask: wgpu::ColorWrite::ALL,
                    blend: Some(wgpu::BlendState::REPLACE),
                }],
            }),
            depth_stencil: None,

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,                        //
                front_face: wgpu::FrontFace::Ccw,                // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            // color_states: &[wgpu::ColorStateDescriptor {
            //     format: sc_desc.format,
            //     color_blend: wgpu::BlendDescriptor::REPLACE,
            //     alpha_blend: wgpu::BlendDescriptor::REPLACE,
            //     write_mask: wgpu::ColorWrite::ALL,
            // }],
            // vertex_state: wgpu::VertexStateDescriptor {
            //     index_format: wgpu::IndexFormat::Uint16,
            //     vertex_buffers: &[Vertex::desc()],
            // },
            // sample_count: 1,
            // sample_mask: !0,
            // alpha_to_coverage_enabled: false,
        });
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::VERTEX
                | wgpu::BufferUsage::COPY_SRC,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::INDEX
                | wgpu::BufferUsage::COPY_SRC,
        });
        queue.submit(std::iter::once(encoder.finish()));
        let num_indices = INDICES.len() as u32;
        let (dcm, cmd) = decals::DecalContextManager::new(&device);
        queue.submit(std::iter::once(cmd));
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
        self.main_texture.update(&self.queue, data);
        if let Ok(frame) = self.swap_chain.get_current_frame() {
            //.expect("Timeout getting texture");

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                    label: Some("Render Pass"),
                });
                use decals::DrawDecals;

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.main_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
                render_pass.draw_decals(&mut self.dcm, &mut self.device, &mut self.queue);
            }
            self.queue.submit(std::iter::once(encoder.finish()));
        } else {
            println!("Frame timeout !");
        }
    }

    pub fn create_decal(&mut self, spr: (&[u8], (u32, u32))) -> decals::Decal {
        decals::Decal::create(self, spr)
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

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
pub use winit;
use winit::window::Window;
pub mod decals;
mod texture;

#[macro_use]
mod macros {
    #[repr(C)] // guarantee 'bytes' comes after '_align'
    pub struct AlignedAs<Align, Bytes: ?Sized> {
        pub _align: [Align; 0],
        pub bytes: Bytes,
    }

    macro_rules! include_bytes_align_as {
        ($align_ty:ty; $($path:tt)*) => {
            {  // const block expression to encapsulate the static
                use $crate::macros::AlignedAs;

                // this assignment is made possible by CoerceUnsized
                static ALIGNED: &AlignedAs::<$align_ty, [u8]> = &AlignedAs {
                    _align: [],
                    bytes: *include_bytes!($($path)*),
                };

                &ALIGNED.bytes
            }
        };
    }
}

pub trait VertexTrait {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 3],
    // UV + q for warped Decal
    tex_coords: [f32; 3],

    tint: [f32; 4],
}

impl VertexTrait for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
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
    config: wgpu::SurfaceConfiguration,

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
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Error when requesting Adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    #[cfg(not(target_arch = "wasm32"))]
                    limits: Default::default(),
                    #[cfg(target_arch = "wasm32")]
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: Some("device_request"),
                },
                None,
            )
            .await
            .expect("Error when getting device and queue");

        device.on_uncaptured_error(|error| panic!("[WGPU Error] {}", error));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let vs_raw = include_bytes_align_as!(u32; concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/shader.vert.spv"
        ));

        let fs_raw = include_bytes_align_as!(u32; concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/shader.frag.spv"
        ));

        let vs_data: &[u32] = bytemuck::cast_slice(vs_raw);
        let fs_data: &[u32] = bytemuck::cast_slice(fs_raw);

        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vs_module"),
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::from(vs_data)),
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fs_module"),
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::from(fs_data)),
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
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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
                targets: &[Some(wgpu::ColorTargetState {
                    #[cfg(target_arch = "wasm32")]
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    #[cfg(not(target_arch = "wasm32"))]
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    write_mask: wgpu::ColorWrites::ALL,
                    blend: Some(wgpu::BlendState::REPLACE),
                })],
            }),
            depth_stencil: None,
            multiview: None,

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,                        //
                front_face: wgpu::FrontFace::Ccw,                // 2.
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_SRC,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::INDEX
                | wgpu::BufferUsages::COPY_SRC,
        });
        queue.submit(std::iter::once(encoder.finish()));
        let num_indices = INDICES.len() as u32;
        let (dcm, cmd) = decals::DecalContextManager::new(&device);
        queue.submit(std::iter::once(cmd));
        Self {
            surface,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            config,
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
        if let Ok(frame) = self.surface.get_current_texture() {
            //.expect("Timeout getting texture");

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: true,
                        },
                    })],
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
            frame.present();
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

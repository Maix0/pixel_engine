use super::graphics::{Color, Sprite};
use crate::gfx;
use gfx::{traits::FactoryExt, Device};
use glutin::{Event, WindowEvent};
use memblock::MemBlock;
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        awesome: gfx::TextureSampler<[f32; 4]> = "t_Awesome",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct GlHandler {
    event_loop: glutin::EventsLoop,
    window_ctx: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    data: pipe::Data<gfx_device_gl::Resources>,
}
#[derive(Debug, Clone, Copy)]
pub enum Events {
    /// A keyboard input
    Keyboard {
        /// The input
        inp: glutin::KeyboardInput,
    },
    Close,
}

//use glutin::{Event, WindowEvent};
impl GlHandler {
    const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const SQUARE: &'static [Vertex] = &[
        Vertex {
            pos: [1.0, -1.0],
            color: Self::WHITE,
            //uv: [1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [-1.0, -1.0],
            color: Self::WHITE,
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [-1.0, 1.0],
            color: Self::WHITE,
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [1.0, 1.0],
            color: Self::WHITE,
            uv: [1.0, 0.0],
        },
    ];

    pub fn events(&mut self) -> Vec<Events> {
        let mut events = Vec::new();
        self.event_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::KeyboardInput { input: inp, .. } => {
                        events.push(Events::Keyboard { inp });
                    }
                    WindowEvent::CloseRequested => {
                        events.push(Events::Close);
                    }
                    _ => {}
                }
            }
        });
        events
    }

    const INDICES: &'static [u16] = &[0, 1, 2, 2, 3, 0];
    pub fn new(size: (u32, u32, u32)) -> Self {
        use gfx::Factory;
        let events_loop = glutin::EventsLoop::new();
        let window_config = glutin::WindowBuilder::new()
            .with_title("".to_string())
            .with_dimensions((size.0 * size.2, size.1 * size.2).into())
            .with_resizable(false);

        let (api, version) = if cfg!(target_os = "emscripten") {
            (glutin::Api::WebGl, (2, 0))
        } else {
            (glutin::Api::OpenGl, (3, 2))
        };

        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(api, version))
            .with_vsync(false);
        let (window_ctx, device, mut factory, main_color, _) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(
                window_config,
                context,
                &events_loop,
            )
            .expect("Failed to create window");
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        let pso = factory
            .create_pipeline_simple(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_150.glslv"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_150.glslf"
                )),
                pipe::new(),
            )
            .unwrap();
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(&Self::SQUARE, Self::INDICES);
        let texture = Self::load_texture(&mut factory, Sprite::new_blank().get_raw());
        let sampler = factory.create_sampler(gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Scale,
            gfx::texture::WrapMode::Tile,
        )); //factory.create_sampler_linear();
            //let () = sampler;
        let data = pipe::Data {
            vbuf: vertex_buffer,
            awesome: (texture, sampler),
            out: main_color,
        };
        GlHandler {
            event_loop: events_loop,
            window_ctx,
            device,
            factory,
            encoder,
            pso,
            slice,
            data,
        }
    }
    pub fn load_texture<F, R>(
        factory: &mut F,
        img: MemBlock,
    ) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where
        F: gfx::Factory<R>,
        R: gfx::Resources,
    {
        let (width, height) = img.size();
        let kind =
            gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
        let (_, view) = factory
            .create_texture_immutable_u8::<ColorFormat>(
                kind,
                gfx::texture::Mipmap::Provided,
                &[&img],
            )
            .unwrap();
        view
    }
    pub fn update_title(&mut self, text: String) {
        self.window_ctx.window().set_title(&text);
    }
    pub fn update_frame(&mut self, image: MemBlock) {
        self.data.awesome.0 = Self::load_texture(&mut self.factory, image);
        self.encoder.clear(&self.data.out, Color::GREEN.into());
        self.encoder.draw(&self.slice, &self.pso, &self.data);
        self.encoder.flush(&mut self.device);
        self.window_ctx.swap_buffers().unwrap();
        self.device.cleanup();
    }
    /*
    fn spawn_thread(
        size: (u32, u32, u32),
        receiver_handler: std::sync::mpsc::Receiver<GLCommands>,
        sender_handler: std::sync::mpsc::Sender<GLEvents>,
    ) {
        std::thread::spawn(move || {
            use glutin::{Event, WindowEvent};
            let mut handler = GlHandler::new(size.clone());
            let mut events = Vec::new();
            while let Ok(msg) = receiver_handler.recv() {
                use GLCommands::*;
                match msg {
                    FrameUpdate { image } => handler.update_frame(image),
                    ChangeTitle { text } => handler.update_title(text),
                    Destroy => break,
                    RequestEvents => {
                        handler.event_loop.poll_events(|event| {
                            if let Event::WindowEvent { event, .. } = event {
                                match event {
                                    WindowEvent::KeyboardInput { input: inp, .. } => {
                                        events.push(Events::Keyboard { inp });
                                    }
                                    _ => {}
                                }
                            }
                        });

                        sender_handler
                            .send(GLEvents::Events { e: events })
                            .expect("Error while sending events");
                        events = Vec::new();
                    } //_ => println!("{:?}", msg),
                }
            }
        });
    }*/
}
/*
#[derive(Debug)]
/// Commands that are being sent to the Handler
pub enum GLCommands {
    /// Change the title
    ChangeTitle {
        /// New title
        text: String,
    },
    /// Change frame
    FrameUpdate {
        /// New image
        image: image::RgbaImage,
    },
    /// Request to destroy the Handler thread
    Destroy,
    /// Request processing of all events
    RequestEvents,
}
#[derive(Debug, Copy, Clone)]
/// Events that are returned after a request
pub enum Events {
    /// A keyboard input
    Keyboard {
        /// The input
        inp: glutin::KeyboardInput,
    },
}
*/
/*
enum GLEvents {
    Events { e: Vec<Events> },
}
*/
/*
#[derive(Debug)]
/// An Handle to talk to the GLHandler's Thread
pub struct GLHandle {
    /// Send message to the Handler
    pub sender: std::sync::mpsc::Sender<GLCommands>,
    /// Receive message from the Handler
    receiver: std::sync::mpsc::Receiver<GLEvents>,
}
*/

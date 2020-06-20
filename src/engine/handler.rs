use super::graphics::{Color, Sprite};
use super::logic::RenderBarrier;
use super::screen::Screen;
use super::traits::*;
use crate::gfx;
use gfx::{traits::FactoryExt, Device};
use parking_lot::Mutex;
use std::sync::Arc;
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct GlHandler {
    name: Arc<Mutex<String>>,
    _thread: std::thread::JoinHandle<()>,
    pub(crate) event_loop: glutin::EventsLoop,
}

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

    const INDICES: &'static [u16] = &[0, 1, 2, 2, 3, 0];

    pub fn update_title(&mut self, new_name: String) {
        let mut lock = self.name.lock();
        *lock = new_name;
    }

    pub(crate) fn new(
        size: (u32, u32, u32),
        rec: std::sync::mpsc::Receiver<RenderBarrier>,
        mutex: Arc<Mutex<Screen>>,
    ) -> Self {
        let name_in_struct = Arc::new(Mutex::new(String::new()));
        let name = name_in_struct.clone();
        let events_loop = glutin::EventsLoop::new();
        use gfx::Factory;
        let window_config = glutin::WindowBuilder::new()
            .with_title("".to_string())
            .with_dimensions((size.0 * size.2, size.1 * size.2).into())
            .with_resizable(false);

        let (api, version) = if cfg!(target_os = "emscripten") {
            (glutin::Api::WebGl, (2, 0))
        } else {
            (glutin::Api::OpenGl, (3, 2))
        };

        let context_wraper = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(api, version))
            .with_vsync(true)
            .build_windowed(window_config, &events_loop)
            .expect("Error while constructing context wraper");
        let render_thread = std::thread::spawn(move || {
            let (window_ctx, device, mut factory, main_color, _) =
                gfx_window_glutin::init_existing::<ColorFormat, DepthFormat>(context_wraper);
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
            let texture =
                GlInThread::load_texture(&mut factory, Sprite::new_blank().get_raw(), (1, 1));
            let sampler = factory.create_sampler(gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Scale,
                gfx::texture::WrapMode::Tile,
            )); //factory.create_sampler_linear();
                //let () = sampler;
            let data = pipe::Data {
                vbuf: vertex_buffer,
                texture: (texture, sampler),
                out: main_color,
            };

            let mut gl = GlInThread {
                window_ctx,
                device,
                factory,
                encoder,
                pso,
                slice,
                data,
                unblocking: rec,
            };
            loop {
                if gl.unblocking.recv().is_err() {
                    return;
                }
                let mut lock = mutex.lock();
                let name_lock = name.lock();
                let image_data = lock.get_raw();
                let size_img = lock.get_size();
                gl.update_frame(image_data, size_img);
                gl.update_title(name_lock.to_string());
            }
        });

        GlHandler {
            _thread: render_thread,
            name: name_in_struct,
            event_loop: events_loop,
        }
    }
}
struct GlInThread {
    window_ctx: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    data: pipe::Data<gfx_device_gl::Resources>,
    unblocking: std::sync::mpsc::Receiver<RenderBarrier>,
}
impl GlInThread {
    pub fn load_texture<F, R>(
        factory: &mut F,
        img: Box<[u8]>,
        size: (usize, usize),
    ) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where
        F: gfx::Factory<R>,
        R: gfx::Resources,
    {
        let (width, height) = size;
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
    pub fn update_frame(&mut self, image: Box<[u8]>, size: (usize, usize)) {
        self.data.texture.0 = Self::load_texture(&mut self.factory, image, size);
        self.encoder.clear(&self.data.out, Color::GREEN.into());
        self.encoder.draw(&self.slice, &self.pso, &self.data);
        self.encoder.flush(&mut self.device);
        self.window_ctx.swap_buffers().unwrap();
        self.device.cleanup();
    }
}

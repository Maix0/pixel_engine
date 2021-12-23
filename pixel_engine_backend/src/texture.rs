pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: wgpu::Extent3d,
}

impl Texture {
    /// # Panic
    ///
    /// If the given bytes length isn't equal to the `size.0 * size.1 * 4`
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: (&[u8], (u32, u32)),
    ) -> Self {
        let (rgba, dimensions) = img;
        if rgba.len() as u32 != dimensions.0 * dimensions.1 * 4 {
            panic!("Data given isn't at the given size")
        }

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            // array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("texture"),
        });

        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: &rgba,
        //     usage: wgpu::BufferUsages::COPY_SRC,
        // });

        // let mut encoder =
        //     device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // encoder.copy_buffer_to_texture(
        //     wgpu::BufferCopyView {
        //         buffer: &buffer,
        //         layout: wgpu::TextureDataLayout {
        //             offset: 0,
        //             bytes_per_row: 4 * dimensions.0,
        //             rows_per_image: dimensions.1,
        //         },
        //     },
        //     wgpu::TextureCopyView {
        //         texture: &texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //     },
        //     size,
        // );

        // let cmd_buffer = encoder.finish(); // 2.

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTextureBase {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The actual pixel data
            rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
            base_mip_level: 0,
            aspect: wgpu::TextureAspect::All,
            base_array_layer: 0,
            array_layer_count: None,
            dimension: Some(wgpu::TextureViewDimension::D2),
            mip_level_count: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            border_color: None,
            anisotropy_clamp: None,
        });

        Self {
            texture,
            view,
            sampler,
            size,
        }
    }
    pub fn update(&self, queue: &wgpu::Queue, data: &[u8]) {
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTextureBase {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The actual pixel data
            data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );
    }
}

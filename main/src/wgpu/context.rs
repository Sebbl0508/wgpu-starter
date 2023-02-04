use super::texture;
use winit::window::Window;

pub struct WgpuContext {
    device: wgpu::Device,
    adapter: wgpu::Adapter,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_capabilities: wgpu::SurfaceCapabilities,
    surface_config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    depth_texture: texture::Texture,
}

impl WgpuContext {
    pub async fn new(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(window) }?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("primary device"),
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let depth_texture = texture::Texture::create_depth_texture(
            &device,
            &surface_config,
            Some("main depth texture"),
        );

        Ok(Self {
            device,
            adapter,
            queue,
            surface,
            surface_config,
            surface_capabilities,
            size,
            depth_texture,
        })
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }

    pub fn depth_texture(&self) -> &texture::Texture {
        &self.depth_texture
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;

            self.surface.configure(&self.device, &self.surface_config);
            self.depth_texture = texture::Texture::create_depth_texture(
                &self.device,
                &self.surface_config,
                Some("main depth texture"),
            );
        }
    }
}

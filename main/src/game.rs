use crate::wgpu::context::WgpuContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};

pub struct Game {
    ctx: WgpuContext,
    window: Window,
    event_loop: Option<EventLoop<()>>,
}

impl Game {
    pub async fn new(
        event_loop: EventLoop<()>,
        window: Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ctx = WgpuContext::new(&window).await?;

        Ok(Self {
            ctx,
            window,
            event_loop: Some(event_loop),
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ctx.resize(new_size);
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.ctx.surface().get_current_texture()?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.ctx
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("main command encoder"),
                });

        {
            let mut _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main renderpass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.ctx.depth_texture().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
        }

        self.ctx.queue().submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(new_size) => {
                    self.resize(new_size);
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size,
                } => {
                    self.resize(*new_inner_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(_) => match self.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    self.resize(self.ctx.size());
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Timeout) => log::warn!("surface timeout, ignoring..."),
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        });
    }
}

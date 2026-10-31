use std::{ops::Deref, sync::Arc};

use crate::{
    engine::Engine,
    runner::{RenderTarget, Runner, WgpuHandles, WindowHandle},
};

pub struct StandardRunner;

impl Runner for StandardRunner {
    fn run(self, engine: &mut Engine<Self>) {
        let mut app = App::new(engine);
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }
}

struct App<'engine> {
    started: bool,
    engine: &'engine mut Engine<StandardRunner>,
    window: Option<Arc<winit::window::Window>>,
}

impl<'engine> App<'engine> {
    fn new(engine: &'engine mut Engine<StandardRunner>) -> Self {
        Self {
            started: false,
            engine,
            window: None,
        }
    }

    fn start(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.started = true;
        let window_handle = window_handle(event_loop);

        let wgpu_handles =
            pollster::block_on(async { wgpu_handles(Arc::clone(&window_handle.deref())).await })
                .expect("Failed to initialize wgpu");

        self.window = Some(Arc::clone(&window_handle.deref()));

        self.engine.world().insert_resource(window_handle);
        self.engine.world().insert_resource(wgpu_handles);

        self.engine.build_plugins();

        self.engine.setup();
    }
}
impl winit::application::ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.started {
            self.start(event_loop);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                self.engine.update();

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            winit::event::WindowEvent::CloseRequested | winit::event::WindowEvent::Destroyed => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn window_handle(event_loop: &winit::event_loop::ActiveEventLoop) -> WindowHandle {
    let window = Arc::new(
        event_loop
            .create_window(winit::window::WindowAttributes::default())
            .unwrap(),
    );

    WindowHandle(Arc::clone(&window))
}

async fn wgpu_handles(window: Arc<winit::window::Window>) -> anyhow::Result<WgpuHandles> {
    let width = window.inner_size().width;
    let height = window.inner_size().height;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    let surface = instance.create_surface(window)?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: true,
        })
        .await?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        })
        .await?;

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            present_mode: wgpu::PresentMode::AutoVsync,
            ..surface
                .get_default_config(&adapter, width, height)
                .expect("Failed to get default surface configuration")
        },
    );

    let target_format = surface
        .get_capabilities(&adapter)
        .formats
        .first()
        .cloned()
        .expect("Failed to get wgpu surface format");

    Ok(WgpuHandles {
        render_target: RenderTarget::Surface(surface),
        target_format,
        adapter,
        device,
        queue,
    })
}

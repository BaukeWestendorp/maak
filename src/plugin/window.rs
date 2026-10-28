use std::sync::Arc;

use bevy_ecs::resource::Resource;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

use crate::{app::App, plugin::Plugin};

pub struct WindowPlugin {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");

        let window_attrs = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));

        let window = event_loop
            .create_window(window_attrs)
            .expect("Failed to create window");

        let window_arc = Arc::new(window);

        app.world().insert_resource(MainWindow(window_arc));

        app.set_runner(move |mut app: App| {
            struct WinitAppWrapper {
                app: App,
            }

            impl ApplicationHandler for WinitAppWrapper {
                fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

                fn window_event(
                    &mut self,
                    event_loop: &ActiveEventLoop,
                    _id: WindowId,
                    event: WindowEvent,
                ) {
                    match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::RedrawRequested => {
                            self.app.update_schedule.run(&mut self.app.world);

                            if let Some(window) = self.app.world.get_resource::<MainWindow>() {
                                window.0.request_redraw();
                            }
                        }
                        _ => {}
                    }
                }
            }

            app.setup_schedule.run(&mut app.world);

            let mut wrapper = WinitAppWrapper { app };
            event_loop
                .run_app(&mut wrapper)
                .expect("Error running event loop");
        });
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MainWindow(pub Arc<winit::window::Window>);

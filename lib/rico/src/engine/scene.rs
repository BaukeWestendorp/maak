use std::rc::Rc;

use crate::engine::Backend;

pub struct Scene<B: Backend> {
    world: hecs::World,

    on_setup: Option<Rc<dyn Fn(&Self, &mut B)>>,
    on_update: Option<Rc<dyn Fn(&Self, f32, &mut B)>>,
    on_shutdown: Option<Rc<dyn Fn(&Self, &mut B)>>,

    active_camera: Option<hecs::Entity>,
}

impl<B: Backend> Scene<B> {
    pub fn new() -> Self {
        Self {
            world: hecs::World::new(),

            on_setup: None,
            on_update: None,
            on_shutdown: None,

            active_camera: None,
        }
    }

    pub fn on_setup<F>(mut self, f: F) -> Self
    where
        F: Fn(&Self, &mut B) + 'static,
    {
        self.on_setup = Some(Rc::new(f));
        self
    }

    pub fn on_update<F>(mut self, f: F) -> Self
    where
        F: Fn(&Self, f32, &mut B) + 'static,
    {
        self.on_update = Some(Rc::new(f));
        self
    }

    pub fn on_shutdown<F>(mut self, f: F) -> Self
    where
        F: Fn(&Self, &mut B) + 'static,
    {
        self.on_shutdown = Some(Rc::new(f));
        self
    }

    pub fn active_camera(&self) -> Option<hecs::Entity> {
        self.active_camera
    }

    pub fn set_active_camera(&mut self, active_camera: Option<hecs::Entity>) {
        self.active_camera = active_camera;
    }

    pub fn spawn<C, I>(&mut self, bundle: impl hecs::Bundle) -> hecs::Entity {
        self.world.spawn(bundle)
    }

    pub(crate) fn setup(&mut self, cx: &mut B) {
        if let Some(on_setup) = &self.on_setup {
            on_setup(self, cx);
        }

        // for handle in handles {
        //     let Some(entity) = entities.get_mut(&handle) else { continue };

        //     for component in &mut entity.components {
        //         component.setup(handle, self, cx);
        //     }
        // }
    }

    pub(crate) fn update(&mut self, delta_time: f32, cx: &mut B) {
        if let Some(on_update) = &self.on_update {
            on_update(self, delta_time, cx);
        }

        // for handle in handles {
        //     let Some(entity) = entities.get_mut(&handle) else { continue };

        //     for component in &mut entity.components {
        //         component.update(delta_time, handle, self, cx);
        //     }
        // }
    }

    pub(crate) fn shutdown(&mut self, cx: &mut B) {
        if let Some(on_shutdown) = &self.on_shutdown {
            on_shutdown(self, cx);
        }

        // for handle in handles {
        //     let Some(entity) = entities.get_mut(&handle) else { continue };

        //     for component in &mut entity.components {
        //         component.shutdown(handle, self, cx);
        //     }
        // }
    }
}

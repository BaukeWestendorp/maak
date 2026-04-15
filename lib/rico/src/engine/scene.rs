use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ecs::{Bundle, Component, ComponentHandle, Entity, EntityHandle};
use crate::engine::Backend;

pub struct Scene<B: Backend> {
    entities: BTreeMap<EntityHandle, Entity>,
    components: BTreeMap<ComponentHandle, Box<dyn Component<B>>>,
    commands: Commands<B>,

    on_setup: Option<Rc<dyn Fn(&Self, &mut B)>>,
    on_update: Option<Rc<dyn Fn(&Self, f32, &mut B)>>,
    on_shutdown: Option<Rc<dyn Fn(&Self, &mut B)>>,

    active_camera: Option<EntityHandle>,
}

impl<B: Backend> Scene<B> {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::default(),
            components: BTreeMap::default(),
            commands: Commands::default(),
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

    pub fn active_camera(&self) -> Option<EntityHandle> {
        self.active_camera
    }

    pub fn set_active_camera(&mut self, active_camera: Option<EntityHandle>) {
        self.active_camera = active_camera;
    }

    pub fn query<C: Component<B> + 'static>(&self) -> impl Iterator<Item = &C> {
        self.components.values().filter_map(|c| c.as_any().downcast_ref::<C>())
    }

    pub fn query_in<C: Component<B> + 'static>(
        &self,
        entity: EntityHandle,
    ) -> impl Iterator<Item = &C> {
        self.entities
            .get(&entity)
            .into_iter()
            .flat_map(|entity| entity.components.iter())
            .filter_map(|c| c.as_any().downcast_ref::<C>())
    }

    pub fn commands(&mut self) -> &mut Commands<B> {
        &mut self.commands
    }

    fn apply_commands(&mut self) {
        let mut entities = self.entities.borrow_mut();
        for cmd in self.commands.drain() {
            match cmd {
                Command::Spawn { handle, components } => {
                    entities.insert(handle, Entity::from_components(components));
                }
            }
        }
    }

    pub(crate) fn setup(&mut self, cx: &mut B) {
        if let Some(on_setup) = &self.on_setup {
            on_setup(self, cx);
            self.apply_commands();
        }

        let handles: Vec<EntityHandle> = {
            let entities = self.entities.borrow();
            entities.keys().copied().collect()
        };

        for handle in handles {
            let mut entities = self.entities.borrow_mut();
            let Some(entity) = entities.get_mut(&handle) else { continue };

            for component in &mut entity.components {
                component.setup(handle, self, cx);
            }
        }

        self.apply_commands();
    }

    pub(crate) fn update(&mut self, delta_time: f32, cx: &mut B) {
        if let Some(on_update) = &self.on_update {
            on_update(self, delta_time, cx);
            self.apply_commands();
        }

        let handles: Vec<EntityHandle> = {
            let entities = self.entities.borrow();
            entities.keys().copied().collect()
        };

        for handle in handles {
            let mut entities = self.entities.borrow_mut();
            let Some(entity) = entities.get_mut(&handle) else { continue };

            for component in &mut entity.components {
                component.update(delta_time, handle, self, cx);
            }
        }

        self.apply_commands();
    }

    pub(crate) fn shutdown(&mut self, cx: &mut B) {
        if let Some(on_shutdown) = &self.on_shutdown {
            on_shutdown(self, cx);
            self.apply_commands();
        }

        let handles: Vec<EntityHandle> = {
            let entities = self.entities.borrow();
            entities.keys().copied().collect()
        };

        for handle in handles {
            let mut entities = self.entities.borrow_mut();
            let Some(entity) = entities.get_mut(&handle) else { continue };

            for component in &mut entity.components {
                component.shutdown(handle, self, cx);
            }
        }

        self.apply_commands();
    }
}

enum Command<B: Backend> {
    Spawn { handle: EntityHandle, components: Vec<Box<dyn Component<B>>> },
}

pub struct Commands<B: Backend> {
    queue: Vec<Command<B>>,
}

impl<B: Backend> Default for Commands<B> {
    fn default() -> Self {
        Self { queue: Vec::new() }
    }
}

impl<B: Backend> Commands<B> {
    pub fn spawn(&mut self, bundle: impl Bundle<B>) -> EntityHandle {
        let handle = EntityHandle::new();
        self.queue.push(Command::Spawn { handle, components: bundle.into_components() });
        handle
    }

    fn drain(&mut self) -> std::vec::Drain<'_, Command<B>> {
        self.queue.drain(..)
    }
}

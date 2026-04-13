use crate::engine::RaylibContext;
use crate::prelude::*;

slotmap::new_key_type! { pub struct SceneHandle; }

pub struct SceneData {
    entities: slotmap::DenseSlotMap<EntityHandle, Entity>,
}

impl SceneData {
    pub fn new() -> Self {
        Self { entities: slotmap::DenseSlotMap::default() }
    }

    pub fn query<C: Component + 'static>(&self) -> impl Iterator<Item = &C> {
        self.entities
            .values()
            .flat_map(|e| e.components())
            .filter_map(|c| c.as_any().downcast_ref::<C>())
    }

    pub fn spawn(&mut self, components: impl Bundle) -> EntityHandle {
        self.entities.insert(Entity::new(components))
    }
}

pub trait Scene {
    fn setup(&mut self, _cx: &mut SceneData) {}

    fn update(&mut self, _cx: &mut SceneData) {}

    fn shutdown(&mut self, _cx: &mut SceneData) {}
}

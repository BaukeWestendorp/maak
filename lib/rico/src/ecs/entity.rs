use crate::prelude::*;

slotmap::new_key_type! { pub struct EntityHandle; }

pub struct Entity {
    components: Vec<Box<dyn Component>>,
    parent: Option<EntityHandle>,
}

impl Entity {
    pub fn new(bundle: impl Bundle) -> Self {
        Self { components: bundle.into_iter().collect(), parent: None }
    }

    pub fn components(&self) -> &[Box<dyn Component + 'static>] {
        &self.components
    }

    pub fn parent(&self) -> Option<EntityHandle> {
        self.parent
    }
}

use crate::ecs::ComponentHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityHandle(uuid::Uuid);

impl EntityHandle {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

pub struct Entity {
    pub(crate) components: Vec<ComponentHandle>,
    pub(crate) parent: Option<EntityHandle>,
}

impl Entity {
    pub fn components(&self) -> &[ComponentHandle] {
        &self.components
    }

    pub fn parent(&self) -> Option<EntityHandle> {
        self.parent
    }
}

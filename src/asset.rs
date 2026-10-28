use std::collections::HashMap;

use bevy_ecs::resource::Resource;

#[derive(Resource)]
pub struct Assets<T> {
    items: HashMap<usize, T>,
    next_id: usize,
}

impl<T> Assets<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, item: T) -> AssetHandle<T> {
        let id = self.next_id;
        self.items.insert(id, item);
        self.next_id += 1;
        AssetHandle {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get(&self, handle: &AssetHandle<T>) -> Option<&T> {
        self.items.get(&handle.id)
    }
}

pub struct AssetHandle<T> {
    id: usize,
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetHandle<T> {}

impl<T> PartialEq for AssetHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for AssetHandle<T> {}

impl<T> std::hash::Hash for AssetHandle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> std::fmt::Debug for AssetHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AssetHandle").field(&self.id).finish()
    }
}

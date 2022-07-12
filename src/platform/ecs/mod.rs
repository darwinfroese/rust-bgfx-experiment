pub mod component;
pub mod entity;
pub mod system;

use crate::platform::error;

pub struct EcsManager {
    entities: Vec<entity::Entity>,
    systems: Vec<Box<dyn system::System>>,

    next_entity_id: u16,
}

impl EcsManager {
    pub fn new() -> EcsManager {
        EcsManager {
            systems: Vec::new(),
            entities: Vec::new(),
            next_entity_id: 0,
        }
    }

    pub fn create_entity(mut self) -> error::Result<u16> {
        if self.next_entity_id >= std::u16::MAX {
            return Err(error::PlatformError::EcsMaxEntitiesError());
        }

        let id = self.next_entity_id;
        self.next_entity_id += 1;

        self.entities.push(entity::Entity::new(id));
        Ok(id)
    }

    pub fn register_system<S: system::System>() {}

    pub fn run_systems(&self) {
        for system in &self.systems {
            system.run();
        }
    }
}

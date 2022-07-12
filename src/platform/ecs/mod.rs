pub mod component;
pub mod entity;
pub mod system;

pub struct EcsManager {
    systems: Vec<Box<dyn system::System>>,
}

impl EcsManager {
    pub fn new() -> EcsManager {
        EcsManager {
            systems: Vec::new(),
        }
    }

    pub fn create_entity() -> u16 {
        0
    }

    pub fn register_system<S: system::System>() {}

    pub fn run_systems(&self) {
        for system in &self.systems {
            system.run();
        }
    }
}

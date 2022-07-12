use crate::platform::ecs::component;

pub struct Entity {
    id: u16,

    components: Vec<Box<dyn component::Component>>,
}

impl Entity {
    pub(crate) fn new(id: u16) -> Entity {
        Entity {
            id,
            components: Vec::with_capacity(16), // TODO: make this configurable; pre-allocated to reduce memory calls on resize
        }
    }

    pub(crate) fn get_id(&self) -> u16 {
        self.id
    }

    pub(crate) fn add_component<C: 'static + component::Component>(mut self, component: C) {
        self.components.push(Box::new(component))
    }
}

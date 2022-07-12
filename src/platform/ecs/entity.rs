use crate::platform::ecs::component;

pub struct Entity {
    id: u16,

    components: Vec<Box<dyn component::Component>>,
}

impl Entity {
    pub(crate) fn new(id: u16) -> Entity {
        Entity {
            id,
            components: Vec::with_capacity(16), // TODO: do this better
        }
    }
    fn add_component<C: component::Component>(&self, component: C) {}
}

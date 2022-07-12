use crate::platform::ecs::component;

pub trait Entity {
    fn add_component<C: component::Component>(self, component: &C);
}

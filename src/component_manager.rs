use super::{
    bag::Bag,
    component::{Component, TestComponent},
    instance::EcsInstance,
};

#[derive(Debug)]
pub struct ComponentManager<'a, T: Component> {
    instance: &'a EcsInstance,
    components: Bag<Bag<T>>,
    next_type_id: i32,
}

impl<'a, T: Component> ComponentManager<'a, T> {
    pub fn new(instance: &'a EcsInstance) -> Self {
        Self {
            instance,
            components: Bag::<Bag<T>>::default(),
            next_type_id: 0,
        }
    }

    pub fn register_component_type() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let instance = EcsInstance::default();
        let cm = ComponentManager::<TestComponent>::new(&instance);
    }
}

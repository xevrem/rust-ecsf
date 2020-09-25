use super::{
    bag::Bag,
    component::{Component, TestComponent},
    entity::Entity,
    instance::EcsInstance,
};

// #[derive(Debug)]
pub struct ComponentManager<'a, T: Component + Clone> {
    instance: &'a EcsInstance,
    components: Bag<Bag<Box<dyn Component, Clone>>>,
    next_type_id: usize,
}

impl<'a, T: Component + Clone> ComponentManager<'a, T> {
    pub fn new(instance: &'a EcsInstance) -> ComponentManager<'a, T> {
        ComponentManager {
            instance,
            components: Bag::<Bag<Box<dyn Component>>>::new(16_usize),
            next_type_id: 0,
        }
    }

    pub fn register_component_type<T>(&mut self, mut component: T)
    where
        T: Component,
    {
        if component.get_id() == 0 {
            component.set_id(self.next_type_id);
            self.next_type_id += 1;
        }
        if component.get_id() < self.components.capacity() {
            match self.components.get(component.get_id()) {
                None => {
                    let new_bag = Bag::<Box<dyn Component>>::new(16_usize);
                    self.components.set(component.get_id(), new_bag);
                }
                _ => {}
            }
        } else {
            let new_bag = Bag::<Box<dyn Component>>::new(16_usize);
            self.components.set(component.get_id(), new_bag);
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Box<dyn Component>) {
        match self.components.get(component.get_id()) {
            Some(val) => {
                let foo: &mut Bag<Box<dyn Component>> = val;
                foo.set(entity.id, component);
            }
            None => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::TestComponent;
    use super::*;

    #[test]
    fn test_creation() {
        let instance = EcsInstance::new();
        ComponentManager::new(&instance);
    }

    #[test]
    fn test_register_component() {
        let instance = EcsInstance::new();
        let mut cm = ComponentManager::new(&instance);
        let mut tc = TestComponent::new();
        cm.register_component_type(tc);
    }
}

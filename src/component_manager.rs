use super::{
    bag::Bag,
    component::{Component, TestComponent},
    instance::EcsInstance,
};

// #[derive(Debug)]
pub struct ComponentManager<'a> {
    instance: &'a EcsInstance,
    components: Bag<'a, &'a Bag<'a, Box<dyn Component>>>,
    next_type_id: usize,
}

impl<'a> ComponentManager<'a> {
    pub fn new(instance: &'a EcsInstance) -> ComponentManager<'a> {
        ComponentManager {
            instance,
            components: Bag::<'a, &'a Bag<'a, Box<dyn Component>>>::new(16_usize),
            next_type_id: 0,
        }
    }

    pub fn register_component_type<T>(&mut self, component: T)
    where
        T: Component,
    {
        if component.get_type() == 0 {
            component.set_type(self.next_type_id);
            self.next_type_id += 1;
        }
        if component.get_type() < self.components.capacity() {
            match component.get_type() {
                0 => self
                    .components
                    .set(component.get_type(), Bag::<'a, Box<dyn Component>>::new(16_usize)),
                _ => {}
            }
        // if self.components.get(component.get_type()) == None {

        // }
        } else {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let instance = EcsInstance::new();
        let cm = ComponentManager::new(&instance);
    }
}

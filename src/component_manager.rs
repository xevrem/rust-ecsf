use super::{bag::Bag, component::Component, entity::Entity, instance::EcsInstance};

// #[derive(Debug)]
pub struct ComponentManager<'a> {
    instance: &'a EcsInstance,
    components: Bag<Bag<Box<dyn Component>>>,
    next_type_id: usize,
}

impl<'a> ComponentManager<'a> {
    pub fn new(instance: &'a EcsInstance) -> ComponentManager<'a> {
        ComponentManager {
            instance,
            components: Bag::<Bag<Box<dyn Component>>>::new(16_usize),
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
                0 => {
                    let new_bag = Bag::<Box<dyn Component>>::new(16_usize);
                    self.components.set(component.get_type(), new_bag);
                }
                _ => {}
            }
        // if self.components.get(component.get_type()) == None {

        // }
        } else {
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
    use super::*;

    #[test]
    fn test_creation() {
        let instance = EcsInstance::new();
        let cm = ComponentManager::new(&instance);
    }
}

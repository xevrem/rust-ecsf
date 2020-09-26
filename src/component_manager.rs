use super::{
    bag::Bag,
    component::{Component, TestComponent},
    entity::Entity,
    instance::EcsInstance,
};

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

    pub fn register_component_type<T>(&mut self, component: &mut T)
    where
        T: Component,
    {
        if component.get_id() == 0 {
            component.set_id(self.next_type_id);
            self.next_type_id += 1;
        }
        if component.get_id() < self.components.capacity() {
            if let None = self.components.get(component.get_id()) {
                self.components
                    .set(component.get_id(), Bag::<Box<dyn Component>>::new(16_usize));
            }
        // match self.components.get(component.get_id()) {
        //     None => {
        //         self.components
        //             .set(component.get_id(), Bag::<Box<dyn Component>>::new(16_usize));
        //     }
        //     _ => {}
        // }
        } else {
            self.components
                .set(component.get_id(), Bag::<Box<dyn Component>>::new(16_usize));
        }
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: Box<dyn Component>)
    where
        T: Component,
    {
        if let Some(ref mut val) = self.components.get(component.get_id()) {
            val.set(entity.id, component);
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
        assert!(tc.id == 0, "default id not 0");
        assert!(cm.components.count == 0, "default count not 0");
        println!("capacity::{}", cm.components.capacity());
        println!("count::{}", cm.components.count);
        cm.register_component_type(&mut tc);
        println!("count::{}", cm.components.count);
        println!("length::{}", cm.components.data.len());
        assert!(
            cm.components.count == 1_usize,
            "did not increase components"
        );
        let mut bar = TestComponent::new();
        cm.register_component_type(&mut bar);
        assert!(bar.id != 0, "default id is still 0");
    }

    #[test]
    fn test_add_component() {
        let instance = EcsInstance::new();
        let mut cm = ComponentManager::new(&instance);
    }
}

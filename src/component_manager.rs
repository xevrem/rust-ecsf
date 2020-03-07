
use super::{instance::EcsInstance, bag::Bag, component::Component};


#[derive(Debug)]
pub struct ComponentManager<'a> {
    instance: &'a EcsInstance,
    components: Bag<Component>,
    next_type_id: i32,
}

impl<'a> ComponentManager<'a> {
    pub fn new(instance: &'a EcsInstance) -> Self {
        Self{
            instance,
            components: Bag::<Component>::new(16),
            next_type_id: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let instance = EcsInstance::default();
        let cm = ComponentManager::new(&instance);


    }
}

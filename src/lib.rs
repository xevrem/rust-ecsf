
mod bag;
mod entity;

pub use crate::entity::Entity;


#[cfg(test)]
mod tests_library {
    use super::*;

#[test]
    fn test_entity_available() {
        let e: Entity = Entity { id: 5 };
        assert!(e.id == 5, "e not instantiated");
    }
}

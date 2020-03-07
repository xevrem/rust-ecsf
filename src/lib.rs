mod bag;
mod entity;

pub use bag::Bag;
pub use entity::Entity;

#[cfg(test)]
mod tests_library {
    use super::*;

    #[test]
    fn test_entity_available() {
        let e: Entity = Entity { id: 5 };
        assert!(e.id == 5, "e not instantiated");
    }

    #[test]
    fn test_bag_available() {
        let b: Bag<i32> = Bag::<i32>::new(5);
        assert!(b.length == 5, "bag does not have set length")
    }
}

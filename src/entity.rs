#[derive(Debug, Default)]
pub struct Entity {
    pub id: usize,
}

#[cfg(test)]
mod tests_entity {
    use super::*;

    #[test]
    fn test_entity_id() {
        let e = Entity { id: 4 };
        assert!(e.id == 4, "entity id not created")
    }
}

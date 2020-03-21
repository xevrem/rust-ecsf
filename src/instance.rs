
use super::entity::Entity;

#[derive(Debug)]
pub struct EcsInstance {
    updating : Vec<Entity>,
    deleting : Vec<Entity>,
    delta : f32,
    last_time : f32,
    elapsed : f32,
}

impl EcsInstance {
    pub fn new() -> EcsInstance {
        EcsInstance {
            updating: Vec::<Entity>::new(),
            deleting: Vec::<Entity>::new(),
            delta: 0.0,
            last_time: 0.0,
            elapsed: 0.0
        }
    }
}


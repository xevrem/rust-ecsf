pub trait Component {
    fn get_id(&self) -> i32;
    fn set_id(&self) -> i32;
    fn get_type(&self) -> i32;
    fn set_type(&self) -> i32;
    fn get_owner(&self) -> i32;
    fn set_owner(&self) -> i32;
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct TestComponent {
    pub id: i32,
    pub ctype: i32,
    pub owner: i32,
}

impl Component for TestComponent {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_type(&self) -> i32 {
        unimplemented!()
    }
    fn set_owner(&self) -> i32 {
        unimplemented!()
    }
    fn get_owner(&self) -> i32 {
        unimplemented!()
    }
    fn set_type(&self) -> i32 {
        unimplemented!()
    }
    fn set_id(&self) -> i32 {
        unimplemented!()
    }
}

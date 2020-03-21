pub trait Component {
    fn get_id(&self) -> usize;
    fn set_id(&self, value: usize) -> ();
    fn get_type(&self) -> usize;
    fn set_type(&self, value: usize) -> ();
    fn get_owner(&self) -> usize;
    fn set_owner(&self, value: usize) -> ();
    // fn default(&self) -> Self
    // where
    //     Self: Sized;
    // fn copy(&self) -> Self
    // where
    //     Self: Sized;
    // fn eq(&self, other: &Self) -> bool
    // where
    //     Self: Sized;
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct TestComponent {
    pub id: i32,
    pub ctype: i32,
    pub owner: i32,
}

impl Component for TestComponent {
    fn get_id(&self) -> usize {
        unimplemented!()
    }
    fn set_id(&self, value: usize) {
        unimplemented!()
    }
    fn get_owner(&self) -> usize {
        unimplemented!()
    }
    fn set_owner(&self, value: usize) {
        unimplemented!()
    }
    fn get_type(&self) -> usize {
        unimplemented!()
    }
    fn set_type(&self, value: usize) {
        unimplemented!()
    }
}

pub trait Component {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, value: usize);
    fn get_owner(&self) -> usize;
    fn set_owner(&mut self, value: usize);
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
    pub id: usize,
    pub owner: usize,
}

impl TestComponent {
    pub fn new() -> TestComponent {
        TestComponent { id: 0, owner: 0 }
    }
}

impl Component for TestComponent {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, value: usize) -> () {
        self.id = value;
    }

    fn get_owner(&self) -> usize {
        self.owner
    }

    fn set_owner(&mut self, value: usize) -> () {
        self.owner = value;
    }
}

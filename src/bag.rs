#[derive(Debug)]
pub struct Bag<T> {
    pub data: Vec<T>,
    pub count: usize,
    pub length: usize,
}

impl<T> Default for Bag<T> {
    fn default() -> Self {
        Self {
            count: 0,
            data: Vec::<T>::with_capacity(16_usize),
            length: 16_usize,
        }
    }
}

impl<T> Bag<T> {
    pub fn new(length: usize) -> Self {
        Self {
            count: 0,
            data: Vec::<T>::with_capacity(length),
            length,
        }
    }
}

#[cfg(test)]
mod tests_bag {
    use super::*;

    #[test]
    fn test_bag_new() {
        let bag = Bag::<i32>::new(5);

        assert!(bag.count == 0, "count not set to 0");
        assert!(bag.data ==  Vec::<i32>::with_capacity(5) , "data not set to vec of given length");
        assert!(bag.length == 5_usize, "length set to given length");
    }
}

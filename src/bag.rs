#[derive(Debug)]
pub struct Bag<T> {
    pub data: Vec<Option<T>>,
    pub count: usize,
    pub length: usize,
}

// impl<T> Default for Bag<T> {
//     fn default() -> Self {
//         Self {
//             count: 0,
//             data: Vec::<T>::with_capacity(16_usize),
//             length: 16_usize,
//         }
//     }
//}

impl<T> Bag<T> {
    pub fn new(length: usize) -> Bag<T> {
        let mut data = Vec::<Option<T>>::new();
        for _i in 0..length {
            data.push(None);
        }
        Bag {
            count: 0,
            data,
            length,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        match self.data.get_mut(index) {
            Some(val) => match val {
                Some(foo) => Some(foo),
                None => None,
            },
            None => None,
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.data.len() {
            self.grow(index * 2);
            self.count = index + 1;
        } else if index >= self.count {
            self.count = index + 1;
        }

        self.data[index] = Some(value);
    }

    pub fn add(&mut self, element: T) {
        if self.count + 1 >= self.data.len() {
            self.grow(self.data.len() * 2 + 1);
        }
        self.data[self.count] = Some(element);
        self.count += 1;
    }

    pub fn add_range(&mut self, other: &Bag<T>)
    where
        T: Clone,
    {
        // for i in 0..other.count {
        //     match other.data.remove(i) {
        //         Some(val) => self.add(Some(val)),
        //         None => {}
        //     }
        // }
        self.data.clone_from_slice(other.data.as_slice())
    }

    pub fn clear(&mut self) {
        self.data = Vec::<Option<T>>::with_capacity(self.data.len());
        self.count = 0;
    }

    pub fn contains(&self, element: T) -> bool
    where
        T: PartialEq,
    {
        self.data.contains(&Some(element))
    }

    pub fn index_of(&self, element: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        for i in 0..self.data.len() {
            match &self.data[i] {
                Some(val) if val == element => return Some(i),
                _ => {}
            }
        }
        None
    }

    pub fn remove(&mut self, element: T) -> Option<T>
    where
        T: PartialEq,
    {
        match self.index_of(&element) {
            Some(index) => {
                self.data.push(None);
                let value = self.data.swap_remove(index);
                self.count -= 1;
                value
            }
            None => None,
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            let value = self.data.remove(index);
            self.data[index] = None;
            self.count -= 1;
            value
        } else {
            None
        }
    }

    pub fn remove_last(&mut self) -> Option<T> {
        let value = match self.data.pop() {
            Some(val) => val,
            None => None,
        };
        self.count -= 1;
        value
    }

    pub fn grow(&mut self, size: usize) {
        let mut new = Vec::<Option<T>>::with_capacity(size);
        for i in 0..self.data.len() {
            new[i] = self.data.remove(i);
        }
        self.data = new;
    }
}

#[cfg(test)]
mod tests_bag {
    use super::*;

    #[test]
    fn test_bag_new() {
        let bag = Bag::<i32>::new(5);

        assert!(bag.count == 0, "count not set to 0");
        assert!(bag.data.len() == 5, "data not set to vec of given length");
        assert!(bag.length == 5_usize, "length set to given length");
    }
}

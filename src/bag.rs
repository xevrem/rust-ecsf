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

impl<T: Copy + PartialEq + Default> Bag<T> {
    pub fn new(length: usize) -> Self {
        Self {
            count: 0,
            data: Vec::<T>::with_capacity(length),
            length,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get(&self, index: usize) -> T {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.data.len() {
            self.grow(index * 2);
            self.count = index + 1;
        } else if index >= self.count {
            self.count = index + 1;
        }

        self.data[index] = value;
    }

    pub fn add(&mut self, element: T) {
        if self.count + 1 >= self.data.len() {
            self.grow(self.data.len() * 2 + 1);
        }
        self.data[self.count] = element;
        self.count += 1;
    }

    pub fn add_range(&mut self, bag: Bag<T>) {
        for i in 0..bag.count {
            self.add(bag.get(i));
        }
    }

    pub fn clear(&mut self) {
        self.data = Vec::<T>::with_capacity(self.data.len());
        self.count = 0;
    }

    pub fn contains(&self, element: T) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] == element {
                return true;
            }
        }
        return false;
    }

    pub fn index_of(&self, element: T) -> Option<usize> {
        for i in 0..self.data.len() {
            if self.data[i] == element {
                return Some(i);
            }
        }
        None
    }

    pub fn remove(&mut self, element: T) -> Option<T> {
        match self.index_of(element) {
            Some(index) => {
                let value = self.data[index];
                self.data[index] = T::default();
                self.count -= 1;
                Some(value)
            }
            None => None,
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            let value = self.data[index];
            self.data[index] = T::default();
            self.count -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn remove_last(&mut self) -> T {
        let index = self.data.len() - 1;
        let value = self.data[index];
        self.data[index] = T::default();
        self.count -= 1;
        value
    }

    fn grow(&mut self, size: usize) {
        let mut new = Vec::with_capacity(size);
        for i in 0..self.data.len() {
            new[i] = self.data[i];
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
        assert!(
            bag.data == Vec::<i32>::with_capacity(5),
            "data not set to vec of given length"
        );
        assert!(bag.length == 5_usize, "length set to given length");
    }
}

use std::fmt::{Debug, Formatter, Result};

/// A dynamic list with circular indexing.
pub struct CircularVec<T>
{
    data: Vec<T>,
}

impl<T> Debug for CircularVec<T>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T> CircularVec<T>
    where T: std::marker::Copy + std::cmp::PartialEq
{
    pub fn new() -> CircularVec<T> {
        CircularVec { data: Vec::new() }
    }

    pub fn wrap_index(&self, i: isize) -> usize {
        let len = self.data.len() as isize; // Risk: mnight overflow for very large indexes...

        if len == 0 {
            return 0;
        }

        if i < 0 {
            // modulo with negative i is weird, make sure i is positive
            return self.wrap_index(i + len);
        } else {
            return (i % len) as usize;
        }
    }

    /// Get the element at `index`. Returns None if the list is empty.
    pub fn get(&self, i: isize) -> Option<&T> {
        let index = self.wrap_index(i);

        self.data.get(index as usize)
    }

    /// Insert a new element at `index` and return the index of the new element.
    pub fn insert(&mut self, i: isize, el: T) -> isize {
        if self.data.len() == 0 {
            self.data.push(el);
            return 0;
        }

        let index = self.wrap_index(i);

        self.data.insert(index, el);

        index as isize
    }

    /// Insert a new element at the end of list.
    pub fn push(&mut self, el: T) {
        self.data.push(el);
    }

    /// Get the element at `index`, remove it from the list and return it.
    pub fn get_and_remove(&mut self, i: isize) -> (isize, T) {
        let index = self.wrap_index(i);

        (index as isize, self.data.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_index() {
        // test is implementation specific!
        let mut list1 = CircularVec::new();
        list1.data.push(1);
        list1.data.push(2);
        list1.data.push(3);

        assert_eq!(list1.wrap_index(0), 0);
        assert_eq!(list1.wrap_index(1), 1);
        assert_eq!(list1.wrap_index(2), 2);
        assert_eq!(list1.wrap_index(3), 0);
        assert_eq!(list1.wrap_index(4), 1);
        assert_eq!(list1.wrap_index(5), 2);
        assert_eq!(list1.wrap_index(6), 0);

        assert_eq!(list1.wrap_index(-1), 2);
        assert_eq!(list1.wrap_index(-2), 1);
        assert_eq!(list1.wrap_index(-3), 0);
        assert_eq!(list1.wrap_index(-4), 2);
        assert_eq!(list1.wrap_index(-5), 1);
        assert_eq!(list1.wrap_index(-6), 0);
    }

    #[test]
    fn test_get() {
        let mut list1: CircularVec<i32> = CircularVec::new();

        assert_eq!(list1.get(-1), None);
        assert_eq!(list1.get( 0), None);
        assert_eq!(list1.get( 1), None);

        list1.data.push(0);
        list1.data.push(1);
        list1.data.push(2);

        assert_eq!(list1.get(-3), Some(&0));
        assert_eq!(list1.get(-2), Some(&1));
        assert_eq!(list1.get(-1), Some(&2));
        assert_eq!(list1.get( 0), Some(&0));
        assert_eq!(list1.get( 1), Some(&1));
        assert_eq!(list1.get( 2), Some(&2));
        assert_eq!(list1.get( 3), Some(&0));
    }

    #[test]
    fn test_insert() {
        let mut list1: CircularVec<i32> = CircularVec::new();

        list1.insert(0, 10);
        list1.insert(1, 11);
        list1.insert(-1, 12);
        list1.insert(3, 13);

        assert_eq!(list1.get(0), Some(&13));
        assert_eq!(list1.get(1), Some(&11));
        assert_eq!(list1.get(2), Some(&12));
        assert_eq!(list1.get(3), Some(&10));
    }

    #[test]
    fn test_get_and_remove() {
        let mut list1: CircularVec<i32> = CircularVec::new();
        list1.data.push(1);
        list1.data.push(2);
        list1.data.push(3);
        list1.data.push(4);
        list1.data.push(5);

        assert_eq!(list1.get_and_remove(2), (2, 3));
        assert_eq!(list1.get_and_remove(-1), (3, 5));
        assert_eq!(list1.get_and_remove(5), (2, 4));

        assert_eq!(list1.get(0), Some(&1));
        assert_eq!(list1.get(1), Some(&2));
    }
}

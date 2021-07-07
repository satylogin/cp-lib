use std::borrow::Borrow;
use std::collections::btree_map;
use std::collections::BTreeMap;

/// MultiSet: This data structure behaves similar to set
/// with the exception that it can have repeated elements.
///
/// Does not allocate anything on its own
///
/// # Examples
///
///```
///#![allow(unused_mut)]
///use cp_lib::ds::multiset::MultiSet;
///
///let mut multiset: MultiSet<String> = MultiSet::new();
///```
#[derive(Debug, Clone)]
pub struct MultiSet<T> {
    freq: BTreeMap<T, usize>,
    len: usize,
}

pub struct Iter<'a, T> {
    iter: btree_map::Iter<'a, T, usize>,
    front: Option<&'a T>,
    front_to_dispatch: usize,
    back: Option<&'a T>,
    back_to_dispatch: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next() {
                self.front = Some(k);
                self.front_to_dispatch = v;
            } else if self.back_to_dispatch > 0 {
                self.back_to_dispatch -= 1;
                return self.back;
            }
        }
        if self.front_to_dispatch > 0 {
            self.front_to_dispatch -= 1;
            return self.front;
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_to_dispatch == 0 {
            if let Some((k, &v)) = self.iter.next_back() {
                self.back = Some(k);
                self.back_to_dispatch = v;
            } else if self.front_to_dispatch > 0 {
                self.front_to_dispatch -= 1;
                return self.front;
            }
        }
        if self.back_to_dispatch > 0 {
            self.back_to_dispatch -= 1;
            return self.back;
        }
        None
    }
}

impl<T> MultiSet<T> {
    pub fn new() -> MultiSet<T>
    where
        T: Ord,
    {
        MultiSet {
            freq: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, val: T)
    where
        T: Ord,
    {
        *self.freq.entry(val).or_insert(0) += 1;
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains<Q: ?Sized>(&self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.freq.contains_key(val)
    }

    /// Removes one occurance of value from multiset.
    /// Returns true if value was present in set and
    /// deleted, and false otherwise.
    pub fn remove<Q: ?Sized>(&mut self, val: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord,
    {
        if self.contains(val) {
            *self.freq.get_mut(val).unwrap() -= 1;
            if self.freq[val] == 0 {
                self.freq.remove(val);
            }
            self.len -= 1;
            return true;
        }
        false
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    where
        T: Ord,
    {
        Iter {
            iter: self.freq.iter(),
            front: None,
            front_to_dispatch: 0,
            back: None,
            back_to_dispatch: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultiSet;

    #[test]
    fn test_initialize() {
        let _: MultiSet<String> = MultiSet::new();
    }

    #[test]
    fn test_insert() {
        let mut m: MultiSet<&str> = MultiSet::new();
        m.insert("a");
        m.insert("B");
        m.insert("a");
        m.insert("c");
        assert_eq!(m.len(), 4);
    }

    #[test]
    fn test_len() {
        let mut m: MultiSet<usize> = MultiSet::new();
        let entries = vec![1, 1, 2, 3, 4, 4, 4, 5];
        for &x in entries.iter() {
            m.insert(x);
        }
        assert_eq!(entries.len(), m.len());
    }

    #[test]
    fn test_contains() {
        let mut m: MultiSet<i32> = MultiSet::new();
        for i in 1..5 {
            m.insert(i);
        }
        for i in 1..5 {
            assert!(m.contains(&i));
        }
        assert!(!m.contains(&5));
    }

    #[test]
    fn test_remove() {
        let mut m: MultiSet<usize> = MultiSet::new();

        assert!(!m.remove(&5));

        m.insert(1);
        m.insert(2);
        m.insert(1);

        assert!(m.remove(&1));
        assert!(m.remove(&1));
        assert!(!m.remove(&1));

        m.insert(3);
        assert!(m.remove(&2));
    }

    #[test]
    fn test_iter() {
        let mut m: MultiSet<usize> = MultiSet::new();
        m.insert(1);
        m.insert(1);
        m.insert(2);
        m.insert(2);
        m.insert(3);
        m.insert(3);

        let mut iter = m.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&3), iter.next_back());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&3), iter.next_back());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&2), iter.next_back());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
}

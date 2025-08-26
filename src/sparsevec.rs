use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

const EMPTY_INDEX: usize = usize::MAX;

/// # SparseVec
/// Implements a dense unordered storage with stable ids.
///
/// ```
/// # use containers::SparseVec;
/// let mut data = SparseVec::<usize>::default();
/// let id_1 = data.insert(5);
/// let id_2 = data.insert(6);
/// let id_3 = data.insert(7);
///
/// data.remove(id_2);
/// assert_eq!(data.get(id_1), Some(&5));
/// assert_eq!(data.get(id_3), Some(&7));
/// assert_eq!(data.as_slice(), &[5, 7]);
/// ```
#[derive(Debug, Clone)]
pub struct SparseVec<T> {
    /// Stores the actual user's data
    data: Vec<T>,
    /// Maps this set's public indices with a slot in `data`.
    positions: Vec<usize>,
    /// Stores removed indices that are available for re-use
    free_indices: Vec<usize>,
}

impl<T> Default for SparseVec<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            positions: Vec::new(),
            free_indices: Vec::new(),
        }
    }
}

impl<T> SparseVec<T> {
    /// Constructs a new empty [`SparseVec`]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the data in this container as a slice. No guarantees on order.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns the data in this container as a mut slice. No guarantees on order.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.positions.clear();
        self.free_indices.clear();
    }

    /// Inserts a new element in the `SparseVec`, returning its index. Might
    /// reuse a previously deleted index.
    ///
    /// ```
    /// # use containers::SparseVec;
    /// let mut data = SparseVec::<usize>::default();
    /// let id_1 = data.insert(5);
    /// let id_2 = data.insert(6);
    /// let id_3 = data.insert(7);
    ///
    /// data.remove(id_2);
    /// assert_eq!(data.get(id_1), Some(&5));
    /// assert_eq!(data.get(id_3), Some(&7));
    /// ```
    pub fn insert(&mut self, value: T) -> usize {
        // Store incremented position.
        let position = self.data.len();

        // Reuse empty space in the positions.
        let index = match self.free_indices.pop() {
            None => {
                self.positions.push(position);
                self.positions.len() - 1
            }
            Some(i) => {
                self.positions[i] = position;
                i
            }
        };

        self.data.push(value);

        index
    }

    /// Removes the element at index `n` from the `SparseVec`, returning it, if
    /// it was at all present.
    ///
    /// Inserts a new element in the `SparseVec`, returning its index. Might
    /// reuse a previously deleted index.
    ///
    /// ```
    /// # use containers::SparseVec;
    /// let mut data = SparseVec::<usize>::default();
    /// let id_1 = data.insert(5);
    /// let id_2 = data.insert(6);
    /// let id_3 = data.insert(7);
    ///
    /// assert_eq!(data.remove(id_2), Some(6));
    /// assert_eq!(data.get(id_2), None);
    /// assert!(!data.contains(id_2));
    /// ```
    pub fn remove(&mut self, n: usize) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let position = self.position(n)?;

        let deleted = if position == self.data.len() - 1 {
            self.data.pop()
        } else {
            let value = self.data.swap_remove(position);
            self.positions[self.data.len()] = self.positions[n];
            self.positions[n] = EMPTY_INDEX;
            Some(value)
        };
        self.free_indices.push(n);

        deleted
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns whether there is an element at the index `n`.
    #[inline]
    pub fn contains(&self, n: usize) -> bool {
        self.position(n).is_some()
    }

    /// Returns the position in `self.data` of the element at index `n`, if any.
    #[inline]
    fn position(&self, n: usize) -> Option<usize> {
        self.positions
            .get(n)
            .and_then(|&p| (p != EMPTY_INDEX).then_some(p))
    }

    /// Returns the position in `self.data` of the element at index `n`, if any.
    #[inline]
    unsafe fn position_unchecked(&self, n: usize) -> usize {
        *self.positions.get_unchecked(n)
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<&T> {
        let position = self.position(n)?;
        self.data.get(position)
    }

    #[inline]
    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        let position = self.position(n)?;
        self.data.get_mut(position)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, n: usize) -> &T {
        let position = self.position_unchecked(n);
        self.data.get_unchecked(position)
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, n: usize) -> &mut T {
        let position = self.position_unchecked(n);
        self.data.get_unchecked_mut(position)
    }

    #[inline]
    pub fn items(&self) -> impl Iterator<Item = (usize, &T)> {
        self.keys().filter_map(|n| Some((n, self.get(n)?)))
    }

    /// Returns all keys of elements of this set
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.positions
            .iter()
            .enumerate()
            .filter_map(|(n, pos)| (*pos != EMPTY_INDEX).then_some(n))
    }

    /// Returns all elements of this set
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns all elements of this set mutably
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T> Index<usize> for SparseVec<T> {
    type Output = T;

    fn index(&self, n: usize) -> &T {
        self.get(n).unwrap()
    }
}

impl<T> IndexMut<usize> for SparseVec<T> {
    fn index_mut(&mut self, n: usize) -> &mut T {
        self.get_mut(n).unwrap()
    }
}

impl<T> Deref for SparseVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::SparseVec;
    use typed_test_gen::test_with;

    #[derive(Clone, Default, Debug)]
    struct Dummy {
        _a: usize,
        _b: String,
        _c: Vec<usize>,
    }

    #[test_with(usize, String, Dummy)]
    fn insertion<T: Default>() {
        let mut set = SparseVec::<T>::new();

        let mut indices = vec![];
        for _ in 0..10 {
            indices.push(set.insert(T::default()));
        }
        assert_eq!(set.len(), 10);

        for idx in indices {
            assert!(set.contains(idx));
        }
        assert!(!set.contains(10));
    }

    #[test_with(usize, String, Dummy)]
    fn remove_first<T: Default>() {
        let mut set = SparseVec::<T>::new();

        let idx = set.insert(T::default());
        assert!(set.remove(idx).is_some());
        assert_eq!(set.len(), 0);
    }

    #[test_with(usize, String, Dummy)]
    fn remove<T: Default>() {
        let mut set = SparseVec::<T>::new();

        let mut indices = vec![];
        for _ in 0..10 {
            indices.push(set.insert(T::default()));
        }
        assert_eq!(indices.len(), 10);

        for idx in indices.iter().take(5) {
            assert!(set.remove(*idx).is_some());
            assert!(set.remove(*idx).is_none());
        }
        for idx in indices.iter().skip(5) {
            assert!(set.contains(*idx));
        }
        assert!(set.remove(20).is_none());
        assert_eq!(set.len(), 5);

        let size = set.positions.len();
        assert!(set.insert(T::default()) < 5);
        assert_eq!(size, set.positions.len());
    }

    #[test_with(usize, String, Dummy)]
    fn remove_non_present<T: Default>() {
        let mut set = SparseVec::<T>::new();
        for _ in 0..10 {
            set.insert(T::default());
        }
        assert_eq!(set.len(), 10);
        assert_eq!(set.data.len(), 10);
        assert_eq!(set.positions.len(), 10);
        assert_eq!(set.free_indices.len(), 0);

        set.remove(20);
        assert_eq!(set.len(), 10);
        assert_eq!(set.data.len(), 10);
        assert_eq!(set.positions.len(), 10);
        assert_eq!(set.free_indices.len(), 0);
    }

    #[test]
    fn get() {
        let mut set = SparseVec::<usize>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0).is_some());
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        *set.get_mut(i1).unwrap() = 10;
        assert_eq!(set.get(i1), Some(&10));
        assert_eq!(set.get(i0), None);
        assert_eq!(set.get_mut(i0), None);
    }

    #[test]
    fn index() {
        let mut set = SparseVec::<usize>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0).is_some());
        assert!(set.contains(i1));
        assert!(!set.contains(i0));
        assert_eq!(set[i1], 1);

        set[i1] = 10;
        assert_eq!(set[i1], 10);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let mut set = SparseVec::<usize>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0).is_some());
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        set[i0] = 10;
    }

    #[test]
    fn get_unchecked() {
        let mut set = SparseVec::<usize>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0).is_some());
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        unsafe {
            *set.get_unchecked_mut(i1) = 10;
            assert_eq!(set.get_unchecked(i1), &10);
        }
    }

    #[test]
    fn iteration() {
        let mut set = SparseVec::<usize>::new();

        let mut indices = vec![];
        for i in 0..10 {
            indices.push(set.insert(i));
        }

        assert_eq!(indices, set.items().map(|t| t.0).collect::<Vec<_>>());
        assert_eq!(indices, set.items().map(|t| *t.1).collect::<Vec<_>>());
        assert_eq!(indices, set.keys().collect::<Vec<_>>());
        assert_eq!(indices, set.values().copied().collect::<Vec<_>>());

        for i in set.values_mut() {
            *i += 1;
        }

        for (n, &m) in indices.iter().zip(set.values()) {
            assert_eq!(n + 1, m)
        }
    }

    #[test]
    fn clear() {
        let mut set = SparseVec::<usize>::new();

        let mut indices = vec![];
        for i in 0..10 {
            indices.push(set.insert(i));
        }

        set.clear();
        assert_eq!(set.len(), 0);

        for i in &indices {
            assert!(!set.contains(*i));
        }

        let mut indices = vec![];
        for i in 0..10 {
            indices.push(set.insert(i));
        }

        assert_eq!(set.len(), 10);

        for i in indices {
            assert!(set.contains(i));
        }
    }

    #[test]
    fn test_deref() {
        let mut set = SparseVec::<usize>::new();

        let mut indices = vec![];
        for i in 0..10 {
            indices.push(set.insert(i));
        }
        let to_remove = indices.remove(0);
        set.remove(to_remove);

        let as_slice: &[usize] = &set;
        assert_eq!(as_slice, &[9, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

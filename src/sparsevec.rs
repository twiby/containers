use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
pub struct SparseVec<T> {
    data: Vec<(usize, T)>,
    positions: Vec<usize>,
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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn data(&self) -> &[(usize, T)] {
        &self.data
    }

    #[inline]
    pub fn positions(&self) -> &[usize] {
        &self.positions
    }

    #[inline]
    pub fn free_indices(&self) -> &[usize] {
        &self.free_indices
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.positions.clear();
        self.free_indices.clear();
    }

    /// Inserts a new element in the `SparseVec`, returning its index.
    pub fn insert(&mut self, value: T) -> usize {
        // Store incremented position (0 is a removed element).
        let position = self.data.len() + 1;

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

        // Hold onto the index such that we can re-link removed entries properly.
        self.data.push((index, value));

        index
    }

    /// Removes the element at index `n` from the `SparseVec`, returning it, if
    /// it was at all present.
    pub fn remove(&mut self, n: usize) -> Option<T> {
        let position = self.position(n)?;

        let deleted = if position == self.data.len() - 1 {
            self.data.pop().map(|(_, value)| value)
        } else {
            let (_, value) = self.data.swap_remove(position);
            let ex_last = self.data[position].0;
            self.positions[ex_last] = self.positions[n];
            self.positions[n] = 0;
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
            .and_then(|&p| if p > 0 { Some(p - 1) } else { None })
    }

    /// Returns the position in `self.data` of the element at index `n`, if any.
    #[inline]
    unsafe fn position_unchecked(&self, n: usize) -> usize {
        self.positions.get_unchecked(n) - 1
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<&T> {
        let position = self.position(n)?;
        Some(&self.data.get(position)?.1)
    }

    #[inline]
    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        let position = self.position(n)?;
        Some(&mut self.data.get_mut(position)?.1)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, n: usize) -> &T {
        let position = self.position_unchecked(n);
        &self.data.get_unchecked(position).1
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, n: usize) -> &mut T {
        let position = self.position_unchecked(n);
        &mut self.data.get_unchecked_mut(position).1
    }

    pub fn items(&self) -> impl Iterator<Item = &(usize, T)> {
        self.data.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.data().iter().map(|(i, _)| *i)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data().iter().map(|(_, val)| val)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, val)| val)
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
        assert_eq!(indices, set.items().map(|t| t.1).collect::<Vec<_>>());
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
}

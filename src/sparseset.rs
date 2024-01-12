use crate::staticvec::StaticVec;
use crate::staticvec::VecLike;

pub trait SparseSetContainers<T> {
    type DataContainer: VecLike<(usize, T)>;
    type PositionsContainer: VecLike<Option<usize>>;
    type FreeIndicesContainer: VecLike<usize>;
}

pub struct DynamicContainerMarker;
impl<T> SparseSetContainers<T> for DynamicContainerMarker {
    type DataContainer = Vec<(usize, T)>;
    type PositionsContainer = Vec<Option<usize>>;
    type FreeIndicesContainer = Vec<usize>;
}

pub struct StaticContainerMarker<const N: usize>;
impl<T: Default, const N: usize> SparseSetContainers<T> for StaticContainerMarker<N> {
    type DataContainer = StaticVec<(usize, T), N>;
    type PositionsContainer = StaticVec<Option<usize>, N>;
    type FreeIndicesContainer = StaticVec<usize, N>;
}

pub type SparseSet<T> = GenericSparseSet<T, DynamicContainerMarker>;
pub type StaticSparseSet<T, const N: usize> = GenericSparseSet<T, StaticContainerMarker<N>>;

#[derive(Debug, Clone)]
pub struct GenericSparseSet<T, Containers>
where
    Containers: SparseSetContainers<T>,
{
    data: Containers::DataContainer,
    positions: Containers::PositionsContainer,
    free_indices: Containers::FreeIndicesContainer,
}

impl<T, Containers> GenericSparseSet<T, Containers>
where
    Containers: SparseSetContainers<T>,
{
    pub fn new() -> Self {
        Self {
            data: Containers::DataContainer::new(),
            positions: Containers::PositionsContainer::new(),
            free_indices: Containers::FreeIndicesContainer::new(),
        }
    }

    /// Inserts a new element in the set, returning its index
    pub fn insert(&mut self, value: T) -> usize {
        let index = match self.free_indices.pop() {
            None => {
                self.positions.push(Some(self.data.len()));
                self.positions.len() - 1
            }
            Some(i) => {
                self.positions[i] = Some(self.data.len());
                i
            }
        };

        self.data.push((index, value));

        index
    }

    /// Removes the element at index `n` from the set, returning whether the element was at all present
    pub fn remove(&mut self, n: usize) -> bool {
        let Some(position) = self.position(n) else {
            return false;
        };

        self.data.swap_remove(position);
        let idx = self.data[position].0;
        self.positions[idx] = Some(position);
        self.positions[n] = None;

        self.free_indices.push(n);
        return true;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether there is an element at the index `n` in the set
    #[inline(always)]
    pub fn contains(&self, n: usize) -> bool {
        self.position(n).is_some()
    }

    /// Returns the position in `self.data` of the element at index `n`, if any
    #[inline(always)]
    fn position(&self, n: usize) -> Option<usize> {
        self.positions.get(n).copied().flatten()
    }

    #[inline(always)]
    pub fn get(&self, n: usize) -> Option<&T> {
        Some(&self.data.get(self.position(n)?)?.1)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        let position = self.position(n)?;
        Some(&mut self.data.get_mut(position)?.1)
    }

    #[inline(always)]
    pub fn get_unchecked(&self, n: usize) -> &T {
        unsafe {
            &self
                .data
                .get_unchecked(self.positions.get_unchecked(n).unwrap())
                .1
        }
    }

    #[inline(always)]
    pub fn get_unchecked_mut(&mut self, n: usize) -> &mut T {
        unsafe {
            let position = self.positions.get_unchecked(n).unwrap();
            &mut self.data.get_unchecked_mut(position).1
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &(usize, T)> {
        self.data.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.items().map(|(i, _)| *i)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.items().map(|(_, val)| val)
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut().map(|(_, val)| val)
    }
}

impl<T, Containers> std::ops::Index<usize> for GenericSparseSet<T, Containers>
where
    Containers: SparseSetContainers<T>,
{
    type Output = T;

    fn index(&self, n: usize) -> &T {
        self.get(n).unwrap()
    }
}

impl<T, Containers> std::ops::IndexMut<usize> for GenericSparseSet<T, Containers>
where
    Containers: SparseSetContainers<T>,
{
    fn index_mut(&mut self, n: usize) -> &mut T {
        self.get_mut(n).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::sparseset::DynamicContainerMarker;
    use crate::sparseset::GenericSparseSet;
    use crate::sparseset::SparseSetContainers;
    use crate::sparseset::StaticContainerMarker;
    use crate::SparseSet;
    use typed_test_gen::test_with;

    #[derive(Clone, Default, Debug)]
    struct Dummy {
        _a: usize,
        _b: String,
        _c: Vec<usize>,
    }

    #[test_with(usize, String, Dummy)]
    fn insertion<T: Default>() {
        let mut set = SparseSet::<T>::new();

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
    fn remove<T: Default>() {
        let mut set = SparseSet::<T>::new();

        let mut indices = vec![];
        for _ in 0..10 {
            indices.push(set.insert(T::default()));
        }
        assert_eq!(indices.len(), 10);

        for idx in indices.iter().take(5) {
            assert!(set.remove(*idx));
            assert!(!set.remove(*idx));
        }
        for idx in indices.iter().skip(5) {
            assert!(set.contains(*idx));
        }
        assert!(!set.remove(20));
        assert_eq!(set.len(), 5);

        let size = set.positions.len();
        assert!(set.insert(T::default()) < 5);
        assert_eq!(size, set.positions.len());
    }

    #[test_with(DynamicContainerMarker, StaticContainerMarker<2>)]
    fn get<T: SparseSetContainers<usize>>() {
        let mut set = GenericSparseSet::<usize, T>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0));
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        *set.get_mut(i1).unwrap() = 10;
        assert_eq!(set.get(i1), Some(&10));
        assert_eq!(set.get(i0), None);
        assert_eq!(set.get_mut(i0), None);
    }

    #[test_with(DynamicContainerMarker, StaticContainerMarker<2>)]
    fn index<T: SparseSetContainers<usize>>() {
        let mut set = GenericSparseSet::<usize, T>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0));
        assert!(set.contains(i1));
        assert!(!set.contains(i0));
        assert_eq!(set[i1], 1);

        set[i1] = 10;
        assert_eq!(set[i1], 10);
    }

    #[test_with(DynamicContainerMarker, StaticContainerMarker<1>)]
    #[should_panic]
    fn index_panic<T: SparseSetContainers<usize>>() {
        let mut set = GenericSparseSet::<usize, T>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0));
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        set[i0] = 10;
    }

    #[test_with(DynamicContainerMarker, StaticContainerMarker<2>)]
    fn get_unchecked<T: SparseSetContainers<usize>>() {
        let mut set = GenericSparseSet::<usize, T>::new();
        let i0 = set.insert(0);
        let i1 = set.insert(1);

        assert!(set.remove(i0));
        assert!(set.contains(i1));
        assert!(!set.contains(i0));

        *set.get_unchecked_mut(i1) = 10;
        assert_eq!(set.get_unchecked(i1), &10);
    }

    #[test_with(DynamicContainerMarker, StaticContainerMarker<20>)]
    fn iteration<T: SparseSetContainers<usize>>() {
        let mut set = GenericSparseSet::<usize, T>::new();

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
}

use std::borrow::Borrow;
use std::collections::hash_map::IterMut;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::RandomState;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::Clear;

/// Wraps a usual [`HashMap`] so that no elements are ever dropped. They are
/// kept alive hidden. When inserting a new element, we can reuse one of the
/// previously deleted one, using the [`Clear`] trait to ensure it's behaving as
/// though just created. This is useful when having a "container of containers"
/// like a `HashMap<Vec<_>>`, and we don't want to lose the allocations of the
/// inner containers, or create catastrophic memory fragmentation.
///
/// In case an element is removed or overwritten, its [`Clear`] implementation
/// is called, so that side effects depending on element "clearing" will happen.
///
/// ```
/// # use containers::RecyclingHashMap;
/// let mut data = RecyclingHashMap::<usize, Vec<usize>>::default();
/// data.insert(0, || vec![1, 2, 3], |_| {});
/// data.insert(1, || vec![1, 2, 3], |_| {});
/// data.insert(2, || vec![1, 2, 3], |_| {});
/// assert_eq!(data.len(), 3);
///
/// data.remove(&0);
/// assert_eq!(data.len(), 2);
/// assert_eq!(data.iter().count(), 2);
/// assert_eq!(data.iter_mut().count(), 2);
///
/// let new_element = data.insert(0, || vec![1,2,3,4,5,6], |_| {});
/// assert_eq!(new_element.len(), 0);
/// assert!(new_element.capacity() >= 3);
///
/// data.remove(&0);
/// let new_element = data.insert_default(0); // default constructor is available
/// assert_eq!(new_element.len(), 0);
/// assert!(new_element.capacity() >= 3);
/// ```
///
/// # Note
/// If you want to use this, you probably actually need a smart memory
/// management scheme implemented as a custom allocator. This is just a useful
/// tool that can be deployed very quickly without all the fuss of an allocator.
pub struct RecyclingHashMap<K, V: Clear, Hasher = RandomState> {
    map: HashMap<K, V, Hasher>,
    dead: Vec<V>,
}

impl<K, V: Clear, Hasher> std::fmt::Debug for RecyclingHashMap<K, V, Hasher>
where
    HashMap<K, V, Hasher>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}

impl<K, V: Clear, Hasher> Deref for RecyclingHashMap<K, V, Hasher> {
    type Target = HashMap<K, V, Hasher>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V: Clear, Hasher> Clone for RecyclingHashMap<K, V, Hasher>
where
    HashMap<K, V, Hasher>: Clone,
{
    /// Clones only alive elements, those counted in `Self::len`
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            dead: vec![],
        }
    }
}

impl<K, V: Clear, Hasher> Default for RecyclingHashMap<K, V, Hasher>
where
    HashMap<K, V, Hasher>: Default,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            dead: vec![],
        }
    }
}

impl<K, V: Clear, Hasher> RecyclingHashMap<K, V, Hasher>
where
    V: Default,
    K: Eq + Hash,
    Hasher: BuildHasher,
{
    /// Works like [`RecyclingHashMap::insert`], using a default
    /// constructor if needed. Notably, the default constructor is not
    /// called if a previously deleted element is available, or an element
    /// already exist at that key.
    pub fn insert_default(&mut self, key: K) -> &mut V {
        self.insert(key, Default::default, |_| {})
    }
}

impl<K, V: Clear, Hasher> RecyclingHashMap<K, V, Hasher> {
    /// Calls the [`Clear`] trait on every element and remove them from the map.
    ///
    /// ```
    /// # use containers::RecyclingHashMap;
    /// let mut data = RecyclingHashMap::<usize, Vec<usize>>::default();
    /// data.insert_default(0);
    /// data.insert_default(1);
    /// data.insert_default(2);
    /// assert_eq!(data.len(), 3);
    ///
    /// data.clear();
    /// assert_eq!(data.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        for (_, mut v) in self.map.drain() {
            v.clear();
            self.dead.push(v);
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.map.iter_mut()
    }

    /// Works like [`HashMap::values_mut`]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.map.values_mut()
    }
}

impl<K, V, Hasher> RecyclingHashMap<K, V, Hasher>
where
    V: Clear,
    K: Eq + Hash,
    Hasher: BuildHasher,
{
    /// If an entry already exist for this key, it is cleared, and `init` is
    /// applied. If a previously deleted element can be recycled, apply
    /// `init`. Otherwise, construct a new_element using `ctor`.
    ///
    /// The new element is added to the map and a mutable reference to it is
    /// returned
    ///
    /// ```
    /// # use containers::RecyclingHashMap;
    /// let mut data = RecyclingHashMap::<usize, Vec<usize>>::default();
    /// data.insert_default(0);
    /// data.remove(&0);
    ///
    /// data.insert(1, || vec![0], |val| val.push(1));
    /// assert_eq!(data[&1], vec![1]);
    /// data.insert(1, || vec![0], |val| val.push(1));
    /// assert_eq!(data[&1], vec![1]);
    /// data.insert(10, || vec![0], |val| val.push(1));
    /// assert_eq!(data[&10], vec![0]);
    /// ```
    pub fn insert(
        &mut self,
        key: K,
        ctor: impl FnOnce() -> V,
        init: impl FnOnce(&mut V),
    ) -> &mut V {
        use std::collections::hash_map;

        match self.map.entry(key) {
            hash_map::Entry::Occupied(entry) => {
                let val = entry.into_mut();
                val.clear();
                init(val);
                val
            }
            hash_map::Entry::Vacant(entry) => entry.insert(match self.dead.pop() {
                Some(mut el) => {
                    init(&mut el);
                    el
                }
                None => ctor(),
            }),
        }
    }

    /// Gets the given key’s corresponding entry in the map for in-place
    /// manipulation.
    ///
    /// ```
    /// # use std::collections::hash_map::HashMap;
    /// # use containers::RecyclingHashMap;
    /// let composers = HashMap::<&str, &str>::from_iter([
    ///    ("Mozart", "Classical"),
    ///    ("JimiHendrix", "Rock"),
    ///    ("Eric Clapton", "Rock")
    /// ].into_iter());
    ///
    /// let mut styles = RecyclingHashMap::<&str, Vec<&str>>::default();
    /// for (composer, style) in composers.iter() {
    ///     styles.entry(style).or_insert_default().push(composer);
    /// }
    ///
    /// assert_eq!(styles.len(), 2);
    /// assert_eq!(styles["Classical"].len(), 1);
    /// assert_eq!(styles["Rock"].len(), 2);
    /// ```
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        use std::collections::hash_map;

        match self.map.entry(key) {
            hash_map::Entry::Occupied(entry) => Entry::Occupied(entry),
            hash_map::Entry::Vacant(entry) => Entry::Vacant(entry, &mut self.dead),
        }
    }

    /// If the container contains an element at the given key, remove it from
    /// the map, call [`Clear`] on it, then keep it in a `dead` element list to
    /// reuse it later
    pub fn remove<Q>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some(mut dead) = self.map.remove(key) {
            dead.clear();
            self.dead.push(dead);
        }
    }

    /// Works like [`HashMap::get_mut`]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_mut(key)
    }
}

impl<K, V, Hasher, Q> Index<Q> for RecyclingHashMap<K, V, Hasher>
where
    V: Clear,
    HashMap<K, V, Hasher>: Index<Q>,
{
    type Output = <HashMap<K, V, Hasher> as Index<Q>>::Output;
    fn index(&self, key: Q) -> &Self::Output {
        &self.map[key]
    }
}

impl<K, V, Hasher, Q> IndexMut<Q> for RecyclingHashMap<K, V, Hasher>
where
    V: Clear,
    HashMap<K, V, Hasher>: Index<Q> + IndexMut<Q>,
{
    fn index_mut(&mut self, k: Q) -> &mut Self::Output {
        &mut self.map[k]
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V, Hasher> IntoParallelIterator for &'data RecyclingHashMap<K, V, Hasher>
where
    V: Clear,
    for<'d> &'d HashMap<K, V, Hasher>: IntoParallelIterator,
{
    type Iter = <&'data HashMap<K, V, Hasher> as IntoParallelIterator>::Iter;
    type Item = <&'data HashMap<K, V, Hasher> as IntoParallelIterator>::Item;
    fn into_par_iter(self) -> Self::Iter {
        self.map.par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'data, K, V, Hasher> IntoParallelIterator for &'data mut RecyclingHashMap<K, V, Hasher>
where
    V: Clear,
    for<'d> &'d mut HashMap<K, V, Hasher>: IntoParallelIterator,
{
    type Iter = <&'data mut HashMap<K, V, Hasher> as IntoParallelIterator>::Iter;
    type Item = <&'data mut HashMap<K, V, Hasher> as IntoParallelIterator>::Item;
    fn into_par_iter(self) -> Self::Iter {
        self.map.par_iter_mut()
    }
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>, &'a mut Vec<V>),
}

impl<'a, K, V: Clear> Entry<'a, K, V> {
    /// Works like [`std::collections::hash_map::Entry::and_modify`]
    pub fn and_modify(mut self, f: impl FnOnce(&mut V)) -> Self {
        if let Self::Occupied(ref mut entry) = self {
            f(entry.get_mut())
        };
        self
    }

    /// If the entry is vacant, fill it with a recycled element (with `init`
    /// applied to it), or if none exist, with the `ctor`.
    ///
    /// ```
    /// # use std::collections::hash_map::HashMap;
    /// # use containers::RecyclingHashMap;
    /// let mut data = RecyclingHashMap::<usize, Vec<usize>>::default();
    /// data.insert(0, || vec![0], |vec| vec.push(0));
    /// data.remove(&0);
    ///
    /// data.insert(1, || vec![1], |vec| vec.push(10));
    /// data.insert(2, || vec![2], |vec| vec.push(20));
    /// assert_eq!(data[&1], vec![10]);
    /// assert_eq!(data[&2], vec![2]);
    /// ```
    pub fn or_insert(self, ctor: impl FnOnce() -> V, init: impl FnOnce(&mut V)) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry, dead) => match dead.pop() {
                Some(mut el) => {
                    init(&mut el);
                    entry.insert(el)
                }
                None => entry.insert(ctor()),
            },
        }
    }
}

impl<'a, K, V: Clear + Default> Entry<'a, K, V> {
    /// If no element was recycled, insert a new default element.
    ///
    /// Note: to maintain coherence, it is strongly advised that
    /// [`Default::default`] construct an object in an identical state as the
    /// state in which an object is left after calling [`Clear::clear`].
    pub fn or_insert_default(self) -> &'a mut V {
        self.or_insert(Default::default, |_| {})
    }
}

#[cfg(test)]
mod test {
    use crate::Clear;
    use crate::RecyclingHashMap;

    struct SomeData {
        alive: bool,
    }

    impl Default for SomeData {
        fn default() -> Self {
            Self { alive: true }
        }
    }

    impl Clear for SomeData {
        fn clear(&mut self) {
            self.alive = false;
        }
    }

    #[test]
    fn insert() {
        // 3 cases:
        //  - 1. Element already exist at key
        //  - 2. Dead element available
        //  - 3. Constructor called

        let ctor = || 10;

        let mut data = RecyclingHashMap::<usize, usize>::default();

        // case 3
        let new_element = data.insert(0, ctor, |_| {});
        assert_eq!(*new_element, 10);

        // case 1
        let new_element = data.insert(0, ctor, |_| {});
        assert_eq!(*new_element, 0);
        *new_element = 10;

        // case 2
        data.remove(&0);
        let new_element = data.insert(0, ctor, |_| {});
        assert_eq!(*new_element, 0);

        // case 3
        let new_element = data.insert(1, ctor, |_| {});
        assert_eq!(*new_element, 10);
    }

    #[test]
    fn remove() {
        let ctor = || 10;

        let mut data = RecyclingHashMap::<usize, usize>::default();

        let new_element = data.insert(0, ctor, |_| {});
        assert_eq!(*new_element, 10);

        data.remove(&0);
        assert_eq!(data.len(), 0);
        assert_eq!(data.map.len(), 0);
        assert_eq!(data.dead.len(), 1);
        assert_eq!(data.dead[0], 0);
    }

    #[test]
    fn direct_insert() {
        let mut data = RecyclingHashMap::<usize, Vec<usize>>::default();
        data.insert(0, || vec![0], |_| {});
        data.insert_default(1);
        data.insert_default(2);
        data.remove(&0);
        assert_eq!(data.len(), 2);
        assert_eq!(data.dead.len(), 1);

        data.insert(0, || vec![1, 2, 3], |v| v.push(1));
        assert_eq!(data.len(), 3);
        assert_eq!(data[&0], vec![1]);
        assert_eq!(data.dead.len(), 0);

        data.insert(0, || vec![1, 2, 3], |v| v.push(1));
        assert_eq!(data.len(), 3);
        assert_eq!(data[&0], vec![1]);
        assert_eq!(data.dead.len(), 0);

        data.insert(10, || vec![1, 2, 3], |v| v.push(1));
        assert_eq!(data.len(), 4);
        assert_eq!(data[&10], vec![1, 2, 3]);
        assert_eq!(data.dead.len(), 0);
    }
}

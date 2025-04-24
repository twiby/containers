use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;

use crate::Clear;

/// Wraps a usual [`Vec`] so that no elements are ever dropped. They are kept
/// alive hidden. When pushing a new element, we can reuse one of the previously
/// deleted one, using the [`Clear`] trait to ensure it's behaving as though
/// just created. This is useful when having a "container of containers" like a
/// `Vec<HashSet<_>>`, and we don't want to lose the allocations of the inner
/// containers, or create catastrophic memory fragmentation.
///
/// In case an element is removed, its [`Clear`] implementation is called, so
/// that side effects depending on element "clearing" will happen.
///
/// ```
/// # use containers::RecyclingVec;
/// let mut data = RecyclingVec::<Vec<usize>>::default();
/// data.push(|| vec![1, 2, 3], |_| {});
/// data.push(|| vec![1, 2, 3], |_| {});
/// data.push(|| vec![1, 2, 3], |_| {});
/// assert_eq!(data.len(), 3);
///
/// data.pop();
/// assert_eq!(data.len(), 2);
/// assert_eq!(data.iter().count(), 2);
/// assert_eq!(data.iter_mut().count(), 2);
///
/// let new_element = data.push(|| vec![1,2,3,4,5,6], |_| {});
/// assert_eq!(new_element.len(), 0);
/// assert!(new_element.capacity() >= 3);
///
/// data.pop();
/// let new_element = data.push_default(); // default constructor is available
/// assert_eq!(new_element.len(), 0);
/// assert!(new_element.capacity() >= 3);
/// ```
///
/// # Note
/// If you want to use this, you probably actually need a smart memory
/// management scheme implemented as a custom allocator. This is just a useful
/// tool that can be deployed very quickly without all the fuss of an allocator.
pub struct RecyclingVec<T: Clear> {
    vec: Vec<T>,
    dead: Vec<T>,
}

impl<T: Clear> std::fmt::Debug for RecyclingVec<T>
where
    Vec<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec.fmt(f)
    }
}

impl<T: Clear + Clone> Clone for RecyclingVec<T> {
    /// Clones only alive elements, those counted in `len`.
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            dead: vec![],
        }
    }
}

impl<T: Clear + Default> RecyclingVec<T> {
    /// If no previously deleted elements can be reused, create a new element
    /// using the default constructor. Otherwise resuse a previously
    /// deleted element, without calling [`Default::default`].
    ///
    /// Note: to maintain coherence, it is strongly advised that
    /// [`Default::default`] construct an object in an identical state as the
    /// state in which an object is left after calling [`Clear::clear`].
    pub fn push_default(&mut self) -> &mut T {
        self.push(Default::default, |_| {})
    }
}

impl<T: Clear> Default for RecyclingVec<T> {
    fn default() -> Self {
        Self {
            vec: vec![],
            dead: vec![],
        }
    }
}

impl<T: Clear> RecyclingVec<T> {
    /// Calls the [`Clear`] trait on every element, and sets the current length
    /// to 0.
    ///
    /// ```
    /// # use containers::RecyclingVec;
    /// let mut data = RecyclingVec::<usize>::default();
    /// data.push_default();
    /// data.push_default();
    /// data.push_default();
    /// assert_eq!(data.len(), 3);
    ///
    /// data.clear();
    /// assert_eq!(data.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.dead.extend(self.vec.drain(..).map(|mut el| {
            el.clear();
            el
        }))
    }

    /// If a previously deleted element can be recycled, apply `init`.
    /// Otherwise, construct a new_element using `ctor`.
    ///
    /// The new element is added to the stack and a mutable reference to it is
    /// returned
    ///
    /// ```
    /// # use containers::RecyclingVec;
    /// # use std::ops::Deref;
    /// let mut data = RecyclingVec::<usize>::default();
    /// data.push_default();
    /// data.pop();
    ///
    /// data.push(|| 1, |val| *val = 2);
    /// data.push(|| 1, |val| *val = 2);
    /// assert_eq!(data.deref(), &[2, 1]);
    /// ```
    pub fn push(&mut self, ctor: impl FnOnce() -> T, init: impl FnOnce(&mut T)) -> &mut T {
        let new_element = if let Some(mut el) = self.dead.pop() {
            init(&mut el);
            el
        } else {
            ctor()
        };

        self.vec.push(new_element);
        self.vec.last_mut().unwrap()
    }

    /// Tries to add an element by recycling a previously deleted one. Returns a
    /// view into that element, that's able to push a new one if no element was
    /// actually recycled.
    ///
    /// ```
    /// # use containers::RecyclingVec;
    /// let mut data = RecyclingVec::<usize>::default();
    /// data.push(|| 10, |_| {});
    /// data.pop();
    ///
    /// // Recyle element, no initialization
    /// let el = data.push_recycled(|_| {});
    /// assert_eq!(el, Some(&mut 0));
    ///
    /// // Recyle element, with initialization
    /// data.pop();
    /// let el = data.push_recycled(|el| *el = 5);
    /// assert_eq!(el, Some(&mut 5));
    ///
    /// // Can't recyle element
    /// let el = data.push_recycled(|el| *el = 5);
    /// assert_eq!(el, None);
    /// ```
    pub fn push_recycled(&mut self, init: impl FnOnce(&mut T)) -> Option<&mut T> {
        let mut recycled = self.dead.pop()?;
        init(&mut recycled);
        self.vec.push(recycled);
        self.vec.last_mut()
    }

    /// If the container has at least one element, calls [`Clear`] on it, then
    /// decrease the length of the container. Otherwise, do nothing.
    ///
    /// Contrary to other container, ownership is kept of deleted elements, so
    /// nothing is returned.
    pub fn pop(&mut self) {
        if let Some(mut dead) = self.vec.pop() {
            dead.clear();
            self.dead.push(dead);
        }
    }

    /// # Panic
    /// panics is `i` is greater than or equal to `self.len()`
    ///
    /// ```
    /// # use containers::RecyclingVec;
    /// let mut data = RecyclingVec::<Vec<usize>>::default();
    /// data.push(|| vec![1, 2, 3], |_| {});
    /// data.push(|| vec![4, 5, 6], |_| {});
    /// data.push(|| vec![7, 8, 9], |_| {});
    /// data.swap_remove(0);
    ///
    /// assert_eq!(data[0], vec![7, 8, 9]);
    /// assert_eq!(data[1], vec![4, 5, 6]);
    /// ```
    pub fn swap_remove(&mut self, i: usize) {
        assert!(i < self.vec.len());
        let last_id = self.vec.len() - 1;
        self.swap(i, last_id);
        self.pop();
    }
}

impl<T: Clear> Deref for RecyclingVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.vec
    }
}

impl<T: Clear> DerefMut for RecyclingVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vec
    }
}

impl<I, T> Index<I> for RecyclingVec<T>
where
    T: Clear,
    Vec<T>: Index<I>,
{
    type Output = <Vec<T> as Index<I>>::Output;
    fn index(&self, n: I) -> &Self::Output {
        &self.vec[n]
    }
}

impl<I, T> IndexMut<I> for RecyclingVec<T>
where
    T: Clear,
    Vec<T>: IndexMut<I>,
{
    fn index_mut(&mut self, n: I) -> &mut Self::Output {
        &mut self.vec[n]
    }
}

impl<'a, T: Clear> IntoIterator for &'a RecyclingVec<T>
where
    &'a Vec<T>: IntoIterator,
{
    type Item = <&'a Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        <&'a Vec<T> as IntoIterator>::into_iter(&self.vec)
    }
}

impl<'a, T: Clear> IntoIterator for &'a mut RecyclingVec<T>
where
    &'a mut Vec<T>: IntoIterator,
{
    type Item = <&'a mut Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        <&'a mut Vec<T> as IntoIterator>::into_iter(&mut self.vec)
    }
}

#[cfg(test)]
mod test {
    use crate::Clear;
    use crate::RecyclingVec;

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
    fn clear_is_called_at_deletion_time() {
        let mut data = RecyclingVec::<SomeData>::default();
        let element = data.push_default();
        assert!(element.alive);

        data.pop();
        assert_eq!(data.vec.len(), 0);
        assert_eq!(data.dead.len(), 1);
        assert!(!data.dead.last().unwrap().alive);
    }

    #[test]
    fn clear_does_not_delete_and_calls_clear() {
        let mut data = RecyclingVec::<SomeData>::default();
        let element = data.push_default();
        assert!(element.alive);

        data.clear();
        assert_eq!(data.vec.len(), 0);
        assert_eq!(data.dead.len(), 1);
        assert!(!data.dead[0].alive);
    }

    #[test]
    fn clone_only_uses_alive_elements() {
        let mut data = RecyclingVec::<usize>::default();
        data.push_default();
        data.push_default();
        data.push_default();
        data.pop();
        assert_eq!(data.len(), 2);
        assert_eq!(data.vec.len(), 2);
        assert_eq!(data.dead.len(), 1);

        let cloned = data.clone();
        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned.vec.len(), 2);
        assert_eq!(cloned.dead.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut data = RecyclingVec::<usize>::default();
        data.push_default();
        data.push_default();
        data.push_default();
        data.pop();
        assert_eq!(data.len(), 2);
        assert_eq!(data.vec.len(), 2);
        assert_eq!(data.dead.len(), 1);

        data.push(|| 1, |val| *val = 2);
        assert_eq!(data.len(), 3);
        assert_eq!(data.vec.len(), 3);
        assert_eq!(data.dead.len(), 0);
        assert_eq!(data.vec, &[0, 0, 2]);

        data.push(|| 1, |val| *val = 2);
        assert_eq!(data.len(), 4);
        assert_eq!(data.vec.len(), 4);
        assert_eq!(data.dead.len(), 0);
        assert_eq!(data.vec, &[0, 0, 2, 1]);
    }
}

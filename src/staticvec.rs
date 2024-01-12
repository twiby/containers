use std::ops::Index;
use std::ops::IndexMut;

pub trait VecLike<T>: Index<usize, Output = T> + IndexMut<usize> {
    fn new() -> Self;
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn swap_remove(&mut self, n: usize) -> T;
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, T>;
    fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T>;
    fn get(&self, n: usize) -> Option<&T>;
    fn get_mut(&mut self, n: usize) -> Option<&mut T>;
    unsafe fn get_unchecked(&self, n: usize) -> &T;
    unsafe fn get_unchecked_mut(&mut self, n: usize) -> &mut T;
}

impl<T> VecLike<T> for Vec<T> {
    fn new() -> Self {
        Self::new()
    }
    fn push(&mut self, val: T) {
        self.push(val)
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn swap_remove(&mut self, n: usize) -> T {
        self.swap_remove(n)
    }
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.as_slice().iter()
    }
    fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.as_mut_slice().iter_mut()
    }
    fn get(&self, n: usize) -> Option<&T> {
        self.as_slice().get(n)
    }
    fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(n)
    }
    unsafe fn get_unchecked(&self, n: usize) -> &T {
        self.as_slice().get_unchecked(n)
    }
    unsafe fn get_unchecked_mut(&mut self, n: usize) -> &mut T {
        self.as_mut_slice().get_unchecked_mut(n)
    }
}

#[derive(Clone, Debug)]
pub struct StaticVec<T, const N: usize> {
    arr: [T; N],
    len: usize,
}

impl<T: Default, const N: usize> VecLike<T> for StaticVec<T, N> {
    fn new() -> Self {
        Self {
            arr: [0; N].map(|_| T::default()),
            len: 0,
        }
    }
    fn push(&mut self, val: T) {
        self.arr[self.len] = val;
        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        self.len -= 1;
        let mut default = T::default();
        std::mem::swap(&mut self.arr[self.len], &mut default);
        Some(default)
    }
    fn len(&self) -> usize {
        self.len
    }
    fn swap_remove(&mut self, n: usize) -> T {
        self.arr.swap(self.len - 1, n);
        self.pop().unwrap()
    }
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.arr[..self.len].iter()
    }
    fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.arr[..self.len].iter_mut()
    }
    fn get(&self, n: usize) -> Option<&T> {
        self.arr.get(n)
    }
    fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.arr.get_mut(n)
    }
    unsafe fn get_unchecked(&self, n: usize) -> &T {
        self.arr.get_unchecked(n)
    }
    unsafe fn get_unchecked_mut(&mut self, n: usize) -> &mut T {
        self.arr.get_unchecked_mut(n)
    }
}

impl<T, const N: usize> Index<usize> for StaticVec<T, N> {
    type Output = T;
    fn index(&self, n: usize) -> &T {
        &self.arr[n]
    }
}
impl<T, const N: usize> IndexMut<usize> for StaticVec<T, N> {
    fn index_mut(&mut self, n: usize) -> &mut T {
        &mut self.arr[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::staticvec::StaticVec;
    use crate::staticvec::VecLike;
    use typed_test_gen::test_with;

    #[test]
    fn staticvec() {
        let mut vec = StaticVec::<usize, 5>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        for (n, &m) in vec.iter().enumerate() {
            assert_eq!(n, m);
        }

        assert_eq!(vec.pop(), Some(4));
        for (n, &m) in vec.iter().enumerate() {
            assert_eq!(n, m);
        }
        assert_eq!(vec.pop(), Some(3));
        for (n, &m) in vec.iter().enumerate() {
            assert_eq!(n, m);
        }
        assert_eq!(vec.pop(), Some(2));
        for (n, &m) in vec.iter().enumerate() {
            assert_eq!(n, m);
        }
        assert_eq!(vec.pop(), Some(1));
        for (n, &m) in vec.iter().enumerate() {
            assert_eq!(n, m);
        }

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.pop(), Some(0));
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.pop(), None);
    }

    #[test_with(Vec<usize>, usize)]
    #[should_panic]
    fn staticvec_fail<T: Default>() {
        let mut vec = StaticVec::<T, 1>::new();
        vec.push(T::default());
        vec.push(T::default());
    }
}

use std::ops::Index;
use std::ops::IndexMut;

pub trait VecLike<T>: Index<usize, Output = T> + IndexMut<usize> {
    fn new() -> Self;
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn swap_remove(&mut self, n: usize) -> T;
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
        let mut default = T::default();
        std::mem::swap(&mut self.arr[self.len - 1], &mut default);
        Some(default)
    }
    fn len(&self) -> usize {
        self.len
    }
    fn swap_remove(&mut self, n: usize) -> T {
        self.arr.swap(self.len - 1, n);
        self.pop().unwrap()
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

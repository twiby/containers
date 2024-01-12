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

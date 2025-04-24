//! This module is meant to be used in conjonction with "recycling containers".
//! The `Clear` trait is an analog to `Drop`, but doesn't delete the object, or
//! free allocations: it just resets the object to a pristine state.

use std::collections::HashMap;
use std::collections::HashSet;

use crate::recycling::RecyclingHashMap;
use crate::RecyclingVec;
use crate::SparseVec;

/// Implemented by types that want to communicate their state can be reset to a
/// default state, without giving up their internal allocation.
pub trait Clear {
    fn clear(&mut self);
}

impl Clear for usize {
    fn clear(&mut self) {
        *self = 0;
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Clear for SparseVec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T, Hasher> Clear for HashSet<T, Hasher> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V, Hasher> Clear for HashMap<K, V, Hasher> {
    fn clear(&mut self) {
        self.clear();
    }
}

/// RecyclingVec could be inside another no drop container!
impl<T: Clear> Clear for RecyclingVec<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

/// RecyclingHashMap could be inside another no drop container!
impl<K, V: Clear, Hasher> Clear for RecyclingHashMap<K, V, Hasher> {
    fn clear(&mut self) {
        self.clear()
    }
}

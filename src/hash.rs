use std::cell::Cell;
use std::hash::BuildHasher;
use std::hash::Hasher;

use rustc_hash::FxHasher;

/// A hash set that uses FxHasher with the `FxRandomState` initializer.
pub type HashSet<V> = std::collections::HashSet<V, FastHashState>;
/// A hash map that uses FxHasher with the `FxRandomState` initializer.
pub type HashMap<K, V> = std::collections::HashMap<K, V, FastHashState>;

pub type RecyclingHashMap<K, V> = crate::recycling::RecyclingHashMap<K, V, FastHashState>;

thread_local! {
    /// Seed shared by all `FastHashState` instances in a single thread.
    static SEED: Cell<usize> = const { Cell::new(0) };
}

/// This is a `FxHasher` initializer that hashes a number at the beginning, to
/// generate different hashes from use to use.
#[derive(Copy, Clone, Debug)]
pub struct FastHashState(usize);

impl FastHashState {
    /// Reset the thread-local seed to the initial value. This is intended to be
    /// used from test code only.
    #[cfg(test)]
    pub fn reset_seed() {
        SEED.set(0);
    }
}

impl BuildHasher for FastHashState {
    type Hasher = FxHasher;

    fn build_hasher(&self) -> FxHasher {
        let mut hasher = FxHasher::default();
        hasher.write_usize(self.0);
        hasher
    }
}

impl Default for FastHashState {
    fn default() -> Self {
        let seed = SEED.get() + 1;
        SEED.set(seed);
        FastHashState(seed)
    }
}

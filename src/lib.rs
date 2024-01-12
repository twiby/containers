pub fn dummy() -> String {
    "hello world".to_string()
}

mod sparseset;
mod staticvec;

pub use sparseset::{SparseSet, StaticSparseSet};

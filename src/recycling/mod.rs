//! "recycling container" are container types that do not actually `Drop`
//! elements when deleted. Elements are kept around to preserve allocations -
//! when removed from a container, they are instead "cleared", meaning their
//! state is reset to a pristine state without deallocating. This behavior is
//! implementable via the `Clear` trait.

mod clear;
pub mod recycling_hash_map;
pub mod recycling_vec;

pub use clear::Clear;
pub use recycling_hash_map::RecyclingHashMap;
pub use recycling_vec::RecyclingVec;

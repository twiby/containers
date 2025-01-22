use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeMap, HashMap, HashSet};

use containers::SparseVec;

pub fn vec_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "vec insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..N {
                v.push(black_box(i));
            }
        })
    });
}

pub fn hash_set_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "hash set insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = HashSet::new();
            for i in 0..N {
                v.insert(black_box(i));
            }
        })
    });
}

pub fn hash_map_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "hash map insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = HashMap::<usize, usize>::new();
            for i in 0..N {
                v.insert(black_box(i), black_box(i));
            }
        })
    });
}

pub fn btree_map_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "btree map insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = BTreeMap::<usize, usize>::new();
            for i in 0..N {
                v.insert(black_box(i), black_box(i));
            }
        })
    });
}

pub fn sparseset_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "sparseset insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = SparseVec::new();
            for i in 0..N {
                v.insert(black_box(i));
            }
        })
    });
}

criterion_group!(
    insertion,
    vec_insertion<100_000>,
    hash_set_insertion<100_000>,
    hash_map_insertion<100_000>,
    btree_map_insertion<100_000>,
    sparseset_insertion<100_000>
);
criterion_main!(insertion);

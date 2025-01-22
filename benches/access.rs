use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeMap, HashMap};

use containers::SparseVec;

pub fn vec_access<const N: usize>(c: &mut Criterion) {
    let mut name = "vec access ".to_string();
    name.push_str(&N.to_string());

    let mut vec = Vec::new();
    for i in 0..N {
        vec.push(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(vec[black_box(mid)]);
        })
    });
}

pub fn hash_map_access<const N: usize>(c: &mut Criterion) {
    let mut name = "hash map access ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map[&black_box(mid)]);
        })
    });
}

pub fn btree_map_access<const N: usize>(c: &mut Criterion) {
    let mut name = "btree map access ".to_string();
    name.push_str(&N.to_string());

    let mut map = BTreeMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map[&black_box(mid)]);
        })
    });
}

pub fn sparseset_access<const N: usize>(c: &mut Criterion) {
    let mut name = "sparseset access ".to_string();
    name.push_str(&N.to_string());

    let mut set = SparseVec::new();
    for i in 0..N {
        set.insert(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(set[black_box(mid)]);
        })
    });
}

criterion_group!(
    access,
    vec_access<100_000>,
    hash_map_access<100_000>,
    btree_map_access<100_000>,
    sparseset_access<100_000>
);
criterion_main!(access);

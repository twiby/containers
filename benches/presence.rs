use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeMap, HashMap, HashSet};

use containers::SparseSet;

pub fn vec_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "vec presence ".to_string();
    name.push_str(&N.to_string());

    let mut v = Vec::new();
    for i in 0..N {
        v.push(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(v.contains(black_box(&mid)));
        })
    });
}

pub fn set_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "set presence ".to_string();
    name.push_str(&N.to_string());

    let mut set = HashSet::new();
    for i in 0..N {
        set.insert(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(set.contains(black_box(&mid)));
        })
    });
}

pub fn hash_map_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "hash map presence ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map.contains_key(&black_box(mid)));
        })
    });
}

pub fn btree_map_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "btree map presence ".to_string();
    name.push_str(&N.to_string());

    let mut map = BTreeMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map.contains_key(&black_box(mid)));
        })
    });
}

pub fn sparseset_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "sparseset presence ".to_string();
    name.push_str(&N.to_string());

    let mut set = SparseSet::new();
    for i in 0..N {
        set.insert(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(set.contains(black_box(mid)));
        })
    });
}

criterion_group!(
    presence,
    vec_presence<100_000>,
    set_presence<100_000>,
    hash_map_presence<100_000>,
    btree_map_presence<100_000>,
    sparseset_presence<100_000>
);
criterion_main!(presence);

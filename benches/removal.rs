use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

use containers::SparseVec;

pub fn vec_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "vec removal ".to_string();
    name.push_str(&N.to_string());

    let mut v = Vec::new();
    for i in 0..N {
        v.push(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                v.push(mid);
                let start = Instant::now();
                black_box(v.remove(black_box(mid)));
                ret += start.elapsed();
            }
            ret
        })
    });
}

pub fn hash_set_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "hash set removal ".to_string();
    name.push_str(&N.to_string());

    let mut set = HashSet::new();
    for i in 0..N {
        set.insert(i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                set.insert(mid);
                let start = Instant::now();
                black_box(set.remove(&black_box(mid)));
                ret += start.elapsed();
            }
            ret
        })
    });
}

pub fn hash_map_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "hash map removal ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                map.insert(mid, mid);
                let start = Instant::now();
                black_box(map.remove(&black_box(mid)));
                ret += start.elapsed();
            }
            ret
        })
    });
}

pub fn btree_map_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "btree map removal ".to_string();
    name.push_str(&N.to_string());

    let mut map = BTreeMap::new();
    for i in 0..N {
        map.insert(i, i);
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                map.insert(mid, mid);
                let start = Instant::now();
                black_box(map.remove(&black_box(mid)));
                ret += start.elapsed();
            }
            ret
        })
    });
}

pub fn sparseset_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "sparseset removal ".to_string();
    name.push_str(&N.to_string());

    let mut set = SparseVec::new();
    let mut indices = VecDeque::new();
    for i in 0..N {
        indices.push_back(set.insert(i));
    }
    let mid = N >> 1;

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                indices.push_back(set.insert(mid));
                let to_remove = indices.pop_front().unwrap();
                let start = Instant::now();
                black_box(set.remove(black_box(to_remove)));
                ret += start.elapsed();
            }
            ret
        })
    });
}

criterion_group!(
    removal,
    vec_removal<100_000>,
    hash_set_removal<100_000>,
    hash_map_removal<100_000>,
    btree_map_removal<100_000>,
    sparseset_removal<100_000>
);
criterion_main!(removal);

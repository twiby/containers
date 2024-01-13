use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{HashSet, HashMap};

use containers::SparseSet;

pub fn vec_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "vec insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| b.iter(|| {
        let mut v = Vec::new();
        for i in 0..N {
            v.push(black_box(i));
        }
    }));
}

pub fn set_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "set insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| b.iter(|| {
        let mut v = HashSet::new();
        for i in 0..N {
            v.insert(black_box(i));
        }
    }));
}

pub fn map_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "map insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| b.iter(|| {
        let mut v = HashMap::<usize, usize>::new();
        for i in 0..N {
            v.insert(black_box(i), black_box(i));
        }
    }));
}

pub fn sparseset_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "sparseset insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| b.iter(|| {
        let mut v = SparseSet::new();
        for i in 0..N {
            v.insert(black_box(i));
        }
    }));
}

criterion_group!(
    insertion, 
    vec_insertion<10_000>, 
    set_insertion<10_000>, 
    map_insertion<10_000>, 
    sparseset_insertion<10_000>
);
criterion_main!(insertion);

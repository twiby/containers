use containers::StringMap;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use criterion::black_box;

pub fn hash_map_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "string hash map insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = HashMap::<String, usize>::new();
            for i in 0..N {
                v.insert(black_box(i.to_string()), black_box(i));
            }
        })
    });
}

pub fn string_map_insertion<const N: usize>(c: &mut Criterion) {
    let mut name = "string map insertion ".to_string();
    name.push_str(&N.to_string());
    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            let mut v = StringMap::<usize>::new();
            for i in 0..N {
                v.insert(black_box(i.to_string()), black_box(i));
            }
        })
    });
}

pub fn hash_map_access<const N: usize>(c: &mut Criterion) {
    let mut name = "string hash map access ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map[&mid]);
        })
    });
}

pub fn string_map_access<const N: usize>(c: &mut Criterion) {
    let mut name = "string map access ".to_string();
    name.push_str(&N.to_string());

    let mut map = StringMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map[&mid]);
        })
    });
}

pub fn hash_map_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "string hash map presence ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map.contains_key(&mid));
        })
    });
}

pub fn string_map_presence<const N: usize>(c: &mut Criterion) {
    let mut name = "string map presence ".to_string();
    name.push_str(&N.to_string());

    let mut map = StringMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter(|| {
            black_box(map.contains_key(&mid));
        })
    });
}

pub fn hash_map_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "string hash map removal ".to_string();
    name.push_str(&N.to_string());

    let mut map = HashMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                map.insert(mid.to_string(), N >> 1);
                let start = Instant::now();
                black_box(map.remove(&mid));
                ret += start.elapsed();
            }
            ret
        })
    });
}

pub fn string_map_removal<const N: usize>(c: &mut Criterion) {
    let mut name = "string map removal ".to_string();
    name.push_str(&N.to_string());

    let mut map = StringMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    let mid = (N >> 1).to_string();

    c.bench_function(name.as_str(), |b| {
        b.iter_custom(|iters| {
            let mut ret: Duration = Default::default();
            for _ in 0..iters {
                map.insert(mid.to_string(), N >> 1);
                let start = Instant::now();
                black_box(map.remove(&mid));
                ret += start.elapsed();
            }
            ret
        })
    });
}

criterion_group!(
    string_map,
    hash_map_insertion<100_000>,
    string_map_insertion<100_000>,
    hash_map_access<100_000>,
    string_map_access<100_000>,
    hash_map_presence<100_000>,
    string_map_presence<100_000>,
    hash_map_removal<100_000>,
    string_map_removal<100_000>,
);
criterion_main!(string_map);

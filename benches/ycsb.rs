/* ycsb.rs --- YCSB

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use cachetable::CacheTable;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::hint::black_box;

const NUM_OPS: usize = 10_000;
const KEY_SPACE: u64 = 1000;

/// YCSB Workload A: 50% GET, 50% PUT
pub fn workload_a(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload A");
    group.throughput(Throughput::Elements(NUM_OPS as u64));
    group.bench_function("CacheTable", |b| {
        b.iter_batched(
            || {
                let cache = CacheTable::<u64, Vec<u32>, 4, 32>::new();
                for i in 0..KEY_SPACE {
                    cache.insert(i, vec![i as u32]);
                }
                (cache, StdRng::seed_from_u64(42))
            },
            |(cache, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.5 {
                        black_box(cache.get(&key));
                    } else {
                        cache.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("HashTable", |b| {
        b.iter_batched(
            || {
                let mut hashtable = HashMap::<u64, Vec<u32>>::new();
                for i in 0..KEY_SPACE {
                    hashtable.insert(i, vec![i as u32]);
                }
                (hashtable, StdRng::seed_from_u64(42))
            },
            |(mut hashtable, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.5 {
                        black_box(hashtable.get(&key));
                    } else {
                        hashtable.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// YCSB Workload B: 95% GET, 5% PUT
pub fn workload_b(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload B");

    group.throughput(Throughput::Elements(NUM_OPS as u64));
    group.bench_function("CacheTable", |b| {
        b.iter_batched(
            || {
                let cache = CacheTable::<u64, Vec<u32>, 4, 32>::new();
                for i in 0..KEY_SPACE {
                    cache.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (cache, rng)
            },
            |(cache, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.95 {
                        black_box(cache.get(&key));
                    } else {
                        cache.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("HashTable", |b| {
        b.iter_batched(
            || {
                let mut hashtable = HashMap::<u64, Vec<u32>>::new();
                for i in 0..KEY_SPACE {
                    hashtable.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (hashtable, rng)
            },
            |(mut hashtable, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.95 {
                        black_box(hashtable.get(&key));
                    } else {
                        hashtable.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// YCSB Workload C: 100% GET
pub fn workload_c(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload C");

    group.throughput(Throughput::Elements(NUM_OPS as u64));
    group.bench_function("CacheTable", |b| {
        b.iter_batched(
            || {
                let cache = CacheTable::<u64, Vec<u32>, 4, 32>::new();
                for i in 0..KEY_SPACE {
                    cache.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (cache, rng)
            },
            |(cache, mut rng)| {
                for _ in 0..NUM_OPS {
                    let key = rng.random_range(0..KEY_SPACE);
                    black_box(cache.get(&key));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("HashTable", |b| {
        b.iter_batched(
            || {
                let mut hashtable = HashMap::<u64, Vec<u32>>::new();
                for i in 0..KEY_SPACE {
                    hashtable.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (hashtable, rng)
            },
            |(hashtable, mut rng)| {
                for _ in 0..NUM_OPS {
                    let key = rng.random_range(0..KEY_SPACE);
                    black_box(hashtable.get(&key));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// YCSB Workload F: 50% GET, 50% UPDATE
pub fn workload_f(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload F");

    group.throughput(Throughput::Elements(NUM_OPS as u64));
    group.bench_function("CacheTable", |b| {
        b.iter_batched(
            || {
                let cache = CacheTable::<u64, Vec<u32>, 4, 32>::new();
                for i in 0..KEY_SPACE {
                    cache.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (cache, rng)
            },
            |(cache, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.5 {
                        black_box(cache.get(&key));
                    } else {
                        // update existing key
                        cache.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("HashTable", |b| {
        b.iter_batched(
            || {
                let mut hashtable = HashMap::<u64, Vec<u32>>::new();
                for i in 0..KEY_SPACE {
                    hashtable.insert(i, vec![i as u32]);
                }
                let rng = StdRng::seed_from_u64(42);
                (hashtable, rng)
            },
            |(mut hashtable, mut rng)| {
                for _ in 0..NUM_OPS {
                    let op: f64 = rng.random();
                    let key = rng.random_range(0..KEY_SPACE);
                    if op < 0.5 {
                        black_box(hashtable.get(&key));
                    } else {
                        hashtable.insert(key, vec![key as u32]);
                    }
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, workload_a, workload_b, workload_c, workload_f);
criterion_main!(benches);

/* ycsb.rs ends here */

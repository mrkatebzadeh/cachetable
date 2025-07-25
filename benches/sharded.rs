/* sharded.rs

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

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::hint::black_box;
use std::sync::{Arc, Mutex};
use std::thread;

use cachetable::ShardedTable;

const NUM_OPS: usize = 10_000;
const KEY_SPACE: u64 = 1000;

pub fn workload_a(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload A");

    for threads in [1, 2, 4, 8, 16, 32] {
        group.throughput(Throughput::Elements((NUM_OPS * threads) as u64));

        group.bench_function(format!("ShardedTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(ShardedTable::<u64, Vec<u32>, 32, 32>::new());

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table: Arc<ShardedTable<u64, Vec<u32>, 32, 32>> = Arc::clone(&table);
                        thread::spawn(move || {
                            let shard = table.get_shard(tid % 32);
                            shard.register();
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let op: f64 = rng.random();
                                let key = rng.random_range(0..KEY_SPACE);
                                if op < 0.5 {
                                    black_box(shard.get(&key));
                                } else {
                                    shard.insert(key, vec![key as u32]);
                                }
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });

        group.bench_function(format!("HashTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(Mutex::new(HashMap::<u64, Vec<u32>>::new()));

                {
                    let mut map = table.lock().unwrap();
                    for i in 0..KEY_SPACE {
                        map.insert(i, vec![i as u32]);
                    }
                }

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table = Arc::clone(&table);
                        thread::spawn(move || {
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let op: f64 = rng.random();
                                let key = rng.random_range(0..KEY_SPACE);
                                if op < 0.5 {
                                    let guard = table.lock().unwrap();
                                    black_box(guard.get(&key));
                                } else {
                                    let mut guard = table.lock().unwrap();
                                    guard.insert(key, vec![key as u32]);
                                }
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    }

    group.finish();
}

pub fn workload_b(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload B");

    for threads in [1, 2, 4, 8, 16, 32] {
        group.throughput(Throughput::Elements((NUM_OPS * threads) as u64));

        group.bench_function(format!("ShardedTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(ShardedTable::<u64, Vec<u32>, 32, 32>::new());

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table: Arc<ShardedTable<u64, Vec<u32>, 32, 32>> = Arc::clone(&table);
                        thread::spawn(move || {
                            let shard = table.get_shard(tid % 32);
                            shard.register();
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let op: f64 = rng.random();
                                let key = rng.random_range(0..KEY_SPACE);
                                if op < 0.95 {
                                    black_box(shard.get(&key));
                                } else {
                                    shard.insert(key, vec![key as u32]);
                                }
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });

        group.bench_function(format!("HashTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(Mutex::new(HashMap::<u64, Vec<u32>>::new()));

                {
                    let mut map = table.lock().unwrap();
                    for i in 0..KEY_SPACE {
                        map.insert(i, vec![i as u32]);
                    }
                }

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table = Arc::clone(&table);
                        thread::spawn(move || {
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let op: f64 = rng.random();
                                let key = rng.random_range(0..KEY_SPACE);
                                if op < 0.95 {
                                    let guard = table.lock().unwrap();
                                    black_box(guard.get(&key));
                                } else {
                                    let mut guard = table.lock().unwrap();
                                    guard.insert(key, vec![key as u32]);
                                }
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    }

    group.finish();
}

pub fn workload_c(c: &mut Criterion) {
    let mut group = c.benchmark_group("Workload C");

    for threads in [1, 2, 4, 8, 16, 32] {
        group.throughput(Throughput::Elements((NUM_OPS * threads) as u64));

        group.bench_function(format!("ShardedTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(ShardedTable::<u64, Vec<u32>, 32, 32>::new());

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table: Arc<ShardedTable<u64, Vec<u32>, 32, 32>> = Arc::clone(&table);
                        thread::spawn(move || {
                            let shard = table.get_shard(tid % 32);
                            shard.register();
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let key = rng.random_range(0..KEY_SPACE);
                                black_box(shard.get(&key));
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });

        group.bench_function(format!("HashTable_{}t", threads), |b| {
            b.iter(|| {
                let table = Arc::new(Mutex::new(HashMap::<u64, Vec<u32>>::new()));

                {
                    let mut map = table.lock().unwrap();
                    for i in 0..KEY_SPACE {
                        map.insert(i, vec![i as u32]);
                    }
                }

                let handles: Vec<_> = (0..threads)
                    .map(|tid| {
                        let table = Arc::clone(&table);
                        thread::spawn(move || {
                            let mut rng = StdRng::seed_from_u64(42 + tid as u64);
                            for _ in 0..NUM_OPS {
                                let key = rng.random_range(0..KEY_SPACE);
                                let guard = table.lock().unwrap();
                                black_box(guard.get(&key));
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, workload_a, workload_b, workload_c);
criterion_main!(benches);
/* sharded.rs ends here */

/* access.rs --- ACCESS

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

use std::{collections::HashMap, hint::black_box};

use cachetable::CacheTable;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn put_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Put");
    group.bench_function("CacheTable", |b| {
        let cachetable = CacheTable::<u64, u64, 4, 32>::new();
        b.iter(|| {
            let key = black_box(10);
            let value = black_box(10);
            cachetable.insert(key, value);
        })
    });
    group.bench_function("HashTable", |b| {
        let mut hashtable = HashMap::<u64, u64>::new();
        b.iter(|| {
            let key = black_box(10);
            let value = black_box(10);
            hashtable.insert(key, value);
        })
    });
}

pub fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get");
    group.bench_function("CacheTable", |b| {
        let cachetable = CacheTable::<u64, u64, 4, 32>::new();
        let key = black_box(10);
        let value = black_box(10);
        cachetable.insert(key, value);
        b.iter(|| {
            black_box(cachetable.get(&key));
        })
    });
    group.bench_function("HashTable", |b| {
        let mut hashtable = HashMap::<u64, u64>::new();
        let key = black_box(10);
        let value = black_box(10);
        hashtable.insert(key, value);
        b.iter(|| {
            black_box(hashtable.get(&key));
        })
    });
}

criterion_group!(benches, put_bench, get_bench);
criterion_main!(benches);
/* access.rs ends here */

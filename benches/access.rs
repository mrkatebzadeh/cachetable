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

use std::collections::HashMap;

use cachetable::cachetable::CacheTable;
use cachetable::key::CacheKey;
use cachetable::value::CacheValue;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn put_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Put");
    group.bench_function("CacheTable", |b| {
        let mut cachetable = CacheTable::<4, 32>::new();
        b.iter(|| {
            let key = CacheKey::new(black_box(10));
            let value = CacheValue::new(black_box(&[10]));
            cachetable.insert(key, value);
        })
    });
    group.bench_function("HashTable", |b| {
        let mut hashtable = HashMap::<u32, Vec<u32>>::new();
        b.iter(|| {
            let key = 10;
            let value = vec![10];
            hashtable.insert(key, value);
        })
    });
}

pub fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get");
    group.bench_function("CacheTable", |b| {
        let mut cachetable = CacheTable::<4, 32>::new();
        let key = CacheKey::new(black_box(10));
        let value = CacheValue::new(black_box(&[10]));
        cachetable.insert(key, value);
        b.iter(|| {
            let get_value = cachetable.get(&key);
        })
    });
    group.bench_function("HashTable", |b| {
        let mut hashtable = HashMap::<u32, Vec<u32>>::new();
        let key = 10;
        let value = vec![10];
        hashtable.insert(key, value);
        b.iter(|| {
            let get_value = hashtable.get(&key);
        })
    });
}

criterion_group!(benches, put_bench, get_bench);
criterion_main!(benches);
/* access.rs ends here */

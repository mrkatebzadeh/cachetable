/* shardedtable.rs

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

use crate::CacheTable;
use std::hash::Hash;
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

const NUM_SHARDS: usize = 8;

pub struct Shard<K, V, const L: usize, const B: usize> {
    data: UnsafeCell<CacheTable<K, V, L, B>>,
    registered_thread: AtomicUsize,
}

unsafe impl<K: Send, V: Send, const L: usize, const B: usize> Sync for Shard<K, V, L, B> {}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const B: usize,
    > Shard<K, V, L, B>
{
    fn new() -> Self {
        Self {
            data: UnsafeCell::new(CacheTable::<K, V, L, B>::new()),
            registered_thread: AtomicUsize::new(usize::MAX),
        }
    }

    pub fn register(&self) -> bool {
        let tid = thread::current().id().as_u64().get() as usize;
        self.registered_thread
            .compare_exchange(usize::MAX, tid, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
    }

    pub fn insert(&self, key: K, value: V)
    where
        K: Eq + std::hash::Hash,
    {
        assert_eq!(
            self.registered_thread.load(Ordering::Relaxed),
            thread::current().id().as_u64().get() as usize
        );
        unsafe { &mut *self.data.get() }.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        K: Eq + std::hash::Hash,
        V: Clone,
    {
        assert_eq!(
            self.registered_thread.load(Ordering::Relaxed),
            thread::current().id().as_u64().get() as usize
        );
        unsafe { &*self.data.get() }.get(key)
    }
}

pub struct ShardedTable<K, V, const L: usize, const B: usize> {
    shards: Vec<Shard<K, V, L, B>>,
}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const B: usize,
    > ShardedTable<K, V, L, B>
{
    pub fn new() -> Self {
        Self {
            shards: (0..B).map(|_| Shard::new()).collect(),
        }
    }

    pub fn get_shard(&self, shard_id: usize) -> &Shard<K, V, L, B> {
        &self.shards[shard_id]
    }
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_basic_insert_get_single_thread() {
        let table = ShardedTable::<u64, &str, 2, 32>::new();
        let shard = table.get_shard(0);
        shard.register();
        shard.insert(10, "ten");
        shard.insert(20, "twenty");

        assert_eq!(shard.get(&10), Some("ten"));
        assert_eq!(shard.get(&20), Some("twenty"));
        assert_eq!(shard.get(&30), None);
    }

    #[test]
    fn test_double_register() {
        let table = ShardedTable::<u32, u32, 2, 32>::new();
        let shard = table.get_shard(0);

        assert!(shard.register());
        assert!(!shard.register());
    }

    #[test]
    fn test_eviction_in_shard() {
        let table = ShardedTable::<u32, u32, 8, 8>::new();
        let shard = table.get_shard(0);
        shard.register();

        for i in 0..50 {
            shard.insert(i, i);
        }

        let mut hits = 0;
        for i in 40..50 {
            if shard.get(&i).is_some() {
                hits += 1;
            }
        }

        assert!(hits >= 5);
    }

    #[test]
    fn test_shard_isolation() {
        let table = ShardedTable::<u64, &str, 2, 32>::new();

        let shard0 = table.get_shard(0);
        shard0.register();
        shard0.insert(1, "a");

        let shard1 = table.get_shard(1);
        shard1.register();
        shard1.insert(1 << 32, "b");

        assert_eq!(shard0.get(&1), Some("a"));
        assert_eq!(shard1.get(&(1 << 32)), Some("b"));
    }
}
/* shardedtable.rs ends here */

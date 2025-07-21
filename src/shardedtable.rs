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

use crate::shard::Shard;
use std::hash::Hash;

pub struct ShardedTable<KEY, VALUE, const LOG_SIZE: usize, const SET_SIZE: usize> {
    shards: Vec<Shard<KEY, VALUE, LOG_SIZE, SET_SIZE>>,
}

impl<
        KEY: Default + Hash + Eq + PartialEq + Clone,
        VALUE: Default + Clone,
        const LOG_SIZE: usize,
        const SET_SIZE: usize,
    > ShardedTable<KEY, VALUE, LOG_SIZE, SET_SIZE>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_shard(&self, shard_id: usize) -> &Shard<KEY, VALUE, LOG_SIZE, SET_SIZE> {
        &self.shards[shard_id]
    }
}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const S: usize,
    > Default for ShardedTable<K, V, L, S>
{
    fn default() -> Self {
        Self {
            shards: (0..S).map(|_| Shard::new()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
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

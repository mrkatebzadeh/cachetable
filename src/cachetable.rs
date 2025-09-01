/* cachetable.rs --- Cachetable

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

use crate::set::Set;
use crate::{kv::LogItem, log::Log};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use wyhash2::WyHash;

/// The `InnerCache` struct is responsible for managing the internal structure of the cache.
/// It uses sets to organize cache entries and maintains a log for storing key-value pairs.
struct InnerCache<K, V, const L: usize, const S: usize> {
    sets: [Set; S],
    log: Log<K, V, L>,
    set_mask: usize,
    log_mask: usize,
    log_head: usize,
}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const S: usize,
    > InnerCache<K, V, L, S>
{
    /// Creates a new `InnerCache` instance with default values.
    /// Ensures that the number of sets and log size are powers of two, which is
    /// required for efficient hashing and indexing.
    fn new() -> Self {
        assert!(S.is_power_of_two(), "Set size must be a power of two!");
        assert!(L.is_power_of_two(), "Log size must be a power of two!");
        let bkt_mask = S - 1;
        let log_mask = L - 1;
        Self {
            sets: [Set::default(); S],
            log: Log::<K, V, L>::default(),
            set_mask: bkt_mask,
            log_mask,
            log_head: 0,
        }
    }

    /// Probes the cache for a given key and returns the set index, fingerprint,
    /// and slot index if available.
    #[inline]
    fn probe(&self, key: &K) -> (usize, u8, Option<usize>) {
        let mut hasher = WyHash::with_seed(0);
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        let set = self.extract_set(key_hash);
        let finger = self.extract_finger(key_hash);
        (set, finger, self.sets[set].probe(finger))
    }

    /// Invalidates an entry in the cache associated with the given key.
    /// If the key is found, it marks the corresponding slot as invalid.
    #[inline]
    fn invalid(&mut self, key: &K) {
        match self.probe(key) {
            (_, _, None) => {}
            (set, _, Some(slot)) => {
                self.sets[set].valid_mask &= !(1 << slot);
            }
        }
    }

    /// Inserts a log item into the cache, replacing the oldest entry if necessary.
    /// If the key already exists, it updates the entry; otherwise, it inserts
    /// the new item and adjusts the log head.
    fn insert(&mut self, item: LogItem<K, V>) {
        let (set, finger, way) = self.probe(&item.key);

        match way {
            None => {
                let mut log_head = self.log_head;
                let old_key = self.log.entries[log_head].key.clone();
                self.invalid(&old_key);
                let slot = self.sets[set].next_slot();

                self.sets[set].set_finger(slot, finger);
                self.sets[set].valid_mask |= 1 << slot;
                self.sets[set].pointers[slot] = log_head;

                self.log.entries[log_head & self.log_mask] = item;
                log_head = (log_head + 1) % L;
                self.log_head = log_head;
            }
            Some(slot) => {
                let pointer = self.sets[set].pointers[slot];
                self.log.entries[pointer] = item;
            }
        }
    }

    /// Retrieves a value from the cache for a given key.
    /// Returns `Some(value)` if the key exists and is valid, `None` otherwise.
    fn get(&self, key: &K) -> Option<V> {
        let (set, _, slot) = self.probe(key);
        match slot {
            Some(slot) => {
                if (self.sets[set].valid_mask & (1 << slot)) == 0 {
                    return None;
                }

                let log_pos = self.sets[set].pointers[slot];
                Some(self.log.entries[log_pos].value.clone())
            }
            None => None,
        }
    }

    /// Extracts the set index from the hash key using the set mask.
    #[inline]
    fn extract_set(&self, key: u64) -> usize {
        (key as usize) & self.set_mask
    }

    /// Extracts the fingerprint from the hash key.
    #[inline]
    fn extract_finger(&self, key: u64) -> u8 {
        (key & 0xFF) as u8
    }
}

/// The `CacheTable` struct serves as the main interface for interacting with the cache.
/// It provides methods to insert, retrieve, and invalidate entries.
/// Internally, it manages an `InnerCache` instance wrapped in a RefCell for interior mutability.
///
/// # Type Parameters
/// - `K`: Key type, must implement `Default`, `Hash`, `Eq`, `PartialEq`, and `Clone`.
/// - `V`: Value type, must implement `Default` and `Clone`.
/// - `L`: Log size, must be a power of two.
/// - `B`: Number of sets in the cache, must be a power of two.
pub struct CacheTable<K, V, const L: usize, const B: usize> {
    inner: RefCell<InnerCache<K, V, L, B>>,
}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const B: usize,
    > CacheTable<K, V, L, B>
{
    /// Creates a new `CacheTable` instance.
    ///
    /// # Returns
    /// A new `CacheTable` object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the key already exists, its value will be updated.
    ///
    /// # Arguments
    /// * `key` - The key to insert.
    /// * `value` - The value associated with the key.
    pub fn insert(&self, key: K, value: V) {
        let mut item = LogItem::new();
        item.key = key;
        item.value = value;
        let mut inner = self.inner.borrow_mut();
        inner.insert(item);
    }

    /// Retrieves the value associated with the given key from the cache.
    ///
    /// # Arguments
    /// * `key` - The key to retrieve.
    ///
    /// # Returns
    /// An `Option` containing the value if the key exists and is valid, `None` otherwise.
    pub fn get(&self, key: &K) -> Option<V> {
        let inner = self.inner.borrow();
        inner.get(key)
    }

    /// Invalidates the cache entry associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to invalidate.
    pub fn invalid(&self, key: &K) {
        let mut inner = self.inner.borrow_mut();
        inner.invalid(key);
    }
}

impl<
        K: Default + Hash + Eq + PartialEq + Clone,
        V: Default + Clone,
        const L: usize,
        const B: usize,
    > Default for CacheTable<K, V, L, B>
{
    fn default() -> Self {
        let inner = RefCell::new(InnerCache::new());
        Self { inner }
    }
}

#[cfg(test)]
mod tests {

    use super::CacheTable;

    /// Tests the initialization of a CacheTable.
    #[test]
    fn init() {
        let _ = CacheTable::<u32, u32, 2, 32>::new();
    }

    /// Tests the insertion of a key-value pair into the cache.
    #[test]
    fn insert() {
        let key = 10;
        let value = vec![10];
        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value);
    }

    /// Tests the retrieval of a value by key from the cache.
    #[test]
    fn get() {
        let key = 10;
        let value = vec![10];

        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value.clone());

        let get_value = ctable.get(&key);
        assert!(get_value.is_some());
        assert_eq!(get_value.unwrap(), value);
    }

    /// Tests the invalidation of a cache entry.
    #[test]
    fn invalidate() {
        let key = 10;
        let value = vec![10];

        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value.clone());

        let get_value = ctable.get(&key);
        assert!(get_value.is_some());
        assert_eq!(get_value.unwrap(), value);

        ctable.invalid(&key);

        let get_value = ctable.get(&key);
        assert!(get_value.is_none());
    }

    /// Tests the retrieval failure of a non-existent key.
    #[test]
    fn get_fail() {
        let mut key = 10;
        let value = vec![10];

        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value);

        key = 11;
        let get_value = ctable.get(&key);
        assert!(get_value.is_none());
    }

    /// Tests the wrapping of log entries in the cache.
    #[test]
    fn log_wrap() {
        let mut key = 10;
        let value = vec![10];

        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value.clone());
        key = 15;
        ctable.insert(key, value.clone());

        key = 55;
        ctable.insert(key, value.clone());

        key = 15;
        let get_value = ctable.get(&key);

        assert!(get_value.is_some());
        assert_eq!(get_value.unwrap(), value);
    }
}

/* cachetable.rs ends here */

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
    fn new() -> Self {
        assert!(S.is_power_of_two(), "S must be a power of two!");
        assert!(L.is_power_of_two(), "L must be a power of two!");
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

    #[inline]
    fn probe(&self, key: &K) -> (usize, u8, Option<usize>) {
        let mut hasher = WyHash::with_seed(0);
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        let set = self.extract_set(key_hash);
        let finger = self.extract_finger(key_hash);
        (set, finger, self.sets[set].probe(finger))
    }

    #[inline]
    fn invalid(&mut self, key: &K) {
        match self.probe(key) {
            (_, _, None) => {}
            (set, _, Some(slot)) => {
                self.sets[set].valid_mask &= !(1 << slot);
            }
        }
    }

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

    #[inline]
    fn extract_set(&self, key: u64) -> usize {
        (key as usize) & self.set_mask
    }

    #[inline]
    fn extract_finger(&self, key: u64) -> u8 {
        (key & 0xFF) as u8
    }
}

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, key: K, value: V) {
        let mut item = LogItem::new();
        item.key = key;
        item.value = value;
        let mut inner = self.inner.borrow_mut();
        inner.insert(item);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let inner = self.inner.borrow();
        inner.get(key)
    }

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

    #[test]
    fn init() {
        let _ = CacheTable::<u32, u32, 2, 32>::new();
    }

    #[test]
    fn insert() {
        let key = 10;
        let value = vec![10];
        let ctable = CacheTable::<u32, Vec<u32>, 2, 32>::new();

        ctable.insert(key, value);
    }

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

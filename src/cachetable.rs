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

use crate::{bucket::Bucket, key::CacheKey, log::Log, op::Op, value::CacheValue};
use std::sync::atomic::{AtomicUsize, Ordering};

const BUCKET_SIZE: usize = 8;

pub struct CacheTable<const L: usize, const B: usize> {
    buckets: [Bucket<L, BUCKET_SIZE>; B],
    log: Log<L>,
    bkt_mask: usize,
    log_mask: usize,
    log_head: AtomicUsize,
}

impl<const L: usize, const B: usize> CacheTable<L, B> {
    pub fn new() -> CacheTable<L, B> {
        assert!(B.is_power_of_two(), "B must be a power of two!");
        assert!(L.is_power_of_two(), "L must be a power of two!");
        let bkt_mask = B - 1;
        let log_mask = L - 1;
        CacheTable {
            buckets: [Bucket::<L, BUCKET_SIZE>::default(); B],
            log: Log::<L>::default(),
            bkt_mask,
            log_mask,
            log_head: AtomicUsize::new(0),
        }
    }

    pub fn insert(&mut self, key: CacheKey, value: CacheValue) {
        let mut op = Op::new();
        op.key = key;
        op.value = value;
        self.insert_op(&op);
    }

    pub fn get(&self, key: &CacheKey) -> Option<&CacheValue> {
        let key_raw = key.key();
        let bkt = self.extract_bkt(key_raw);
        let tag = self.extract_tag(key_raw);

        let mut slot_idx: Option<usize> = None;
        for (index, slot) in self.buckets[bkt].slots.iter().enumerate() {
            if (slot.tag() & self.log_mask) == tag as usize && slot.valid() {
                slot_idx = Some(index);
                break;
            }
        }

        match slot_idx {
            Some(slot_idx) => {
                if self.log.entries[self.buckets[bkt].slots[slot_idx].log_idx()].key == *key {
                    let value =
                        &self.log.entries[self.buckets[bkt].slots[slot_idx].log_idx()].value;
                    Some(value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<const L: usize, const B: usize> CacheTable<L, B> {
    fn extract_bkt(&self, key: u64) -> usize {
        (key as usize) & self.bkt_mask
    }

    fn extract_tag(&self, key: u64) -> u32 {
        (key >> self.bkt_mask.count_ones()) as u32
    }

    fn insert_op(&mut self, op: &Op) {
        let key_raw = op.key.key();
        let bkt = self.extract_bkt(key_raw);
        let tag = self.extract_tag(key_raw);

        let mut slot_idx: Option<usize> = None;
        for (index, slot) in self.buckets[bkt].slots.iter().enumerate() {
            if (slot.tag() & self.log_mask) == tag as usize || !slot.valid() {
                slot_idx = Some(index);
                break;
            }
        }

        if slot_idx.is_none() {
            slot_idx = Some((tag & (BUCKET_SIZE - 1) as u32) as usize);
        }

        let slot_idx = slot_idx.unwrap();

        let mut log_head = self.log_head.load(Ordering::Acquire);

        self.buckets[bkt].slots[slot_idx].set_valid(true);
        self.buckets[bkt].slots[slot_idx].set_log_idx(log_head);
        self.buckets[bkt].slots[slot_idx].set_tag(tag as usize);

        self.log.entries[log_head & self.log_mask] = op.clone();

        log_head = (log_head + 1) % L;
        self.log_head.store(log_head, Ordering::Release);
    }
}
#[cfg(test)]
mod tests {
    use crate::{key::CacheKey, value::CacheValue};

    use super::CacheTable;

    #[test]
    fn init() {
        let _ = CacheTable::<2, 32>::new();
    }

    #[test]
    fn insert() {
        let key = CacheKey::new(10);
        let value = CacheValue::new(&[10]);
        let mut ctable = CacheTable::<2, 32>::new();

        ctable.insert(key, value);
    }

    #[test]
    fn get() {
        let key = CacheKey::new(10);
        let value = CacheValue::new(&[10]);

        let mut ctable = CacheTable::<2, 32>::new();

        ctable.insert(key, value.clone());

        let get_value = ctable.get(&key);
        assert!(get_value.is_some());
        assert_eq!(get_value.unwrap(), &value);
    }

    #[test]
    fn get_fail() {
        let mut key = CacheKey::new(10);
        let value = CacheValue::new(&[10]);

        let mut ctable = CacheTable::<2, 32>::new();

        ctable.insert(key, value.clone());

        key.set_key(11);
        let get_value = ctable.get(&key);
        assert!(get_value.is_none());
    }

    #[test]
    fn log_wrap() {
        let mut key = CacheKey::new(10);
        let value = CacheValue::new(&[10]);

        let mut ctable = CacheTable::<2, 32>::new();

        ctable.insert(key, value.clone());
        key.set_key(15);
        ctable.insert(key, value.clone());

        key.set_key(55);
        ctable.insert(key, value.clone());

        key.set_key(15);
        let get_value = ctable.get(&key);

        assert!(get_value.is_some());
        assert_eq!(get_value.unwrap(), &value);
    }
}

/* cachetable.rs ends here */

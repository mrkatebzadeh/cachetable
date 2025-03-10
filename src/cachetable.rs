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

use crate::{bucket::Bucket, key::Key, log::Log, op::Op, value::Value};
use std::sync::atomic::{AtomicUsize, Ordering};

const MICA_INDEX_SHM_KEY: usize = 1185;
const MICA_LOG_SHM_KEY: usize = 2185;
const BUCKET_SIZE: usize = 8;

pub struct CacheTable<const L: usize, const B: usize> {
    buckets: [Bucket<BUCKET_SIZE>; B],
    log: Log<L>,
    bkt_mask: usize,
    log_mask: usize,
    log_head: AtomicUsize,
}

impl<const L: usize, const B: usize> CacheTable<L, B> {
    pub fn insert(&mut self, op: &Op) {
        let bkt = op.key.bkt() & self.bkt_mask;
        let tag = op.key.tag();

        let mut slot_idx: Option<usize> = None;
        for (index, slot) in self.buckets[bkt].slots.iter().enumerate() {
            if slot.tag() == tag as usize || !slot.in_use() {
                slot_idx = Some(index);
                break;
            }
        }

        if slot_idx.is_none() {
            slot_idx = Some((tag & (BUCKET_SIZE - 1) as u16) as usize);
        }

        let slot_idx = slot_idx.unwrap();

        let mut log_head = self.log_head.load(Ordering::Relaxed);

        self.buckets[bkt].slots[slot_idx].set_in_use(true);
        self.buckets[bkt].slots[slot_idx].set_offset(log_head as usize);
        self.buckets[bkt].slots[slot_idx].set_tag(tag as usize);

        self.log.entries[log_head & self.log_mask] = op.clone();

        log_head = (log_head + 1) % L;
        self.log_head.store(log_head, Ordering::Relaxed);
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        let bkt = key.bkt() & self.bkt_mask;
        let tag = key.tag();

        let mut slot_idx: Option<usize> = None;
        for (index, slot) in self.buckets[bkt].slots.iter().enumerate() {
            if slot.tag() == tag as usize && slot.in_use() {
                slot_idx = Some(index);
                break;
            }
        }

        match slot_idx {
            Some(slot_idx) => {
                let value = &self.log.entries[self.buckets[bkt].slots[slot_idx].offset()].value;
                Some(value)
            }
            None => None,
        }
    }
    pub fn new() -> CacheTable<L, B> {
        assert!(B.is_power_of_two(), "BS must be a power of two!");
        let bkt_mask = B - 1;
        let log_mask = L - 1;
        CacheTable {
            buckets: [Bucket::default(); B],
            log: Log::<L>::default(),
            bkt_mask,
            log_mask,
            log_head: AtomicUsize::new(0),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{key::Key, op::Op};

    use super::CacheTable;

    #[test]
    fn init() {
        let _ = CacheTable::<10, 32>::new();
    }

    #[test]
    fn insert() {
        let mut op = Op::new();
        op.key.set_key(10);

        let mut ctable = CacheTable::<10, 32>::new();

        ctable.insert(&op);
    }

    #[test]
    fn get() {
        let mut op = Op::new();
        op.key.set_key(10);

        let mut ctable = CacheTable::<10, 32>::new();

        ctable.insert(&op);

        let mut key = Key::new();
        key.set_key(10);
        let value = ctable.get(&key);
        assert!(value.is_some());
        assert_eq!(value.unwrap(), &op.value);
    }

    #[test]
    fn get_fail() {
        let mut op = Op::new();
        op.key.set_key(10);

        let mut ctable = CacheTable::<10, 32>::new();

        ctable.insert(&op);

        let mut key = Key::new();
        key.set_key(11);
        let value = ctable.get(&key);
        assert!(value.is_none());
    }
}

/* cachetable.rs ends here */

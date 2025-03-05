/* kvs.rs --- KVS

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

use crate::{
    bucket::{MicaBucket, BUCKET_SIZE},
    op::{MicaOp, MICA_OP_SIZE},
    shm::SharedMemory,
    slot::{Slot, MICA_LOG_BITS},
};
use std::{
    ptr,
    sync::atomic::{AtomicU64, Ordering},
};

const MICA_INDEX_SHM_KEY: usize = 1185;
const MICA_LOG_SHM_KEY: usize = 2185;

pub struct MicaKVS {
    ht_index: SharedMemory<MicaBucket>,
    ht_log: SharedMemory<u8>,

    instance_id: i32,
    node_id: i32,
    num_bkts: i32,
    bkt_mask: i32,
    log_cap: u64,
    log_mask: u64,

    log_head: AtomicU64,

    num_get_op: AtomicU64,
    num_put_op: AtomicU64,
    num_get_fail: AtomicU64,
    num_put_fail: AtomicU64,
    num_insert_op: AtomicU64,
    num_index_evictions: AtomicU64,
}

impl MicaKVS {
    pub fn insert_one(&self, op: &MicaOp) {
        let bkt = op.key.bkt() & self.bkt_mask as u32;
        let bkt_ptr = unsafe { self.ht_index.buf().add(bkt as usize) };
        let tag = op.key.tag();

        self.num_insert_op.fetch_add(1, Ordering::Relaxed);

        let mut slot_to_use: Option<usize> = None;
        for i in 0..BUCKET_SIZE {
            let slot = unsafe { &(*bkt_ptr).slots[i] };
            if slot.tag() == tag as usize || !slot.in_use() {
                slot_to_use = Some(i);
                if slot.in_use() {
                    let evicted_op = unsafe {
                        &*(self
                            .ht_log
                            .buf()
                            .add((slot.offset() & self.log_mask as usize) as usize)
                            as *const MicaOp)
                    };
                    println!("Key {} overwrites {}", op.key_id, evicted_op.key_id);
                    panic!("Eviction detected");
                }
                break;
            }
        }

        let mut evict_flag = false;
        if slot_to_use.is_none() {
            slot_to_use = Some((tag & 7) as usize);
            self.num_index_evictions.fetch_add(1, Ordering::Relaxed);
            evict_flag = true;
        }

        let slot: &mut Slot = unsafe { &mut (*bkt_ptr).slots[slot_to_use.unwrap()] };
        slot.set_in_use(true);
        let offset = self.log_head.load(Ordering::Relaxed);
        slot.set_offset(offset as usize);
        slot.set_tag(tag as usize);

        let log_head = self.log_head.load(Ordering::Relaxed);
        let len_to_copy = std::mem::size_of::<MicaOp>();
        assert!((1u64 << MICA_LOG_BITS) - log_head > len_to_copy as u64 + 8);

        let log_offset = log_head & self.log_mask;
        let log_ptr = unsafe { self.ht_log.buf().add(log_offset as usize) as *mut u8 };

        if evict_flag {
            let evicted_op = unsafe { &*(log_ptr as *const MicaOp) };
            println!(
                "Evicting key: bkt {}, server {}, tag {}",
                evicted_op.key.bkt(),
                evicted_op.key.server(),
                evicted_op.key.tag()
            );
            panic!("Eviction error");
        }

        unsafe {
            ptr::copy_nonoverlapping(op as *const MicaOp as *const u8, log_ptr, len_to_copy);
        }

        let kv_ptr = log_ptr as *mut MicaOp;
        self.log_head
            .fetch_add(len_to_copy as u64, Ordering::Relaxed);

        self.log_head
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |log_head| {
                Some((log_head + 7) & !7)
            })
            .unwrap();

        let log_head = self.log_head.load(Ordering::Relaxed);
        if self.log_cap - log_head <= MICA_OP_SIZE as u64 + 32 {
            self.log_head
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |log_head| {
                    Some((log_head + self.log_cap) & !self.log_mask)
                })
                .unwrap();
            println!(
                "mica: Instance {} wrapping around. Wraps = {}",
                self.instance_id,
                self.log_head.load(Ordering::Relaxed) / self.log_cap
            );
            panic!("Log wraparound detected");
        }
    }

    pub fn instance_id(&self) -> i32 {
        self.instance_id
    }

    pub fn set_instance_id(&mut self, instance_id: i32) {
        self.instance_id = instance_id;
    }

    pub fn node_id(&self) -> i32 {
        self.node_id
    }

    pub fn set_node_id(&mut self, node_id: i32) {
        self.node_id = node_id;
    }

    pub fn num_bkts(&self) -> i32 {
        self.num_bkts
    }

    pub fn log_cap(&self) -> u64 {
        self.log_cap
    }
}

#[derive(Default)]
pub struct MicaBuilder {
    instance_id: i32,
    node_id: i32,
    num_bkts: i32,
    log_cap: u64,
}

impl MicaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_instance_id(mut self, instance_id: i32) -> Self {
        self.instance_id = instance_id;
        self
    }

    pub fn set_node_id(mut self, node_id: i32) -> Self {
        self.node_id = node_id;
        self
    }

    pub fn set_num_bkts(mut self, num_bkts: i32) -> Self {
        self.num_bkts = num_bkts;
        self
    }

    pub fn set_log_cap(mut self, log_cap: u64) -> Self {
        self.log_cap = log_cap;
        self
    }

    pub fn build(self) -> MicaKVS {
        assert!((self.num_bkts as u32).is_power_of_two());
        let bkt_mask = self.num_bkts - 1;
        let log_mask = self.log_cap - 1;
        let ht_index_key = MICA_INDEX_SHM_KEY as i32 + self.instance_id;

        let ht_index = SharedMemory::<MicaBucket>::new(
            ht_index_key,
            self.num_bkts as usize * size_of::<MicaBucket>(),
            false, //hugepages
            self.node_id,
        )
        .expect("Failed to allocate shared memory for buckets.");

        let ht_log_key = MICA_LOG_SHM_KEY as i32 + self.instance_id;
        let ht_log = SharedMemory::<u8>::new(
            ht_log_key,
            self.log_cap as usize,
            false, //hugepages
            self.node_id,
        )
        .expect("Failed to allocate shared memory for log.");

        MicaKVS {
            ht_index,
            ht_log,
            instance_id: self.instance_id,
            node_id: self.node_id,
            num_bkts: self.num_bkts,
            bkt_mask,
            log_cap: self.log_cap,
            log_mask,
            log_head: AtomicU64::new(0),
            num_get_op: AtomicU64::new(0),
            num_put_op: AtomicU64::new(0),
            num_get_fail: AtomicU64::new(0),
            num_put_fail: AtomicU64::new(0),
            num_insert_op: AtomicU64::new(0),
            num_index_evictions: AtomicU64::new(0),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{key::MicaKey, op::MicaOpBuilder};

    use super::MicaBuilder;

    #[test]
    fn init() {
        let kvs = MicaBuilder::new()
            .set_instance_id(1)
            .set_log_cap(10)
            .set_node_id(1)
            .set_num_bkts(4)
            .build();
    }

    #[test]
    fn insert() {
        let mut op = MicaOpBuilder::new().value_size(46).padding_size(10).build();
        op.key.set_key(10);
        let kvs = MicaBuilder::new()
            .set_instance_id(1)
            .set_log_cap(10)
            .set_node_id(1)
            .set_num_bkts(4)
            .build();
        kvs.insert_one(&op);
    }
}

/* kvs.rs ends here */

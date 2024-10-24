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

use crate::consts::{MAX_SLOTS, MICA_INDEX_SHM_KEY, MICA_LOG_BITS};
use crate::numbers::M_16;
use crate::op::MicaOp;
use crate::slot::MicaSlot;
use crate::utils::is_power_of_2;

struct MicaBucket {
    slots: [MicaSlot; MAX_SLOTS],
}

struct MicaKV {
    ht_index: Box<MicaBucket>,
    ht_log: Box<u8>,
    // Metadata
    instance_id: usize,
    node_id: usize,

    num_bkts: u32,
    bkt_mask: u32,

    log_cap: u64,
    log_mask: u64,

    // State
    log_head: u64,

    // Stats
    num_get_op: u64,
    num_put_op: u64,
    num_get_fail: u64,
    num_put_fail: u64,
    num_insert_op: u64,
    num_index_evictions: u64,
}

impl MicaKV {
    pub fn new(instance_id: usize, node_id: usize, num_bkts: u32, log_cap: u64) -> Self {
        // Structure size
        assert_eq!(std::mem::size_of::<MicaSlot>(), 8);
        // Structure alignment
        assert_eq!(std::mem::size_of::<MicaOp>() % 64, 0);
        assert!(node_id == 0 || node_id == 1);

        // 16 million buckets = a 1GB index
        assert!(is_power_of_2(num_bkts));
        assert!(num_bkts <= M_16);

        // Minimum log w_size = 16MB
        assert!(MICA_LOG_BITS >= 24);

        let ht_index_key = MICA_INDEX_SHM_KEY + instance_id;
    }
}

/* kvs.rs ends here */

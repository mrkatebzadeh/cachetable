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

const MicaLogBits: u32 = 40;
const MaxSlots: u32 = 8;

struct MicaSlot {
    tag: u32,
    offseet: u64,
    in_use: bool,
}

struct MicaBucket {
    slots: [MicaSlot; MaxSlots],
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

/* kvs.rs ends here */

/* op.rs --- OP

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

use crate::consts::{MICA_OP_PADDING_SIZE, MICA_VALUE_SIZE};
use crate::seqlock::SeqLock;

#[repr(C)]
pub(crate) struct MicaOp {
    value: [u8; MICA_VALUE_SIZE],
    key: Key,
    seqlock: SeqLock,
    version: u64,
    state: u8,
    unused: [u8; 3],
    key_id: u32, // strictly for debug
    padding: [u8; MICA_OP_PADDING_SIZE],
}

/* op.rs ends here */

/* slot.rs --- SLOT

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

use crate::consts::MICA_LOG_BITS;

pub(crate) struct MicaSlot {
    tag: u32,
    offset: u64,
    in_use: bool,
}

impl MicaSlot {
    pub(crate) fn new(in_use: u32, tag: u64, offset: u64) -> Self {
        let in_use = in_use & 0b1;
        let tag_mask = (1 << (64 - MICA_LOG_BITS - 1)) - 1;
        let tag = tag & tag_mask;
        let offset_mask = (1 << MICA_LOG_BITS) - 1;
        let offset = offset & offset_mask;

        MicaSlot {
            in_use,
            tag,
            offset,
        }
    }
}

/* slot.rs ends here */

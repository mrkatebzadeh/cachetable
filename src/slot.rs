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

use std::fmt::Display;

pub(crate) const LOG_BITS: usize = 40;
const SLOT_SIZE: usize = 64;

#[derive(Default, Clone, Copy)]
pub(crate) struct Slot {
    raw: usize,
}

impl Slot {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set_in_use(&mut self, in_use: bool) {
        let in_use_bit = if in_use { 1 } else { 0 };
        self.raw = (self.raw & !(1 << (SLOT_SIZE - 1))) | (in_use_bit << (SLOT_SIZE - 1));
    }

    pub(crate) fn set_tag(&mut self, tag: usize) {
        let tag_bits = SLOT_SIZE - LOG_BITS - 1;
        assert!(tag < (1 << tag_bits), "Tag is too large");

        self.raw = (self.raw & !((1 << (LOG_BITS)) - 1)) | (tag << LOG_BITS);
    }

    pub(crate) fn set_offset(&mut self, offset: usize) {
        assert!(offset < (1 << LOG_BITS), "Offset is too large");
        self.raw = (self.raw & !((1 << LOG_BITS) - 1)) | offset;
    }

    pub(crate) fn in_use(&self) -> bool {
        (self.raw >> (SLOT_SIZE - 1)) & 1 == 1
    }

    pub(crate) fn tag(&self) -> usize {
        (self.raw >> LOG_BITS) & ((1 << (SLOT_SIZE - LOG_BITS - 1)) - 1)
    }

    pub(crate) fn offset(&self) -> usize {
        self.raw & ((1 << LOG_BITS) - 1)
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Slot=> in use: {}, tag: {}, offset: {}",
            self.in_use(),
            self.tag(),
            self.offset()
        )
    }
}

/* slot.rs ends here */

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

const SLOT_SIZE: usize = 64;

#[derive(Default, Clone, Copy)]
pub(crate) struct Slot<const L: usize> {
    raw: usize,
}

impl<const L: usize> Slot<L> {
    const LOG_BITS: usize = L.next_power_of_two().trailing_zeros() as usize;
    const TAG_BITS: usize = SLOT_SIZE - Self::LOG_BITS - 1;
    const VALID_MASK: usize = 1 << (SLOT_SIZE - 1);
    const LOG_MASK: usize = (1 << Self::LOG_BITS) - 1;
    const TAG_MASK: usize = (1 << Self::TAG_BITS) - 1;

    pub(crate) fn set_valid(&mut self, valid: bool) {
        if valid {
            self.raw |= Self::VALID_MASK;
        } else {
            self.raw &= !Self::VALID_MASK;
        }
    }

    pub(crate) fn set_tag(&mut self, tag: usize) {
        self.raw = (self.raw & !(Self::TAG_MASK << Self::LOG_BITS))
            | ((tag & Self::TAG_MASK) << Self::LOG_BITS);
    }

    pub(crate) fn set_log_idx(&mut self, idx: usize) {
        self.raw = (self.raw & !Self::LOG_MASK) | (idx & Self::LOG_MASK);
    }

    pub(crate) fn valid(&self) -> bool {
        (self.raw & Self::VALID_MASK) != 0
    }

    pub(crate) fn tag(&self) -> usize {
        (self.raw >> Self::LOG_BITS) & (Self::TAG_MASK)
    }

    pub(crate) fn log_idx(&self) -> usize {
        self.raw & Self::LOG_MASK
    }
}

impl<const L: usize> Display for Slot<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Slot=> valid: {}, tag: {}, log_idx: {}",
            self.valid(),
            self.tag(),
            self.log_idx()
        )
    }
}

/* slot.rs ends here */

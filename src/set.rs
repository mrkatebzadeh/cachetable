/* set.rs

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

use std::simd::{cmp::SimdPartialEq, u8x16};
pub(crate) const SIMD_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Set {
    pub(crate) fingers: u8x16,
    pub(crate) valid_mask: u16,
    pub(crate) _reserved: u16,
    pub(crate) next: usize,
    pub(crate) pointers: [usize; SIMD_SIZE],
}

impl Default for Set {
    fn default() -> Self {
        Self {
            fingers: u8x16::splat(0),
            valid_mask: 0,
            next: 0,
            _reserved: 0,
            pointers: [0; SIMD_SIZE],
        }
    }
}

const LANE_MASKS: [u8x16; SIMD_SIZE] = {
    let mut masks = [u8x16::splat(0); SIMD_SIZE];
    let mut i = 0;
    while i < SIMD_SIZE {
        let mut arr = [0u8; SIMD_SIZE];
        arr[i] = 0xFF;
        masks[i] = u8x16::from_array(arr);
        i += 1;
    }
    masks
};

impl Set {
    #[inline(always)]
    pub fn next_slot(&mut self) -> usize {
        if self.valid_mask != u16::MAX {
            let inv_mask = !self.valid_mask;
            let first_zero = inv_mask.trailing_zeros() as usize;
            return first_zero;
        }

        let slot = self.next;
        self.next = (self.next + 1) % SIMD_SIZE;
        slot
    }

    #[inline(always)]
    pub fn set_finger(&mut self, slot: usize, value: u8) {
        let value_vec = u8x16::splat(value);
        let mask = LANE_MASKS[slot];
        self.fingers = (self.fingers & !mask) | (value_vec & mask);
    }

    #[inline(always)]
    pub fn probe(&self, needle: u8) -> Option<usize> {
        let simd_needle = u8x16::splat(needle);
        let cmp_mask = self.fingers.simd_eq(simd_needle).to_bitmask();
        let live = cmp_mask as u16 & self.valid_mask;
        if live != 0 {
            let slot = live.trailing_zeros() as usize;
            return Some(slot);
        }
        None
    }
}
/* set.rs ends here */

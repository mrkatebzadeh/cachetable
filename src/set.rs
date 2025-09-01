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
/// The `Set` struct is used to manage a collection of cache entries.
/// It utilizes SIMD (Single Instruction, Multiple Data) operations to store
/// and manipulate the cache entries efficiently. Each `Set` contains:
///
/// - `fingers`: A 128-bit register divided into sixteen 8-bit slots, each slot
///   holds a "finger" which is a small hash of the key.
/// - `valid_mask`: A 16-bit mask that indicates the validity of the corresponding
///   slots in the `fingers` register.
/// - `_padding`: A padding field for alignment.
/// - `next`: An index used for round-robin selection when all slots are filled.
/// - `pointers`: An array of pointers to the actual cache entries.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Set {
    pub(crate) fingers: u8x16,
    pub(crate) valid_mask: u16,
    pub(crate) _padding: u16,
    pub(crate) next: usize,
    pub(crate) pointers: [usize; SIMD_SIZE],
}

impl Default for Set {
    fn default() -> Self {
        Self {
            fingers: u8x16::splat(0),
            valid_mask: 0,
            next: 0,
            _padding: 0,
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
    /// Finds the next available slot in the cache.
    ///
    /// This function first attempts to find the first empty slot by looking for
    /// a zero in the `valid_mask`. If no empty slots are found, it resorts to
    /// round-robin selection using the `next` index.
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

    /// Sets the finger value at a specified slot in the `fingers` register.
    ///
    /// This method uses SIMD operations to efficiently place a value into
    /// one of the 16 slots in the `fingers` 128-bit register.
    ///
    /// # Arguments
    /// * `slot` - The index of the slot to set.
    /// * `value` - The 8-bit value to be stored in the slot.
    #[inline(always)]
    pub fn set_finger(&mut self, slot: usize, value: u8) {
        let value_vec = u8x16::splat(value);
        let mask = LANE_MASKS[slot];
        self.fingers = (self.fingers & !mask) | (value_vec & mask);
    }

    /// Probes the `fingers` register for a given needle value.
    ///
    /// This function checks if the given 8-bit needle value exists in the
    /// `fingers` register. Returns the index of the slot if found, or `None`
    /// if the needle is not present.
    ///
    /// # Arguments
    /// * `needle` - The 8-bit value to search for in the `fingers` register.
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

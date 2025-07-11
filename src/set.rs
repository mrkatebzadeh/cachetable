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

pub(crate) const WAYS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Set {
    pub fingers: u8x16,
    pub valid: u32,
    pub next: u32,
    pub pointers: [usize; WAYS],
}
impl Default for Set {
    fn default() -> Self {
        Self {
            fingers: u8x16::splat(0),
            valid: 0,
            next: 0,
            pointers: [0; WAYS],
        }
    }
}

impl Set {
    #[inline(always)]
    pub fn probe(&self, needle: u8) -> Option<usize> {
        let cmp_mask = self.fingers.simd_eq(u8x16::splat(needle)).to_bitmask();
        let masked = cmp_mask & self.valid as u64;

        if masked == 0 {
            None
        } else {
            Some(masked.trailing_zeros() as usize)
        }
    }

    #[inline(always)]
    pub fn set_finger(&mut self, index: usize, value: u8) {
        self.fingers[index] = value;
    }
}
/* set.rs ends here */

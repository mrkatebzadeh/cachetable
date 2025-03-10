/* key.rs --- KEY

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

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Key {
    raw: u64,
}

impl Key {
    pub fn new() -> Self {
        Key { raw: 0 }
    }

    pub fn bkt(&self) -> usize {
        (self.raw & 0xFFFFFFFF) as usize
    }

    pub fn set_bkt(&mut self, bkt: usize) {
        self.raw = (self.raw & !0xFFFFFFFF) | bkt as u64;
    }

    pub fn tag(&self) -> u16 {
        ((self.raw >> 48) & 0xFFFF) as u16
    }

    pub fn set_tag(&mut self, tag: u16) {
        self.raw = (self.raw & !(0xFFFF << 48)) | ((tag as u64) << 48);
    }

    pub fn set_key(&mut self, key: u64) {
        self.raw = key;
    }

    pub fn key(&mut self) -> u64 {
        self.raw
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key: {}", self.raw)
    }
}
/* key.rs ends here */

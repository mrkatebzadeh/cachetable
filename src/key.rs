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

pub(crate) const KEY_SIZE: usize = std::mem::size_of::<CacheKey>();

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[repr(C, packed)]
pub struct CacheKey {
    raw: u64,
}

impl CacheKey {
    pub fn new(key: u64) -> Self {
        CacheKey { raw: key }
    }

    pub fn set_key(&mut self, key: u64) {
        self.raw = key;
    }

    pub fn key(&self) -> u64 {
        let field_ptr = std::ptr::addr_of!(self.raw);
        unsafe { field_ptr.read_unaligned() }
    }

    pub fn len(&self) -> usize {
        KEY_SIZE
    }
}

impl From<&str> for CacheKey {
    fn from(key: &str) -> Self {
        let mut raw = [0u8; KEY_SIZE];
        let bytes = key.as_bytes();
        let len = bytes.len().min(KEY_SIZE);
        Self {
            raw: u64::from_le_bytes(bytes[..len].try_into().unwrap()),
        }
    }
}

impl From<u64> for CacheKey {
    fn from(key: u64) -> Self {
        Self { raw: key }
    }
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key: {}", self.key())
    }
}
/* key.rs ends here */

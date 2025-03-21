/* value.rs --- VALUE

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

use crate::key::KEY_SIZE;

pub(crate) const CACHE_LINE: usize = 64;
pub(crate) const VALUE_SIZE: usize = CACHE_LINE - KEY_SIZE - 1 - 8;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(C, packed)]
pub struct CacheValue {
    pub(crate) raw: [u8; VALUE_SIZE],
}

impl CacheValue {
    pub fn new(value: &[u8]) -> Self {
        let mut raw = [0u8; VALUE_SIZE];

        let len = value.len().min(VALUE_SIZE);
        raw[..len].copy_from_slice(&value[..len]);

        Self { raw }
    }

    pub fn len(&self) -> usize {
        VALUE_SIZE
    }
}

impl From<&str> for CacheValue {
    fn from(value: &str) -> Self {
        let mut raw = [0u8; VALUE_SIZE];
        let bytes = value.as_bytes();
        let len = bytes.len().min(VALUE_SIZE);
        raw[..len].copy_from_slice(&bytes[..len]);
        Self { raw }
    }
}

impl Display for CacheValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value: {}",
            self.raw
                .iter()
                .map(|v| { format!("{}", v) })
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Default for CacheValue {
    fn default() -> Self {
        Self {
            raw: std::array::from_fn(|_| 0),
        }
    }
}
/* value.rs ends here */

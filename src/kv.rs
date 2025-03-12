/* kv.rs --- KV

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

use crate::{
    key::{CacheKey, KEY_SIZE},
    value::{CacheValue, CACHE_LINE, VALUE_SIZE},
};
use std::fmt::Display;

const LOG_ITEM_PADDING: usize = CACHE_LINE - KEY_SIZE - VALUE_SIZE;
#[derive(Clone)]
pub(crate) struct LogItem {
    pub(crate) key: CacheKey,
    pub(crate) value: CacheValue,
    _padding: [u8; LOG_ITEM_PADDING],
}

impl LogItem {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Default for LogItem {
    fn default() -> Self {
        Self {
            value: CacheValue::default(),
            key: CacheKey::default(),
            _padding: [0; LOG_ITEM_PADDING],
        }
    }
}

impl Display for LogItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}

/* kv.rs ends here */

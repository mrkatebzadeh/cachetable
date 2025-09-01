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

use std::fmt::Display;

/// A `LogItem` structure that holds a key-value pair.
///
/// # Type Parameters
/// - `K`: The type of the key.
/// - `V`: The type of the value.
#[derive(Clone)]
#[repr(align(64))]
pub(crate) struct LogItem<K, V> {
    pub(crate) key: K,
    pub(crate) value: V,
}

impl<K: Default, V: Default> LogItem<K, V> {
    /// Creates a new `LogItem` with default key and value.
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl<K: Default, V: Default> Default for LogItem<K, V> {
    /// Provides a default implementation for `LogItem`.
    /// Returns a `LogItem` with default key and value.
    fn default() -> Self {
        Self {
            value: V::default(),
            key: K::default(),
        }
    }
}

impl<K: Display, V: Display> Display for LogItem<K, V> {
    /// Formats the `LogItem` as a string in the form "(key, value)".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}

/* kv.rs ends here */

/* log.rs --- LOG

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

use crate::kv::LogItem;
use std::fmt::Display;

/// A `Log` structure that holds a fixed-size array of log items.
///
/// # Type Parameters
/// - `Key`: The type of the key.
/// - `Value`: The type of the value.
/// - `LOG_SIZE`: The fixed size of the log.
///
/// The `Log` is used to store `LogItem` instances, each containing a key-value pair.
/// It provides a default implementation to initialize the log with default log items.
pub(crate) struct Log<Key, Value, const LOG_SIZE: usize> {
    pub(crate) entries: [LogItem<Key, Value>; LOG_SIZE],
}

impl<Key: Default, Value: Default, const LOG_SIZE: usize> Default for Log<Key, Value, LOG_SIZE> {
    /// Provides a default implementation for `Log`.
    /// Initializes the log with default log items.
    fn default() -> Self {
        Self {
            entries: std::array::from_fn(|_| LogItem::default()),
        }
    }
}

impl<Key: Display, Value: Display, const LOG_SIZE: usize> Display for Log<Key, Value, LOG_SIZE> {
    /// Formats the `Log` as a string, with each `LogItem` separated by a newline.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.entries
                .iter()
                .map(|item| format!("{item}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/* log.rs ends here */

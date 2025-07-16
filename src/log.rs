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

use std::fmt::Display;

use crate::kv::LogItem;

pub(crate) struct Log<Key, Value, const LOG_SIZE: usize> {
    pub(crate) entries: [LogItem<Key, Value>; LOG_SIZE],
}

impl<Key: Default, Value: Default, const LOG_SIZE: usize> Default for Log<Key, Value, LOG_SIZE> {
    fn default() -> Self {
        Self {
            entries: std::array::from_fn(|_| LogItem::default()),
        }
    }
}

impl<Key: Display, Value: Display, const LOG_SIZE: usize> Display for Log<Key, Value, LOG_SIZE> {
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

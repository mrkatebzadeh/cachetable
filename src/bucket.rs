/* bucket.rs --- BUCKET

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

use crate::slot::Slot;

#[derive(Clone, Copy)]
pub(crate) struct Bucket<const BS: usize> {
    pub(crate) slots: [Slot; BS],
}

impl<const BS: usize> Display for Bucket<BS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bucket=> [{}]",
            self.slots
                .iter()
                .map(|slot| format!("{}", slot))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl<const BS: usize> Default for Bucket<BS> {
    fn default() -> Self {
        Self {
            slots: [Slot::default(); BS],
        }
    }
}
/* bucket.rs ends here */

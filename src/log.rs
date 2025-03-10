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

use crate::op::Op;

pub(crate) struct Log<const N: usize> {
    pub(crate) entries: [Op; N],
}

impl<const N: usize> Default for Log<N> {
    fn default() -> Self {
        Self {
            entries: std::array::from_fn(|_| Op::default()),
        }
    }
}

impl<const N: usize> Display for Log<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.entries
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/* log.rs ends here */

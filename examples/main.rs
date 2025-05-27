/* main.rs --- MAIN

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

use cachetable::CacheTable;

fn main() {
    let key = 10;
    let value = vec![10];
    let ctable = CacheTable::<u32, Vec<u32>, 4 /* Log size */, 32 /* Bucket size*/>::new();
    ctable.insert(key, value);
    let value = ctable.get(&key);
}
/* main.rs ends here */

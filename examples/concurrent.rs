/* concurrent.rs

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
use cachetable::ShardedTable;
use std::{sync::Arc, thread};

fn main() {
    let table = Arc::new(ShardedTable::<u64, &str, 2, 32>::new());

    let handles = (0..2)
        .map(|shard_id| {
            let table = Arc::clone(&table);
            thread::spawn(move || {
                let shard = table.get_shard(shard_id);
                shard.register();
                shard.insert(10 * shard_id as u64, "value");
                println!("Inserted value in shard {}", shard_id);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}

/* concurrent.rs ends here */

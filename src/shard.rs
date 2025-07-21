/* shardedtable.rs

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

use crate::CacheTable;
use std::hash::Hash;
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

pub struct Shard<KEY, VALUE, const LOG_SIZE: usize, const SET_SIZE: usize> {
    data: UnsafeCell<CacheTable<KEY, VALUE, LOG_SIZE, SET_SIZE>>,
    registered_thread: AtomicUsize,
}

unsafe impl<KEY: Send, VALUE: Send, const LOG_SIZE: usize, const SET_SIZE: usize> Sync
    for Shard<KEY, VALUE, LOG_SIZE, SET_SIZE>
{
}

impl<
        KEY: Default + Hash + Eq + PartialEq + Clone,
        VALUE: Default + Clone,
        const LOG_SIZE: usize,
        const SET_SIZE: usize,
    > Shard<KEY, VALUE, LOG_SIZE, SET_SIZE>
{
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(CacheTable::<KEY, VALUE, LOG_SIZE, SET_SIZE>::new()),
            registered_thread: AtomicUsize::new(usize::MAX),
        }
    }

    pub fn register(&self) -> bool {
        let tid = thread::current().id().as_u64().get() as usize;
        self.registered_thread
            .compare_exchange(usize::MAX, tid, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
    }

    pub fn insert(&self, key: KEY, value: VALUE)
    where
        KEY: Eq + std::hash::Hash,
    {
        assert_eq!(
            self.registered_thread.load(Ordering::Relaxed),
            thread::current().id().as_u64().get() as usize
        );
        unsafe { &mut *self.data.get() }.insert(key, value);
    }

    pub fn get(&self, key: &KEY) -> Option<VALUE>
    where
        KEY: Eq + std::hash::Hash,
        VALUE: Clone,
    {
        assert_eq!(
            self.registered_thread.load(Ordering::Relaxed),
            thread::current().id().as_u64().get() as usize
        );
        unsafe { &*self.data.get() }.get(key)
    }
}


/* shardedtable.rs ends here */

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

/// The `Shard` struct is responsible for managing a portion of the cache table.
/// It ensures thread-safe access to the underlying `CacheTable` by associating
/// each shard with a specific thread.
///
/// # Type Parameters
/// - `KEY`: The type of the keys used in the cache.
/// - `VALUE`: The type of the values stored in the cache.
/// - `LOG_SIZE`: The size of the log in the cache, must be a power of two.
/// - `SET_SIZE`: The number of sets in the cache, must be a power of two.
///
/// The `Shard` uses an `UnsafeCell` to wrap the `CacheTable` and an `AtomicUsize`
/// to track the thread that is associated with the shard for safe concurrent access.
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
    /// Creates a new `Shard` instance.
    ///
    /// This function initializes a `Shard` with a new `CacheTable` and sets the
    /// `registered_thread` to `usize::MAX`, indicating that the shard is not yet
    /// associated with any thread.
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(CacheTable::<KEY, VALUE, LOG_SIZE, SET_SIZE>::new()),
            registered_thread: AtomicUsize::new(usize::MAX),
        }
    }

    /// Registers the current thread with the `Shard`.
    ///
    /// This method attempts to associate the current thread with the shard. It
    /// uses an atomic compare-and-exchange operation to set the `registered_thread`
    /// to the current thread's ID if it is not already set. Returns `true` if the
    /// registration was successful, `false` otherwise.
    pub fn register(&self) -> bool {
        let tid = thread::current().id().as_u64().get() as usize;
        self.registered_thread
            .compare_exchange(usize::MAX, tid, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
    }

    /// Inserts a key-value pair into the `CacheTable` within the `Shard`.
    ///
    /// This function asserts that the current thread is the registered thread
    /// for the shard and then inserts the key-value pair into the cache.
    ///
    /// # Arguments
    /// * `key` - The key to insert.
    /// * `value` - The value to associate with the key.
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

    /// Retrieves a value from the `CacheTable` within the `Shard` for a given key.
    ///
    /// This function asserts that the current thread is the registered thread
    /// for the shard and then retrieves the value associated with the key.
    ///
    /// # Arguments
    /// * `key` - A reference to the key for which to retrieve the value.
    ///
    /// # Returns
    /// An `Option` containing the value if the key exists, `None` otherwise.
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

/* consts.rs --- CONSTS

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

pub(crate) const USE_HUGE_PAGES: bool = false;
pub(crate) const LEVERAGE_TLB_COALESCING: bool = true;
pub(crate) const HUGE_PAGE_SIZE: usize = 2097152;

const USE_BIG_OBJECTS: bool = false;
const EXTRA_CACHE_LINES: usize = 0;
const BASE_VALUE_SIZE: usize = 32;
const SHIFT_BITS: usize = if USE_BIG_OBJECTS { 3 } else { 0 };

const VALUE_SIZE_: usize = if USE_BIG_OBJECTS {
    (EXTRA_CACHE_LINES * 64) + BASE_VALUE_SIZE
} else {
    BASE_VALUE_SIZE
};

const fn find_padding_cust_align(size: usize, alignment: usize) -> usize {
    (alignment - (size % alignment)) % alignment
}

const VALUE_SIZE: usize = VALUE_SIZE_ + find_padding_cust_align(VALUE_SIZE_, 8);
pub(crate) const MICA_VALUE_SIZE: usize = VALUE_SIZE + find_padding_cust_align(VALUE_SIZE, 32);

const MICA_OP_SIZE_: usize = 32 + MICA_VALUE_SIZE;

const fn find_padding(size: usize) -> usize {
    (64 - (size % 64)) % 64
}

pub(crate) const MICA_OP_PADDING_SIZE: usize = find_padding(MICA_OP_SIZE_);
const MICA_OP_SIZE: usize = MICA_OP_SIZE_ + MICA_OP_PADDING_SIZE;

const KVS_NUM_KEYS: usize = 1_000_000;

pub(crate) const MAX_SLOTS: usize = 8;
pub(crate) const MICA_LOG_BITS: usize = 40;
pub(crate) const MICA_INDEX_SHM_KEY: usize = 1185;
pub(crate) const MICA_LOG_SHM_KEY: usize = 2185;

/* consts.rs ends here */

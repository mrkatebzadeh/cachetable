/* ffi.rs --- FFI

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

extern "C" {
    pub fn mica_new(kvs_id: isize);

    pub fn mica_read(
        kv_ptr: *mut MicaOp,
        value_to_read: *mut uint8_t,
        resp_type: *mut uint8_t,
        t_id: uint16_t,
    );

    pub fn mica_write(kv_ptr: *mut MicaOp, value_to_write: *mut uint8_t);
}

/* ffi.rs ends here */

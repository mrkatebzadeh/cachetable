/* key.rs --- KEY

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

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MicaKey {
    raw: u64,
}

impl MicaKey {
    pub fn new() -> Self {
        MicaKey { raw: 0 }
    }

    pub fn get_bkt(&self) -> u32 {
        (self.raw & 0xFFFFFFFF) as u32
    }

    pub fn set_bkt(&mut self, bkt: u32) {
        self.raw = (self.raw & !0xFFFFFFFF) | (bkt as u64);
    }

    pub fn get_server(&self) -> u16 {
        ((self.raw >> 32) & 0xFFFF) as u16
    }

    pub fn set_server(&mut self, server: u16) {
        self.raw = (self.raw & !(0xFFFF << 32)) | ((server as u64) << 32);
    }

    pub fn get_tag(&self) -> u16 {
        ((self.raw >> 48) & 0xFFFF) as u16
    }

    pub fn set_tag(&mut self, tag: u16) {
        self.raw = (self.raw & !(0xFFFF << 48)) | ((tag as u64) << 48);
    }
}

/* key.rs ends here */

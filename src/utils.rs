/* utils.rs --- UTILS

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
use shm::{
    ffi::{Ipc, Shm},
    shmat, shmget,
};
use std::ffi::{c_void, CStr};
use std::os::raw::c_int;
use std::ptr;

use crate::consts::{HUGE_PAGE_SIZE, LEVERAGE_TLB_COALESCING, USE_HUGE_PAGES};

pub(crate) fn is_power_of_2(x: u32) -> bool {
    matches!(
        x,
        1 | 2
            | 4
            | 8
            | 16
            | 32
            | 64
            | 128
            | 256
            | 512
            | 1024
            | 2048
            | 4096
            | 8192
            | 16384
            | 32768
            | 65536
            | 131072
            | 262144
            | 524288
            | 1048576
            | 2097152
            | 4194304
            | 8388608
            | 16777216
            | 33554432
            | 67108864
            | 134217728
            | 268435456
            | 536870912
            | 1073741824
    )
}

const ERROR_PERMISSIONS: &str = "SHM malloc error: Insufficient permissions.";
const ERROR_EXISTS: &str = "SHM malloc error: Already exists.";
const ERROR_SHM_MISMATCH: &str = "SHM malloc error: SHMMAX/SHMIN mismatch.";
const ERROR_NO_MEMORY: &str = "SHM malloc error: Insufficient memory.";
const ERROR_NO_SEGMENT: &str =
    "SHM malloc error: No segment exists for the given key, and IPC_CREAT was not specified.";
const ERROR_NO_SPACE: &str = "SHM malloc error: All possible shared memory IDs have been taken or the limit of shared memory is exceeded.";
const ERROR_PRIVILEGE: &str =
    "SHM malloc error: The SHM_HUGETLB flag was specified, but the caller was not privileged.";
const ERROR_LIMIT_REACHED: &str =
    "SHM malloc error: The system-wide limit on the total number of open files has been reached.";

extern "C" {
    fn mbind(
        addr: *mut c_void,
        len: usize,
        mode: c_int,
        nodemask: *const usize,
        maxnode: u32,
        flags: c_int,
    ) -> c_int;
}

pub(crate) fn malloc_socket(
    shm_key: c_int,
    size: usize,
    socket_id: c_int,
) -> *mut std::ffi::c_void {
    let shmid = if USE_HUGE_PAGES {
        shmget!(
            shm_key,
            Ipc::CREAT as i32 | Ipc::EXCL as i32 | 0o666 as i32 | Shm::HUGETLB as i32,
            size
        )
    } else {
        shmget!(
            shm_key,
            Ipc::CREAT as i32 | Ipc::EXCL as i32 | 0o666 as i32,
            size
        )
    };

    if shmid == -1 {
        let errno = unsafe { *libc::__errno_location() };
        match errno {
            libc::EACCES => eprintln!("{} (SHM key = {})", ERROR_PERMISSIONS, shm_key),
            libc::EEXIST => eprintln!("{} (SHM key = {})", ERROR_EXISTS, shm_key),
            libc::EINVAL => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_SHM_MISMATCH, shm_key, size
            ),
            libc::ENOMEM => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_NO_MEMORY, shm_key, size
            ),
            libc::ENOENT => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_NO_SEGMENT, shm_key, size
            ),
            libc::ENOSPC => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_NO_SPACE, shm_key, size
            ),
            libc::EPERM => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_PRIVILEGE, shm_key, size
            ),
            libc::ENFILE => eprintln!(
                "{} (SHM key = {}, w_size = {})",
                ERROR_LIMIT_REACHED, shm_key, size
            ),
            _ => {
                let error_message = unsafe { CStr::from_ptr(libc::strerror(errno)) };
                eprintln!(
                    "SHM malloc error: A wild SHM error: {}.",
                    error_message.to_string_lossy()
                );
            }
        }
        panic!("SHM allocation failed");
    }

    let buf = shmat!(shmid, ptr::null_mut(), 0);
    if buf.is_null() {
        eprintln!("SHM malloc error: shmat() failed for key {}", shm_key);
        std::process::exit(-1);
    }

    // Bind the buffer to this socket
    let nodemask = 1 << socket_id;
    let ret = unsafe {
        mbind(
            buf,
            size,
            libc::MPOL_BIND,
            &nodemask,
            std::mem::size_of_val(&nodemask) as u32,
            0,
        )
    };
    if ret != 0 {
        eprintln!("SHM malloc error. mbind() failed for key {}", shm_key);
        std::process::exit(-1);
    }

    // Try to take advantage of TLB coalescing, if it is there
    if LEVERAGE_TLB_COALESCING {
        let page_no = (size + HUGE_PAGE_SIZE - 1) / HUGE_PAGE_SIZE; // CEILING(size, HUGE_PAGE_SIZE)
        for i in 0..page_no {
            unsafe {
                std::ptr::write_bytes(buf.add(i * HUGE_PAGE_SIZE), 0, 1);
            }
        }
    }

    buf
}

/* utils.rs ends here */

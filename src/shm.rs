/* shm.rs --- SHM

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

use anyhow::{Context, Result};
use libc::{
    shmat, shmctl, shmdt, shmget, syscall, EEXIST, EINVAL, ENFILE, ENOENT, ENOMEM, ENOSPC, EPERM,
    IPC_CREAT, IPC_EXCL, IPC_RMID, MAP_FAILED, S_IRUSR, S_IWUSR,
};
use std::ffi::CStr;
use std::ptr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedMemoryError {
    #[error("Insufficient permissions (SHM key = {0})")]
    PermissionDenied(i32),
    #[error("Already exists (SHM key = {0})")]
    AlreadyExists(i32),
    #[error("SHMMAX/SHMIN mismatch (SHM key = {0}, size = {1})")]
    InvalidSize(i32, usize),
    #[error("Insufficient memory (SHM key = {0}, size = {1})")]
    OutOfMemory(i32, usize),
    #[error("No segment exists for the key (SHM key = {0})")]
    SegmentNotFound(i32),
    #[error("Too many shared memory segments (SHM key = {0})")]
    TooManySegments(i32),
    #[error("SHM_HUGETLB requires privileges (SHM key = {0})")]
    HugePagesPermission(i32),
    #[error("System-wide file limit reached (SHM key = {0})")]
    FileLimitReached(i32),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[cfg(target_arch = "x86_64")]
const SYS_MBIND: i64 = 237;

#[cfg(target_arch = "aarch64")]
const SYS_MBIND: i64 = 356;

const SHM_HUGETLB: i32 = 0o04000;
const MPOL_BIND: i32 = 1;

pub struct SharedMemory<T> {
    shmid: i32,
    buf: *mut T,
    size: usize,
}

impl<T> SharedMemory<T> {
    pub fn new(shm_key: i32, size: usize, use_huge_pages: bool, socket_id: i32) -> Result<Self> {
        let shmflg = IPC_CREAT
            | IPC_EXCL
            | S_IRUSR as i32
            | S_IWUSR as i32
            | if use_huge_pages { SHM_HUGETLB } else { 0 };

        let shmid = unsafe { shmget(shm_key, size, shmflg) };
        if shmid == -1 {
            let err = Self::handle_shm_error(shm_key, size);
            println!("Warning: shmget has error: {:?},", err);
        }

        let buf = unsafe { shmat(shmid, ptr::null(), 0) } as *mut T;
        if buf == ptr::null_mut() {
            return Err(SharedMemoryError::Unexpected("shmat() failed".into()).into());
        }

        let nodemask = 1u64 << socket_id;
        let ret = unsafe {
            syscall(
                SYS_MBIND,
                buf as *mut libc::c_void,
                size,
                MPOL_BIND,
                &nodemask as *const u64,
                32,
                0,
            )
        };
        if ret != 0 {
            // return Err(SharedMemoryError::Unexpected("mbind() failed".into()).into());
        }

        Ok(SharedMemory { shmid, buf, size })
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.buf }
    }

    pub fn buf(&self) -> *mut T {
        unsafe { self.buf }
    }

    fn handle_shm_error(shm_key: i32, size: usize) -> SharedMemoryError {
        let err = std::io::Error::last_os_error();
        let errno = err.raw_os_error().unwrap_or(0);
        match errno {
            libc::EACCES => SharedMemoryError::PermissionDenied(shm_key),
            EEXIST => SharedMemoryError::AlreadyExists(shm_key),
            EINVAL => SharedMemoryError::InvalidSize(shm_key, size),
            ENOMEM => SharedMemoryError::OutOfMemory(shm_key, size),
            ENOENT => SharedMemoryError::SegmentNotFound(shm_key),
            ENOSPC => SharedMemoryError::TooManySegments(shm_key),
            EPERM => SharedMemoryError::HugePagesPermission(shm_key),
            ENFILE => SharedMemoryError::FileLimitReached(shm_key),
            _ => SharedMemoryError::Unexpected(err.to_string()),
        }
    }

    pub fn free(&self) -> Result<()> {
        let shmid = unsafe { shmget(self.shmid, 0, 0) };
        if shmid == -1 {
            let err = std::io::Error::last_os_error();
            let errno = err.raw_os_error().unwrap_or(0);
            return Err(match errno {
                libc::EACCES => SharedMemoryError::PermissionDenied(self.shmid),
                ENOENT => SharedMemoryError::SegmentNotFound(self.shmid),
                _ => SharedMemoryError::Unexpected(err.to_string()),
            }
            .into());
        }

        let ret = unsafe { shmctl(shmid, IPC_RMID, ptr::null_mut()) };
        if ret != 0 {
            return Err(SharedMemoryError::Unexpected("shmctl() failed".into()).into());
        }

        let ret = unsafe { shmdt(self.buf as *mut libc::c_void) };
        if ret != 0 {
            return Err(SharedMemoryError::Unexpected("shmdt() failed".into()).into());
        }
        Ok(())
    }
}

impl<T> Drop for SharedMemory<T> {
    fn drop(&mut self) {
        if let Err(e) = self.free() {
            eprintln!("Error freeing shared memory: {}", e);
        }
    }
}

/* shm.rs ends here */

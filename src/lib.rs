use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::path::Path;
use std::os::unix::ffi::OsStrExt;

#[derive(Copy, Clone, Debug)]
pub struct StatVFS {
    /// File system block size
    pub f_bsize: libc::c_ulong,
    /// fragment size
    pub f_frsize: libc::c_ulong,
    /// Size of fs in f_frsize units
    pub f_blocks: libc::fsblkcnt_t,
    /// Number of free blocks
    pub f_bfree: libc::fsblkcnt_t,
    /// Number of free blocks for unprivileged users
    pub f_bavail: libc::fsblkcnt_t,
    /// Number of inodes
    pub f_files: libc::fsfilcnt_t,
    /// Number of free inodes
    pub f_ffree: libc::fsfilcnt_t,
    /// Number of free nodes for unprivileged users
    pub f_favail: libc::fsfilcnt_t,
    /// File system Id
    pub f_fsid: libc::c_ulong,
    /// Mount flags
    pub f_flag: libc::c_ulong,
    /// Maximum file name length
    pub f_namemax: libc::c_ulong,
}


/// An error value returned from the failure of ['get_resource_limit'].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum StatVFSError {
    /// [EACCESS] Search permission is denied for a component of the path prefix of path.
    Permission,
    /// [EBADF] fd is not a valid open file descriptor.
    InvalidFileDescriptor,
    // / [EFAULT] Buf or path points to an invalid address.
    // / [EINTR] This call was interrupted by a signal.
    // / [EIO] An I/O error occurred while reading from the file system.
    // / [ELOOP] Too many symbolic links were encountered in translating path.
    // / [ENAMETOOLONG] path is too long.
    // / [ENOENT] The file referred to by path does not exist.
    // / [ENOMEM] Insufficient kernel memory was available.
    // / [ENOSYS] The file system does not support this call.
    // / [ENOTDIR] A component of the path prefix of path is not a directory.
    // / [EOVERFLOW] Some values were too large to be represented in the returned struct.
}


// pub unsafe extern "C" fn statvfs(
//     path: *const c_char,
//     buf: *mut statvfs
// ) -> c_int


/// Get the limit values for a particular resource.
pub fn statvfs(path: &Path) -> Result<libc::statvfs, StatVFSError> {
    use std::mem::MaybeUninit;
    let s: CString = CString::new(path.as_os_str().as_bytes()).expect("CString::new failed");
    unsafe {
        let mut buf: libc::statvfs = MaybeUninit::zeroed().assume_init();
        match libc::statvfs(s.as_ptr(), &mut buf) {
            0 => Ok(buf),
            -1 => {
                let errno: *mut libc::c_int = libc::__errno_location();
                Err(match *errno {
                    libc::EACCES => StatVFSError::Permission,
                    libc::EBADF => StatVFSError::InvalidFileDescriptor,
                    // libc::EFAULT
                    // libc::EINTR
                    // libc::EIO
                    // libc::ELOOP
                    // libc::ENAMETOOLONG
                    // libc::ENOENT
                    // libc::ENOMEM
                    // libc::ENOSYS
                    // libc::ENOTDIR
                    // libc::EOVERFLOW
                    _ => panic!("Invalid error code"),
                })
            }
            _ => panic!("Invalid error return"),
        }
    }
}

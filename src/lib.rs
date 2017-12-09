extern crate libc;
#[cfg(all(feature="memfd", target_os="linux"))]
#[macro_use]
extern crate syscall;

use std::ffi::CStr;
use libc::{c_int, off_t};

#[cfg(not(any(target_os="freebsd", all(target_os="linux", feature="memfd"))))]
pub fn create_shmem<T: AsRef<CStr>>(name: T, length: usize) -> c_int {
    unsafe {
        let fd = libc::shm_open(name.as_ref().as_ptr(),
                                libc::O_CREAT | libc::O_RDWR | libc::O_EXCL,
                                0o600);
        assert!(fd >= 0);
        assert!(libc::shm_unlink(name.as_ref().as_ptr()) == 0);
        assert!(libc::ftruncate(fd, length as off_t) == 0);
        fd
    }
}

#[cfg(target_os="freebsd")]
pub fn create_shmem<T: AsRef<CStr>>(_name: T, length: usize) -> c_int {
    unsafe {
        let fd = libc::shm_open(libc::SHM_ANON,
                                libc::O_CREAT | libc::O_RDWR | libc::O_EXCL,
                                0o600);
        assert!(fd >= 0);
        assert!(libc::ftruncate(fd, length as off_t) == 0);
        fd
    }
}

#[cfg(all(feature="memfd", target_os="linux"))]
pub fn create_shmem<T: AsRef<CStr>>(name: T, length: usize) -> c_int {
    unsafe {
        let fd = memfd_create(name.as_ref().as_ptr(), 0);
        assert!(fd >= 0);
        assert!(libc::ftruncate(fd, length as off_t) == 0);
        fd
    }
}

#[cfg(all(feature="memfd", target_os="linux"))]
unsafe fn memfd_create(name: *const libc::c_char, flags: usize) -> c_int {
    syscall!(MEMFD_CREATE, name, flags) as c_int
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn test_create_shmem() {
        super::create_shmem(CString::new("/helloworld").unwrap(), 1024);
    }
}

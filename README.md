[![crates.io](https://img.shields.io/crates/v/shmemfdrs.svg)](https://crates.io/crates/shmemfdrs)

# shmemfdrs

This tiny little crate provides a function that creates anonymous shared memory file descriptors for IPC, using the best available platform-specific method:

- on Linux, if the `memfd` Cargo feature is enabled for this crate: [memfd](http://man7.org/linux/man-pages/man2/memfd_create.2.html) (requires kernel >= 3.17)
- on FreeBSD: [shm_open(SHM_ANON)](https://www.freebsd.org/cgi/man.cgi?query=shm_open&sektion=2)
- otherwise: shm_open with a name, instantly unlinked

`memfd` and `SHM_ANON` are actually anonymous from the start, i.e. they don't need access the filesystem at all, so they work well with process sandboxes like [Capsicum](https://www.freebsd.org/cgi/man.cgi?query=capsicum&sektion=4).
(`memfd` actually uses the provided `name`, but only for debugging purposes.)

This crate was extracted from [ipc-channel](https://github.com/servo/ipc-channel).

## Usage

There's just one function exported from this crate:

```rust
fn create_shmem<T: AsRef<CStr>>(name: T, length: usize) -> c_int
```

Should be easy to figure out :)

The name should start with `/` and shouldn't be very long (255 characters max).

## Contributing

Please feel free to submit pull requests!

By participating in this project you agree to follow the [Contributor Code of Conduct](https://www.contributor-covenant.org/version/1/4/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/shmemfdrs/graphs/contributors).

## License

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

// librsync(rust) -- library for network deltas
// Copyright 2015, 2016 Martin Pool.

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

extern crate libc;
extern crate librdiff;


// See http://stackoverflow.com/a/33883281/243712: this marks the pointer Sync allowing it to
// be in a global.
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static rs_librsync_version: StaticCString =
    StaticCString(b"3.0.0\0" as *const u8);


// #[no_mangle]
// pub extern fn rs_version() -> *const libc::c_char {
//     // Version from environment has nul termination (I think we can count on this?)
//     return rs_librsync_version as *const libc::c_char;
// }


#[cfg(test)]
#[test]
pub fn test_versions_consistent() {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    let v = CStr::from_ptr(&rs_librsync_version as *const u8);

    // I can't work out how to automatically store a static CString, but
    // let's at least check they're in sync, and that ours has a nul.
    let their_v = librdiff::VERSION;
    assert_eq!(their_v, v.to_str().unwrap());

    // It should also be consistent with the Cargo version for librdiff-capi-rs.
    assert_eq!(their_v, env!("CARGO_PKG_VERSION"));
}

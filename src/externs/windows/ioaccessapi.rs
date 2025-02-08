#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::os::raw::{c_uchar, c_ulong, c_ushort};

#[link(name = "ioaccess")]
extern "system" {
    // READ_PORT_UCHAR returns a byte read from the specified port address.
    pub fn READ_PORT_UCHAR(Port: *const c_uchar) -> c_uchar;

    // READ_PORT_ULONG returns a ULONG read from the specified port address.
    pub fn READ_PORT_ULONG(Port: *const c_ulong) -> c_ulong;

    // READ_PORT_USHORT returns a USHORT read from the specified port address.
    pub fn READ_PORT_USHORT(Port: *const c_ushort) -> c_ushort;

    // READ_REGISTER_UCHAR returns a byte read from the specified register address.
    pub fn READ_REGISTER_UCHAR(Register: *const c_uchar) -> c_uchar;

    // READ_REGISTER_ULONG returns a ULONG read from the specified register address.
    pub fn READ_REGISTER_ULONG(Register: *const c_ulong) -> c_ulong;

    // READ_REGISTER_USHORT returns a USHORT read from the specified register address.
    pub fn READ_REGISTER_USHORT(Register: *const c_ushort) -> c_ushort;

    // WRITE_PORT_UCHAR writes a byte to the specified port address.
    pub fn WRITE_PORT_UCHAR(Port: *mut c_uchar, Value: c_uchar);

    // WRITE_PORT_ULONG writes a ULONG value to the specified port address.
    pub fn WRITE_PORT_ULONG(Port: *mut c_ulong, Value: c_ulong);

    // WRITE_PORT_USHORT writes a USHORT value to the specified port address.
    pub fn WRITE_PORT_USHORT(Port: *mut c_ushort, Value: c_ushort);

    // WRITE_REGISTER_UCHAR writes a byte to the specified register address.
    pub fn WRITE_REGISTER_UCHAR(Register: *mut c_uchar, Value: c_uchar);

    // WRITE_REGISTER_ULONG writes a ULONG value to the specified register address.
    pub fn WRITE_REGISTER_ULONG(Register: *mut c_ulong, Value: c_ulong);

    // WRITE_REGISTER_USHORT writes a USHORT value to the specified register address.
    pub fn WRITE_REGISTER_USHORT(Register: *mut c_ushort, Value: c_ushort);
}

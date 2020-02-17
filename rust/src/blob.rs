use crate::swipl::*;
use std::ffi;

#[no_mangle]
pub static mut foo_blob_type: PL_blob_t = PL_blob_t {
    magic: PL_BLOB_MAGIC as usize,
    flags: 0,
    name: b"foo\0" as *const u8 as *mut i8,
    acquire: None,
    release: None,
    compare: None,
    write: Some(foo_write),
    save: None,
    load: None,
    padding: 0,
    reserved: [0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void,0 as *mut ffi::c_void],
    registered: 0,
    rank: 0,
    next: 0 as *mut PL_blob_t,
    atom_name: 0
};


unsafe extern "C" fn foo_write(out: *mut io_stream, atom: usize, flags: i32) -> i32 {
    Sfprintf(out, b"<foooooooooooooooo>\0" as *const u8 as *const i8);

    1
}

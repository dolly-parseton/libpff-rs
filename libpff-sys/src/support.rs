//
mod wrapped_functions {
    //
    use crate::{enums::CodePage, libpff_error_t, libpff_get_codepage, libpff_get_version};
    use std::{convert::TryFrom, ffi::CStr, ptr, str};
    /// Returns the static version string.
    unsafe fn get_version() -> &'static str {
        CStr::from_ptr(libpff_get_version())
            .to_str()
            .expect("'get_version() failed to version string.")
    }
    /// Returns code page write.
    unsafe fn get_codepage() -> Result<CodePage, i32> {
        let codepage: *mut i32 = ptr::null_mut();
        let mut err: *mut libpff_error_t = ptr::null_mut();
        match libpff_get_codepage(codepage, &mut err) {
            0 => Err(0),
            -1 => Err(0), // TEMP, read error
            1 | _ => CodePage::try_from(*codepage),
        }
    }
}

//! Fixed PCP identity constants shared by the CNG mechanics modules.

#![cfg(windows)]

use windows_sys::core::PCWSTR;

pub(super) static FIXED_KEY_NAME_WIDE: [u16; 52] = [
    79, 99, 101, 110, 116, 114, 97, 80, 97, 114, 101, 110, 116, 46, 80, 114, 111, 116, 101, 99,
    116, 101, 100, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121,
    46, 83, 105, 103, 110, 105, 110, 103, 46, 118, 49, 0,
];
pub(super) const FIXED_KEY_NAME: PCWSTR = FIXED_KEY_NAME_WIDE.as_ptr();

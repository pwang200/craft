#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::ledger_objects::current_escrow::{get_current_escrow, CurrentEscrow};
use xrpl_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_std::host::{compute_sha512_half, update_data};
//
// //large array
// #[unsafe(no_mangle)]
// pub extern "C" fn finish() -> i32 {
//     let test_data = [0u8; 1024usize * 1024*64];
//     let mut hash_output = [0u8; 32];
//     let hash_result = unsafe {
//         compute_sha512_half(
//             test_data.as_ptr(),
//             test_data.len(),
//             hash_output.as_mut_ptr(),
//             hash_output.len(),
//         )
//     };
//
//     if hash_result != 32 {
//         return hash_result; // SHA512 half computation failed
//     }
//
//     1
// }

// //wrong pointer, but under memory limit
// #[unsafe(no_mangle)]
// pub extern "C" fn finish() -> i32 {
//     let test_data = [0u8; 1024usize];
//     let mut hash_output = [0u8; 32];
//     let hash_result = unsafe {
//         compute_sha512_half(
//             test_data.as_ptr().wrapping_add(1024),
//             test_data.len(),
//             hash_output.as_mut_ptr(),
//             hash_output.len(),
//         )
//     };
//
//     if hash_result != 32 {
//         return hash_result; // SHA512 half computation failed
//     }
//
//     1
// }

//update data correct
#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let test_data = [1u8; 1024usize];
    let res = unsafe {
        update_data(
            test_data.as_ptr(),
            test_data.len(),
        )
    };

    if res < 0 {
        res
    } else {
        let read_data = [0u8; 1024usize];
        let current_escrow: CurrentEscrow = get_current_escrow();
        match current_escrow.get_data() {
            xrpl_std::host::Result::Ok(read_data) => {
                if read_data.len == test_data.len() {
                    for i in 0..read_data.len {
                        if read_data.data[i] != test_data[i] {
                            return -1;
                        }
                    }
                    1
                } else {
                    -1
                }
            }
            xrpl_std::host::Result::Err(_) => {
                -1
            },
        }
    }
}

// //update data, wrong pointer
// #[unsafe(no_mangle)]
// pub extern "C" fn finish() -> i32 {
//     let test_data = [0u8; 1024usize];
//     let res = unsafe {
//         update_data(
//             test_data.as_ptr().wrapping_add(1024),
//             test_data.len(),
//         )
//     };
//
//     if res < 0 {
//         res
//     } else {
//         1
//     }
// }

// //update data, large array
// #[unsafe(no_mangle)]
// pub extern "C" fn finish() -> i32 {
//     let test_data = [0u8; 1024usize*5];
//     let res = unsafe {
//         update_data(
//             test_data.as_ptr(),
//             test_data.len(),
//         )
//     };
//
//     if res < 0 {
//         res
//     } else {
//         1
//     }
// }
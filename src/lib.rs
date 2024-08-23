
/*
MIT License

Copyright (c) 2024 ElectroMutex

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::os::raw::c_char;
use std::ffi::CStr;
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CKernel {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub code: *const c_char,
}

impl CKernel {
    pub fn code_as_string(&self) -> String {
        unsafe {
            if self.code.is_null() {
                String::new()
            } else {
                CStr::from_ptr(self.code)
                    .to_string_lossy()
                    .into_owned()
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CInfo {
    pub bind: u32,
    pub group: u32,
    pub data_len: usize,
    pub data: *mut u8,
}

impl CInfo {
    pub unsafe fn data_as_vec(&self) -> Option<Vec<u8>> {
        if self.data.is_null() {
            None
        } else {
            // Create a Vec<u8> that shares the memory but doesn't deallocate it.
            Some(Vec::from_raw_parts(self.data, self.data_len, self.data_len))
        }
    }
}
#[no_mangle]
pub extern "C" fn compute0 (kernel: CKernel, data0: CInfo, data1: CInfo) {
    let compute_kernel = core_compute::compute_kernel {
        x: kernel.x,
        y: kernel.y,
        z: kernel.z,
        code: kernel.code_as_string(),
    };let not_del0 = ManuallyDrop::new(unsafe { data0.data_as_vec().expect("NULL DATA")});
                let mut dat0 = core_compute::info{bind: data0.bind, group: data0.group, data: not_del0, };
let not_del1 = ManuallyDrop::new(unsafe { data1.data_as_vec().expect("NULL DATA")});
                let mut dat1 = core_compute::info{bind: data1.bind, group: data1.group, data: not_del1, };


core_compute::compute!(compute_kernel , &mut dat0, &mut dat1);
}

#[no_mangle]
pub extern "C" fn compute1 (kernel: CKernel, data0: CInfo, data1: CInfo) {
    let compute_kernel = core_compute::compute_kernel {
        x: kernel.x,
        y: kernel.y,
        z: kernel.z,
        code: kernel.code_as_string(),
    };let not_del0 = ManuallyDrop::new(unsafe { data0.data_as_vec().expect("NULL DATA")});
                let mut dat0 = core_compute::info{bind: data0.bind, group: data0.group, data: not_del0, };
let not_del1 = ManuallyDrop::new(unsafe { data1.data_as_vec().expect("NULL DATA")});
                let mut dat1 = core_compute::info{bind: data1.bind, group: data1.group, data: not_del1, };


core_compute::compute!(compute_kernel , &mut dat0, &mut dat1);
}



use std::env;
use std::fs::File;
use std::io::Write;
extern crate cbindgen;

fn main() {
    // lib.rs writer
    let mut structs = r#"
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
"#.to_string();


    
    let func_name = r#"#[no_mangle]
pub extern "C" fn compute"#.to_string();
    
    let para_list = r#"(kernel: CKernel"#.to_string();
    let fun_body = r#") {
    let compute_kernel = core_compute::compute_kernel {
        x: kernel.x,
        y: kernel.y,
        z: kernel.z,
        code: kernel.code_as_string(),
    };"#.to_string();

    let macro_call = r#"core_compute::compute!(compute_kernel "#.to_string();

    // Get the environment variable `NUM_CORE_COMPUTE_REQ`
    let req = env::var("NUM_CORE_COMPUTE_REQ").unwrap_or_else(|_| "2".to_string());
    println!("cargo:warning=NUM_CORE_COMPUTE_REQ is set to {}", req);
    let req = req.parse::<u32>().expect("cargo:error=NUM_CORE_COMPUTE_REQ is set to invalid value");
    
    let mut file_content = String::new();
    file_content += &mut structs;

    for index in 0..req {
        let env_var_name = format!("NUM_CORE_{}", index);
        let data_amount = env::var(&env_var_name).unwrap_or_else(|_| "2".to_string());
        let data_amount = data_amount.parse::<u32>().expect("cargo:error=NUM_CORE_COMPUTE_REQ is set to invalid value");
        let mut parameters_list: Vec<String> = Vec::new();
        for para in 0..data_amount {
            let parameter = format!("data{}", para);
            parameters_list.push(parameter);
        }
        let mut not_del_list: Vec<String> = Vec::new();
        for not_del in 0..data_amount {
            let para = format!("not_del{}", not_del);
            not_del_list.push(para);
        }

        // Function definition
        let mut function_def = format!("{}{} ", func_name, index);
        function_def.push_str(&para_list);
        for parameter in &parameters_list {
            let prot = format!(", {}: CInfo", parameter);
            function_def.push_str(&prot);
        }
        function_def.push_str(&fun_body);

        for para in 0..data_amount {
            let mut data_handle = String::new();
            {
                let not_del_static = r#".data_as_vec().expect("NULL DATA")});
                "#;
                let not_del_handle = format!(
                    "let {} = ManuallyDrop::new(unsafe {{ {}{}",
                    not_del_list[para as usize],
                    parameters_list[para as usize],
                    not_del_static
                );
                data_handle.push_str(&not_del_handle);
            }
            {
                let type_handle = format!(
                    "let mut dat{} = core_compute::info{{bind: {}.bind, group: {}.group, data: {}, }};",
                    para,
                    parameters_list[para as usize],
                    parameters_list[para as usize],
                    not_del_list[para as usize]
                );
                data_handle.push_str(&type_handle);
            }
            function_def.push_str(&data_handle);
            function_def.push_str("\n");
        }
        function_def.push_str("\n\n");
        let mut macro_paras = String::new();
        for para in 0..data_amount {
            let para_def = format!(", &mut dat{}" , para);
            macro_paras.push_str(&para_def);
        }
        let mut macro_def = String::new();
        macro_def.push_str(&macro_call);
        macro_def.push_str(&macro_paras);
        macro_def.push_str(");\n");
        function_def.push_str(&macro_def);
        function_def.push_str("}\n\n");  // Close the function body
        file_content.push_str(&function_def);
    }

    let mut file = File::create("src/lib.rs").expect("Unable to create log file");
    writeln!(file, "{}", file_content).expect("Unable to write data");

    // Generate the header file using cbindgen
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    config.include_guard = Some("CORE_COMPUTE_NATIVE_H".to_string());
    config.header = Some(r#"/*MIT License

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
SOFTWARE.*/"#.to_string());
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("target/core_compute_native.h");
}


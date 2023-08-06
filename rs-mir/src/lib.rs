#![allow(unused, dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::ffi::CString;

pub struct MIRContext {
    modules: Vec<MIRModule>,
}

impl MIRContext {
    pub fn init() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn emit_number(n: i32) {
        println!("ret {n}")
    }
}

pub struct MIRModule {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn module_test() {}
}

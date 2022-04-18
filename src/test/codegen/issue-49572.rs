// compile-flags: -O

#![crate_type = "rlib"]

#[no_mangle]
pub fn test(num: std::num::NonZeroI32) -> bool {
    // CHECK: ret i1 false
    num.get() == 0
}

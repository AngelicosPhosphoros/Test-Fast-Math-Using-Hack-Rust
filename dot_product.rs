// Use while and unsafe to generate less unoptimized IR
// to make manual changes in fast LLVM IR

#![crate_type="dylib"]

#[no_mangle]
pub extern "C" fn dot_product_fast(arr1: *const f64, arr2: *const f64, len: usize) -> f64 {
    let arr1 = unsafe{
        core::slice::from_raw_parts(arr1, len)
    };
    let arr2 = unsafe{
        core::slice::from_raw_parts(arr2, len)
    };
    let mut dot_p = 0.0;
    let mut i = 0;    
    while i < len {
        unsafe {
            let a = *arr1.get_unchecked(i);
            let b = *arr2.get_unchecked(i);
            dot_p += a*b;
            i+=1;
        }
    }
    dot_p
}

#[no_mangle]
pub extern "C" fn dot_product_slow(arr1: *const f64, arr2: *const f64, len: usize) -> f64 {
    let arr1 = unsafe{
        core::slice::from_raw_parts(arr1, len)
    };
    let arr2 = unsafe{
        core::slice::from_raw_parts(arr2, len)
    };
    let mut dot_p = 0.0;
    let mut i = 0;    
    while i < len {
        unsafe {
            let a = *arr1.get_unchecked(i);
            let b = *arr2.get_unchecked(i);
            dot_p += a*b;
            i+=1;
        }
    }
    dot_p
}


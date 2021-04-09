#![crate_type = "bin"]

use libloading::{Library, Symbol};

type DotProductFun = unsafe extern "C" fn(*const f64, *const f64, usize) -> f64;

fn load_functions() -> (
    Symbol<'static, DotProductFun>,
    Symbol<'static, DotProductFun>,
) {
    unsafe {
        let lib = Library::new("./dot_product.so").unwrap();
        let lib = Box::leak(Box::new(lib));
        let dot_product_fast = lib.get(b"dot_product_fast").unwrap();
        let dot_product_slow = lib.get(b"dot_product_slow").unwrap();
        (dot_product_fast, dot_product_slow)
    }
}

fn main() {
    const LIMIT: usize = 200000;
    let mut arr1 = vec![0.0; LIMIT];
    let mut arr2 = vec![0.0; LIMIT];
    for i in 0..LIMIT {
        arr1[i] = (i + 1) as f64;
        arr2[i] = (i + 1) as f64;
    }

    let (dot_product_fast, dot_product_slow) = load_functions();
    // eager loading of symbols
    unsafe{
        dot_product_fast(arr1.as_ptr(), arr2.as_ptr(), LIMIT);
        dot_product_slow(arr1.as_ptr(), arr2.as_ptr(), LIMIT);
    }
    
    let use_fast = {
        let args: std::collections::HashSet<_> = std::env::args().collect();
        args.contains("--fast")
    };
    if use_fast {
        println!("Running fast floating math");
    } else {
        println!("Running IEEE-754 floating math");
    }
    let now = std::time::Instant::now();
    let dot_p = unsafe {
        if use_fast {
            dot_product_fast(arr1.as_ptr(), arr2.as_ptr(), LIMIT)
        } else {
            dot_product_slow(arr1.as_ptr(), arr2.as_ptr(), LIMIT)
        }
    };
    println!("Elapsed {} microseconds", now.elapsed().as_micros());
    println!("Result: {}", dot_p);
}

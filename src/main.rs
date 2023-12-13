use std::collections::HashMap;

#[cxx::bridge]
mod ffi {

    struct Shared {
        k: u64,
        v: u64,
    }
    extern "Rust" {}

    unsafe extern "C++" {
        include!("candle-vllm-fix/src/cache_kernel.h");

        fn copy_block(vec: Vec<u64>);
        fn copy_block2(vec: Vec<Shared>);
    }
}

fn main() {
    let vec: Vec<u64> = vec![1, 2, 3, 4];
    ffi::copy_block(vec);

    let mut map: HashMap<u64, u64> = HashMap::new();
    map.insert(1, 11);
    map.insert(2, 22);
    map.insert(3, 33);

    let mut vec = Vec::new();
    for (key, value) in map.iter() {
        vec.push(ffi::Shared {
            k: key.clone(),
            v: value.clone(),
        });
    }
    ffi::copy_block2(vec);
}

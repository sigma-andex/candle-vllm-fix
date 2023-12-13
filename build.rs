// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cache_kernel.cc")
        .flag_if_supported("-std=c++14")
        .compile("candle-vllm-fix");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=src/blobstore.h");
}

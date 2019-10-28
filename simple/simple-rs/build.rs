use bindgen;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("rustc-env=LLVM_CONFIG_PATH=/usr/bin/llvm-config");

    let bindings = bindgen::builder()
        .header("../../godot_headers/gdnative_api_struct.gen.h")
        .clang_arg("-I../../godot_headers")
        .layout_tests(false)
        .blacklist_function("wcstold")
        .generate()
        .unwrap();

    bindings
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .unwrap()
}

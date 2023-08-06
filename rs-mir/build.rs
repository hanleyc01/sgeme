use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=./mir/");
    println!("cargo:rustc-link-lib=mir");
    println!("cargo:rerun-if-changed=mir.h");

    let bindings = bindgen::Builder::default()
        .header("./mir/mir.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings to MIR!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


fn main() {
    println!("cargo:rustc-link-lib=c/freetype/freetyped");
    println!("cargo:rerun-if-changed=c/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("c/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

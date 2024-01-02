
fn main() {
    println!("cargo:rustc-link-lib=dependencies/freetype/freetyped");
    println!("cargo:rerun-if-changed=dependencies/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("dependencies/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-return-if-changed=.");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=levenshtein");

    cc::Build::new()
        .file("lib/levenshtein.c")
        .out_dir(".")
        .compile("levenshtein.so");

    let bindings = bindgen::Builder::default()
        .header("lib/levenshtein.h")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("could not write bindings");
}

extern crate cc;

use std::env;

fn main() {
    let cgal_dir = env::var("CGAL_DIR").unwrap();
    let cgal_include_dir = format!("{}/include", cgal_dir);
    let cgal_lib_dir = format!("{}/lib", cgal_dir);

    cc::Build::new()
        .file("src/cgal_wrapper.cpp")
        .cpp(true)
        .include(cgal_include_dir)
        .compile("libcgal.a");

    println!("cargo:rustc-flags=-L {} -l CGAL_Core -l CGAL -l gmp", cgal_lib_dir);
}

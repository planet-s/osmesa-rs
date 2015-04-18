// osmesa-rs: Off-Screen Mesa bindings for Rust.
// The OSMesa library is available under the MIT license.
// These bindings are public domain.

extern crate pkg_config;


fn main () {
  match pkg_config::find_library("osmesa") {
    Ok(_) => { return; },
    Err(_) => { println!("cargo:rustc-link-lib=OSMesa"); },
  }
}

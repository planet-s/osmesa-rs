// osmesa-rs: Off-Screen Mesa bindings for Rust.
// The OSMesa library is available under the MIT license.
// These bindings are public domain.

#![crate_name="osmesa_sys"]
#![crate_type="lib"]

include!(concat!(env!("OUT_DIR"), "/osmesa.rs"));

// osmesa-rs: Off-Screen Mesa bindings for Rust.
// The OSMesa library is available under the MIT license.
// These bindings are public domain.

#![allow(non_snake_case)]
#![crate_name="osmesa_sys"]
#![crate_type="lib"]

#[macro_use]
extern crate shared_library;
extern crate libc;

use libc::{
  c_char,
  c_void,
};

pub type GLboolean = libc::c_uchar;
pub type GLenum = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLsizei = libc::c_int;


//
// functions
//


shared_library!(OsMesa, "libOSMesa.so",
  pub fn OSMesaColorClamp (enable: GLboolean),
  pub fn OSMesaCreateContext (format: GLenum, sharelist: OSMesaContext) -> OSMesaContext,
  pub fn OSMesaCreateContextExt (format: GLenum, depthBits: GLint, stencilBits: GLint, accumBits: GLint, sharelist: OSMesaContext) -> OSMesaContext,
  pub fn OSMesaDestroyContext (ctx: OSMesaContext),
  pub fn OSMesaGetColorBuffer (c: OSMesaContext, width: *mut GLint, height: *mut GLint, format: *mut GLint, buffer: *mut *mut c_void) -> GLboolean,
  pub fn OSMesaGetCurrentContext () -> OSMesaContext,
  pub fn OSMesaGetDepthBuffer (c: OSMesaContext, width: *mut GLint, height: *mut GLint, bytesPerValue: *mut GLint, buffer: *mut *mut c_void) -> GLboolean,
  pub fn OSMesaGetIntegerv (pname: GLint, value: *mut GLint),
  pub fn OSMesaGetProcAddress (funcName: *const c_char) -> OSMESAproc,
  pub fn OSMesaMakeCurrent (ctx: OSMesaContext, buffer: *mut c_void, _type: GLenum, width: GLsizei, height: GLsizei) -> GLboolean,
  pub fn OSMesaPixelStore (pname: GLint, value: GLint),
);


//
// types
//


// opaque structs
#[repr(C)] pub struct osmesa_context;

// types
pub type OSMesaContext = *mut osmesa_context;
pub type OSMESAproc = Option<unsafe extern "C" fn ()>;


//
// constants
//


// context formats
pub const OSMESA_BGRA: GLenum = 0x0001;
pub const OSMESA_ARGB: GLenum = 0x0002;
pub const OSMESA_BGR: GLenum = 0x0004;
pub const OSMESA_RGB_565: GLenum = 0x0005;
pub const OSMESA_COLOR_INDEX: GLenum = 0x1900;
pub const OSMESA_RGB: GLenum = 0x1907;
pub const OSMESA_RGBA: GLenum = 0x1908;

// OSMesaGetIntegerv
pub const OSMESA_WIDTH: GLint = 0x0020;
pub const OSMESA_HEIGHT: GLint = 0x0021;
pub const OSMESA_FORMAT: GLint = 0x0022;
pub const OSMESA_TYPE: GLint = 0x0023;
pub const OSMESA_MAX_WIDTH: GLint = 0x0024;
pub const OSMESA_MAX_HEIGHT: GLint = 0x0025;

// OSMesaPixelStore
pub const OSMESA_ROW_LENGTH: GLint = 0x0010;
pub const OSMESA_Y_UP: GLint = 0x0011;

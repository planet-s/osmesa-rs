// osmesa-rs: Off-Screen Mesa bindings for Rust.
// The OSMesa library is available under the MIT license.
// These bindings are public domain.

#![allow(unused_imports)]

extern crate pkg_config;

use std::fs::File;
use std::io::{
  BufRead,
  BufReader,
  Write,
};
use std::path::PathBuf;

const PACKAGE: &'static str = "osmesa";
const IN_SRC: &'static str = "src/osmesa.rs.in";
const OUT_SRC: &'static str = concat!(env!("OUT_DIR"), "/osmesa.rs");
const GUESS_LIBS: [&'static str; 1] = ["OSMesa"];


fn add_link_paths (link_paths: &[PathBuf]) {
  for link_path in link_paths.iter() {
    match link_path.to_str() {
      Some(s) => { println!("cargo:rustc-link-search=native={}", s); },
      None => { panic!("link path is not valid UTF-8"); },
    }
  }
}

fn add_libs<S: AsRef<str>> (libs: &[S]) {
  for lib in libs.iter() {
    println!("cargo:rustc-link-lib={}", lib.as_ref());
  }
}

fn configure_source<S: AsRef<str>> (in_path: &str, out_path: &str, libs: &[S]) {
  let in_file;
  match File::open(in_path) {
    Ok(f) => { in_file = f; },
    Err(e) => { panic!("can't open '{}': {}", in_path, e); },
  }

  let mut out_file;
  match File::create(out_path) {
    Ok(f) => { out_file = f; },
    Err(e) => { panic!("can't create '{}': {}", out_path, e); },
  }

  for line in BufReader::new(in_file).lines() {
    match line {
      Ok(line) => {
        if line == "//@LINK@" {
          for lib in libs.iter() {
            if let Err(e) = out_file.write_all(format!("#[link(name=\"{}\")]\n", lib.as_ref()).as_bytes()) {
              panic!("write error: {}", e);
            }
          }
        } else {
          if let Err(e) = out_file.write_all(format!("{}\n", line).as_bytes()) {
            panic!("write error: {}", e);
          }
        }
      },
      Err(e) => { panic!("read error: {}", e); },
    }
  }
}

fn configure_build<S: AsRef<str>> (link_paths: &[PathBuf], libs: &[S]) {
  add_link_paths(link_paths);
  if cfg!(feature="force-link") {
    add_libs(libs);
    if let Err(e) = std::fs::copy(IN_SRC, OUT_SRC) {
      panic!("can't copy '{}' to '{}': {}", IN_SRC, OUT_SRC, e);
    }
  } else {
    configure_source(IN_SRC, OUT_SRC, libs);
  }
}

fn main () {
  println!("DAFUQ");
  match pkg_config::find_library(PACKAGE) {
    Ok(l) => { configure_build(&l.link_paths[..], &l.libs[..]); },
    Err(_) => { configure_build(&[], &GUESS_LIBS[..]); },
  }
}

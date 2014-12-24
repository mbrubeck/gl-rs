#![feature(phase)]

//! Making sure that no warning is generated by code from generate_gl_bindings!
#![deny(warnings)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/test_no_warnings.rs"));

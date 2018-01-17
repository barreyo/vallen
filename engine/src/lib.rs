
#![allow(dead_code)]

#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

extern crate cgmath;
#[cfg(test)]
extern crate quickcheck;

mod terrain;

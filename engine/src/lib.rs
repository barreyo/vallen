
#![allow(dead_code)]

// Configure Clippy to run when testing
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

// Use QuickCheck only when testing
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

extern crate cgmath;
extern crate gfx_hal;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as dx12;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as vulkan;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as metal;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as gl;
extern crate petgraph;
extern crate winit;

#[cfg(test)]
extern crate quickcheck;

mod frame_time;
mod terrain;

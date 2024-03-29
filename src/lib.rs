#![feature(scoped_threads)]
pub use pipeline::color::Color;

pub mod algebra;
pub mod pipeline;
pub mod ray_tracing;
pub mod renderer;
pub mod window;

#[cfg(test)]
mod test;

pub mod alloc;
pub mod atom;
pub mod attr;
pub mod builder;
pub mod class;
pub mod clock;
pub mod error;
pub mod inlet;
pub mod method;
pub mod num;
pub mod object;
pub mod outlet;
pub mod slice;
pub mod symbol;
pub mod wrapper;

//re-exports
mod max;
pub use self::max::*;
pub use median_macros::*;

#[cfg(test)]
pub mod test;

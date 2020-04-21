mod base;
mod bitmatrix;
pub mod identity;
pub mod replicate;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

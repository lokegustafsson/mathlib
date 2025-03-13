#[macro_use]
mod core;

mod dense_polynomial;
mod int;
mod modulo;
mod primitive_int;

pub use core::{
    El, SAdd, SFusedMulAdd, SMul, SRem, SSub, Structure, Super2Structure, Super3Structure,
    SuperStructure,
};
pub use dense_polynomial::DensePolynomial;
pub use int::Int;
pub use modulo::Mod;
pub use primitive_int::{I, U};

pub use rug;

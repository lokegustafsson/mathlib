use crate::{SAdd, SFusedMulAdd, SMul, SRem, SSub, Structure};
use std::{borrow::Cow, fmt};

macro_rules! impl_for_primitive {
    ($S:ident<$SW:literal>, $sign:ident, $V:ty) => {
        impl Structure for $S<$SW> {
            type V = $V;
            fn fmt_v(&self, v: &Self::V, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{v}")
            }
        }
        impl std::fmt::Display for $S<$SW> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                if f.alternate() {
                    write!(f, ", ")?;
                }
                write!(f, "{}{}", stringify!($sign), $SW)
            }
        }
        impl SAdd for $S<$SW> {
            fn zero(&self) -> Cow<'_, Self::V> {
                Cow::Owned(0)
            }
            fn add(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
                Cow::Owned(*lhs + *rhs)
            }
        }
        impl SSub for $S<$SW> {
            fn negate(&self, v: &mut Cow<'_, Self::V>) {
                let v: &mut Self::V = v.to_mut();
                *v = 0 - *v;
            }
            fn sub(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
                Cow::Owned(*lhs - *rhs)
            }
        }
        impl SMul for $S<$SW> {
            fn one(&self) -> Cow<'_, Self::V> {
                Cow::Owned(1)
            }
            fn mul(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
                Cow::Owned(*lhs * *rhs)
            }
        }
        impl SRem for $S<$SW> {
            /// Returns nonnegative remainder
            fn rem(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
                Cow::Owned(Self::V::rem_euclid(*lhs, *rhs))
            }
        }
        impl SFusedMulAdd for $S<$SW> {}
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I<const W: usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U<const W: usize>;

impl_for_primitive!(I<8>, i, i8);
impl_for_primitive!(I<16>, i, i16);
impl_for_primitive!(I<32>, i, i32);
impl_for_primitive!(I<64>, i, i64);
impl_for_primitive!(I<128>, i, i128);

impl_for_primitive!(U<8>, u, u8);
impl_for_primitive!(U<16>, u, u16);
impl_for_primitive!(U<32>, u, u32);
impl_for_primitive!(U<64>, u, u64);
impl_for_primitive!(U<128>, u, u128);

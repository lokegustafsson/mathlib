use crate::{SAdd, SFusedMulAdd, SMul, SRem, SSub, Structure};
use rug::Complete;
use std::{borrow::Cow, fmt, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int;
impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(f, ", ")?;
        }
        write!(f, "int")
    }
}

impl Structure for Int {
    type V = rug::Integer;
    fn fmt_v(&self, v: &Self::V, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{v}")
    }
}

impl SAdd for Int {
    fn zero(&self) -> Cow<'_, Self::V> {
        Cow::Owned(rug::Integer::ZERO)
    }
    fn add(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        Cow::Owned((lhs.deref() + rhs.deref()).complete())
    }
}
impl SSub for Int {
    fn negate(&self, v: &mut Cow<'_, Self::V>) {
        let v: &mut Self::V = v.to_mut();
        *v = (0i32 - &*v).complete();
    }
    fn sub(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        Cow::Owned((lhs.deref() - rhs.deref()).complete())
    }
}
impl SMul for Int {
    fn one(&self) -> Cow<'_, Self::V> {
        Cow::Borrowed(rug::Integer::ONE)
    }
    fn mul(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        Cow::Owned((lhs.deref() * rhs.deref()).complete())
    }
}
impl SRem for Int {
    /// Returns nonnegative remainder
    fn rem(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        let lhs = lhs.into_owned();
        let rhs = rhs.into_owned();
        Cow::Owned(lhs.div_rem_euc(rhs).1)
    }
}
impl SFusedMulAdd for Int {
    fn fused_mul_add_ref(&self, acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V) {
        *acc += lhs * rhs
    }
}

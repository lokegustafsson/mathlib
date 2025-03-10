use crate::{EAdd, EFusedMulAdd, EMul, ERem, ESub, El, ElT};
use std::fmt;

macro_rules! impl_for_primitive {
    ($T:ty) => {
        impl ElT for $T {
            type V = Self;
            type TD = ();
            fn take(self) -> El<Self::V, Self::TD> {
                El { v: self, td: () }
            }
            fn of(el: El<Self::V, Self::TD>) -> Self {
                el.v
            }
            fn as_ref(&self) -> (&Self::V, &Self::TD) {
                (self, &())
            }
            fn as_mut(&mut self) -> (&mut Self::V, &Self::TD) {
                (self, &())
            }
            fn fmt_v(v: &Self::V, _: &(), f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{v}")
            }
            fn fmt_td(_: &(), _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                Ok(())
            }
        }
        impl EAdd for $T {
            fn zero(_td: &()) -> Self {
                0
            }
            fn add(lhs: Self, rhs: Self, _td: &()) -> Self {
                lhs + rhs
            }
        }
        impl ESub for $T {
            fn negate(v: &mut Self, _td: &()) {
                *v = 0 - *v;
            }
            fn sub(lhs: Self, rhs: Self, _td: &()) -> Self {
                lhs - rhs
            }
        }
        impl EMul for $T {
            fn one(_td: &()) -> Self {
                1
            }
            fn mul(lhs: Self, rhs: Self, _td: &()) -> Self {
                lhs * rhs
            }
        }
        impl ERem for $T {
            /// Returns nonnegative remainder
            fn rem(lhs: Self, rhs: Self, _td: &()) -> Self {
                Self::rem_euclid(lhs, rhs)
            }
        }
        impl EFusedMulAdd for $T {}
    };
}

impl_for_primitive!(i8);
impl_for_primitive!(i16);
impl_for_primitive!(i32);
impl_for_primitive!(i64);
impl_for_primitive!(i128);
impl_for_primitive!(isize);

impl_for_primitive!(u8);
impl_for_primitive!(u16);
impl_for_primitive!(u32);
impl_for_primitive!(u64);
impl_for_primitive!(u128);
impl_for_primitive!(usize);

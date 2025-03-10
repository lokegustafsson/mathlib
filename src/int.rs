use crate::{EAdd, EFusedMulAdd, EMul, ERem, ESub, El, ElT};
use rug::{Complete, Integer};
use std::fmt;

impl ElT for Integer {
    type V = Integer;
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
impl EAdd for Integer {
    fn zero(_td: &()) -> Integer {
        Integer::ZERO
    }
    fn add_ref(lhs: &Integer, rhs: &Integer, _td: &()) -> Integer {
        (lhs + rhs).complete()
    }
    fn add_ref_lhs(lhs: &Integer, rhs: Integer, _td: &()) -> Integer {
        lhs + rhs
    }
    fn add_ref_rhs(lhs: Integer, rhs: &Integer, _td: &()) -> Integer {
        lhs + rhs
    }
    fn add(lhs: Integer, rhs: Integer, _td: &()) -> Integer {
        lhs + rhs
    }
}
impl ESub for Integer {
    fn negate(x: &mut Integer, _td: &()) {
        *x *= -1;
    }
    fn sub_ref(lhs: &Integer, rhs: &Integer, _td: &()) -> Integer {
        (lhs - rhs).complete()
    }
    fn sub_ref_lhs(lhs: &Integer, rhs: Integer, _td: &()) -> Integer {
        lhs - rhs
    }
    fn sub_ref_rhs(lhs: Integer, rhs: &Integer, _td: &()) -> Integer {
        lhs - rhs
    }
    fn sub(lhs: Integer, rhs: Integer, _td: &()) -> Integer {
        lhs - rhs
    }
}
impl EMul for Integer {
    fn one(_td: &()) -> Integer {
        Integer::ONE.clone()
    }
    fn mul_ref(lhs: &Integer, rhs: &Integer, _td: &()) -> Integer {
        (lhs * rhs).complete()
    }
    fn mul_ref_lhs(lhs: &Integer, rhs: Integer, _td: &()) -> Integer {
        lhs * rhs
    }
    fn mul_ref_rhs(lhs: Integer, rhs: &Integer, _td: &()) -> Integer {
        lhs * rhs
    }
    fn mul(lhs: Integer, rhs: Integer, _td: &()) -> Integer {
        lhs * rhs
    }
}
impl ERem for Integer {
    fn rem_ref(lhs: &Integer, rhs: &Integer, _td: &()) -> Integer {
        lhs.div_rem_euc_ref(rhs).complete().1
    }
    fn rem(lhs: Integer, rhs: Integer, _td: &()) -> Integer {
        lhs.div_rem_euc(rhs).1
    }
}
impl EFusedMulAdd for Integer {
    fn fused_mul_add_ref(acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V, _td: &()) {
        *acc += lhs * rhs
    }
}

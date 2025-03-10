use crate::{EAdd, EFusedMulAdd, EMul, ERem, ESub, El, ElT, ElTLift};
use std::{fmt, mem};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mod<T: ElT + ERem> {
    v: T::V,
    mod_and_td: (T::V, T::TD),
}
impl<T: ERem> ElT for Mod<T> {
    type V = T::V;
    type TD = (T::V, T::TD);
    fn take(self) -> El<Self::V, Self::TD> {
        El {
            v: self.v,
            td: self.mod_and_td,
        }
    }
    fn of(el: El<Self::V, Self::TD>) -> Self {
        Self {
            v: el.v,
            mod_and_td: el.td,
        }
    }
    fn as_ref(&self) -> (&Self::V, &Self::TD) {
        (&self.v, &self.mod_and_td)
    }
    fn as_mut(&mut self) -> (&mut Self::V, &Self::TD) {
        (&mut self.v, &self.mod_and_td)
    }
    fn fmt_v(
        v: &Self::V,
        mod_and_td: &Self::TD,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        let (_, td) = mod_and_td;
        T::fmt_v(v, td, f)
    }
    fn fmt_td(mod_and_td: &Self::TD, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (mod_, td) = mod_and_td;
        write!(f, "(mod ")?;
        T::fmt_v(mod_, td, f)?;
        write!(f, ")")?;
        T::fmt_td(td, f)
    }
}
impl<T: ERem> ElTLift for Mod<T> {
    type Inner = T;
    fn lifted_from(inner: T, td: Self::TD) -> Self {
        let El { v, td: inner_td } = inner.take();
        assert_eq!(inner_td, td.1);
        Self { v, mod_and_td: td }
    }
}
impl<T: EAdd + ERem> std::ops::Add for Mod<T> {
    op_body_add!(Mod<T>);
}
impl<T: ESub + ERem> std::ops::Sub for Mod<T> {
    op_body_sub!(Mod<T>);
}
impl<T: EMul + ERem> std::ops::Mul for Mod<T> {
    op_body_mul!(Mod<T>);
}
impl<T: EAdd + ERem> std::ops::AddAssign<&Mod<T>> for Mod<T> {
    op_body_add_assign!(Mod<T>);
}
impl<T: ESub + ERem> std::ops::SubAssign<&Mod<T>> for Mod<T> {
    op_body_sub_assign!(Mod<T>);
}
impl<T: EMul + ERem> std::ops::MulAssign<&Mod<T>> for Mod<T> {
    op_body_mul_assign!(Mod<T>);
}

impl<T: ERem> Mod<T> {
    pub fn new(val: T, mod_: T) -> Self {
        let val = val.take();
        let mod_ = mod_.take();
        assert_eq!(val.td, mod_.td);
        Self {
            v: T::rem(val.v, mod_.v.clone(), &val.td),
            mod_and_td: (mod_.v, mod_.td),
        }
    }
}
impl<T: EAdd + ERem> EAdd for Mod<T> {
    fn zero((_, td): &Self::TD) -> Self::V {
        T::zero(td)
    }
    fn add_ref(lhs: &Self::V, rhs: &Self::V, (mod_, td): &Self::TD) -> Self::V {
        let sum: Self::V = T::add_ref(lhs, rhs, td);
        T::rem_ref_rhs(sum, mod_, td)
    }
    fn add_ref_lhs(lhs: &Self::V, rhs: Self::V, (mod_, td): &Self::TD) -> Self::V {
        let sum: Self::V = T::add_ref_lhs(lhs, rhs, td);
        T::rem_ref_rhs(sum, mod_, td)
    }
    fn add_ref_rhs(lhs: Self::V, rhs: &Self::V, (mod_, td): &Self::TD) -> Self::V {
        let sum: Self::V = T::add_ref_rhs(lhs, rhs, td);
        T::rem_ref_rhs(sum, mod_, td)
    }
    fn add(lhs: Self::V, rhs: Self::V, (mod_, td): &Self::TD) -> Self::V {
        let sum: Self::V = T::add(lhs, rhs, td);
        T::rem_ref_rhs(sum, mod_, td)
    }
}
impl<T: ESub + ERem> ESub for Mod<T> {
    fn negate(x: &mut Self::V, (mod_, td): &Self::TD) {
        *x = T::sub_ref_lhs(mod_, mem::take(x), td);
    }
    fn sub(lhs: Self::V, rhs: Self::V, (mod_, td): &Self::TD) -> Self::V {
        let diff: Self::V = T::sub(lhs, rhs, td);
        T::rem_ref_rhs(diff, mod_, td)
    }
}
impl<T: EMul + ERem> EMul for Mod<T> {
    fn one((mod_, td): &Self::TD) -> Self::V {
        let ret = T::one(td);
        assert_ne!(mod_, &ret);
        ret
    }
    fn mul(lhs: Self::V, rhs: Self::V, (mod_, td): &Self::TD) -> Self::V {
        let sum: Self::V = T::mul(lhs, rhs, td);
        T::rem_ref_rhs(sum, mod_, td)
    }
}
impl<T: EFusedMulAdd + ERem> EFusedMulAdd for Mod<T> {
    fn fused_mul_add_ref(acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V, (mod_, td): &Self::TD) {
        T::fused_mul_add_ref(acc, lhs, rhs, td);
        *acc = T::rem_ref_rhs(mem::take(acc), mod_, td)
    }
}

impl<T: ERem> fmt::Display for Mod<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Self::fmt_v(&self.v, &self.mod_and_td, f)?;
        write!(f, " ")?;
        Self::fmt_td(&self.mod_and_td, f)?;
        Ok(())
    }
}

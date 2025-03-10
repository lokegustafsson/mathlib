use crate::{EAdd, EFusedMulAdd, EMul, ESub, El, ElT, ElTLift};
use std::{fmt, mem, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DensePolynomial<T: EFusedMulAdd> {
    coeffs: Vec<T::V>,
    symb_and_td: (Arc<str>, T::TD),
}
impl<T: EFusedMulAdd> ElT for DensePolynomial<T> {
    type V = Vec<T::V>;
    type TD = (Arc<str>, T::TD);
    fn take(self) -> El<Self::V, Self::TD> {
        El {
            v: self.coeffs,
            td: self.symb_and_td,
        }
    }
    fn of(el: El<Self::V, Self::TD>) -> Self {
        Self {
            coeffs: el.v,
            symb_and_td: el.td,
        }
    }
    fn as_ref(&self) -> (&Self::V, &Self::TD) {
        (&self.coeffs, &self.symb_and_td)
    }
    fn as_mut(&mut self) -> (&mut Self::V, &Self::TD) {
        (&mut self.coeffs, &self.symb_and_td)
    }
    fn fmt_v(
        coeffs: &Self::V,
        symb_and_td: &Self::TD,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        let (symb, td) = symb_and_td;
        for (i, (deg, coeff)) in coeffs
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != T::zero(td))
            .rev()
            .enumerate()
        {
            if i != 0 {
                write!(f, " + ")?;
            }
            if deg == 0 || *coeff != T::one(td) {
                T::fmt_v(coeff, td, f)?
            }
            match deg {
                0 => {}
                1 => write!(f, "{symb}")?,
                _ => write!(f, "{symb}^{deg}")?,
            }
        }
        Ok(())
    }
    fn fmt_td(symb_and_td: &Self::TD, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        T::fmt_td(&symb_and_td.1, f)
    }
}
impl<T: EFusedMulAdd> ElTLift for DensePolynomial<T> {
    type Inner = T;
    fn lifted_from(inner: T, td: Self::TD) -> Self {
        let El { v, td: inner_td } = inner.take();
        assert_eq!(inner_td, td.1);
        Self {
            coeffs: vec![v],
            symb_and_td: td,
        }
    }
}
impl<T: EFusedMulAdd> std::ops::Add for DensePolynomial<T> {
    op_body_add!(DensePolynomial<T>);
}
impl<T: EFusedMulAdd + ESub> std::ops::Sub for DensePolynomial<T> {
    op_body_sub!(DensePolynomial<T>);
}
impl<T: EFusedMulAdd> std::ops::Mul for DensePolynomial<T> {
    op_body_mul!(DensePolynomial<T>);
}
impl<T: EFusedMulAdd> std::ops::AddAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_add_assign!(DensePolynomial<T>);
}
impl<T: EFusedMulAdd + ESub> std::ops::SubAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_sub_assign!(DensePolynomial<T>);
}
impl<T: EFusedMulAdd> std::ops::MulAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_mul_assign!(DensePolynomial<T>);
}

impl<T: EFusedMulAdd> DensePolynomial<T> {
    pub fn new_symb(symb: impl Into<Arc<str>>, td: &T::TD) -> Self {
        Self {
            coeffs: vec![T::zero(&td), T::one(&td)],
            symb_and_td: (symb.into(), td.clone()),
        }
    }
}
impl<T: EFusedMulAdd> EAdd for DensePolynomial<T> {
    fn zero(_: &Self::TD) -> Self::V {
        Vec::new()
    }
    fn add(lhs: Self::V, rhs: Self::V, (_, td): &Self::TD) -> Self::V {
        let (mut big, small) = if rhs.len() > lhs.len() {
            (rhs, lhs)
        } else {
            (lhs, rhs)
        };
        for (slot, item) in Iterator::zip(big.iter_mut(), small.iter()) {
            *slot = T::add_ref_rhs(mem::take(slot), item, td);
        }
        big
    }
}
impl<T: EFusedMulAdd + ESub> ESub for DensePolynomial<T> {
    fn negate(coeffs: &mut Self::V, (_, td): &Self::TD) {
        for c in coeffs {
            T::negate(c, td);
        }
    }
    fn sub(mut lhs: Self::V, mut rhs: Self::V, (_, td): &Self::TD) -> Self::V {
        if lhs.len() >= rhs.len() {
            for (lhs_i, rhs_i) in Iterator::zip(lhs.iter_mut(), rhs.iter()) {
                *lhs_i = T::sub_ref_rhs(mem::take(lhs_i), rhs_i, td);
            }
            lhs
        } else {
            for (lhs_i, rhs_i) in Iterator::zip(lhs.iter(), rhs.iter_mut()) {
                *rhs_i = T::sub_ref_lhs(lhs_i, mem::take(rhs_i), td);
            }
            for rhs_i in rhs.iter_mut().skip(lhs.len()) {
                T::negate(rhs_i, td);
            }
            rhs
        }
    }
}
impl<T: EFusedMulAdd> EMul for DensePolynomial<T> {
    fn one((_, td): &Self::TD) -> Self::V {
        vec![T::one(td)]
    }
    fn mul(lhs: Self::V, rhs: Self::V, (_, td): &Self::TD) -> Self::V {
        let (mut big, small) = if rhs.len() > lhs.len() {
            (rhs, lhs)
        } else {
            (lhs, rhs)
        };
        let n = big.len();
        let m = small.len();
        big.resize(n + m - 1, Default::default());
        for k in (0..(n + m - 1)).rev() {
            big[k] = T::mul_ref_rhs(mem::take(&mut big[k]), &small[0], td);
            for j in (usize::saturating_sub(k, n) + 1)..usize::min(m, k + 1) {
                let i = k - j;
                assert!(i < k);
                let (pre, post) = big.split_at_mut(k);
                T::fused_mul_add_ref(&mut post[0], &pre[i], &small[j], td);
            }
        }
        big
    }
}
impl<T: EFusedMulAdd> EFusedMulAdd for DensePolynomial<T> {
    fn fused_mul_add_ref(acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V, (_, td): &Self::TD) {
        let n = lhs.len();
        let m = rhs.len();
        acc.resize(usize::max(acc.len(), n + m - 1), Default::default());
        for i in 0..n {
            for j in 0..m {
                T::fused_mul_add_ref(&mut acc[i + j], &lhs[i], &rhs[j], td);
            }
        }
    }
}

impl<T: EFusedMulAdd> fmt::Display for DensePolynomial<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Self::fmt_v(&self.coeffs, &self.symb_and_td, f)?;
        write!(f, " ")?;
        Self::fmt_td(&self.symb_and_td, f)?;
        Ok(())
    }
}

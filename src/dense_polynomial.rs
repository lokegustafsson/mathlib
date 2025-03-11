use crate::{El, SAdd, SFusedMulAdd, SMul, SSub, Structure, SuperStructure};
use std::{borrow::Cow, fmt, mem};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DensePolynomial<S: Structure + SFusedMulAdd> {
    symbol: String,
    inner: S,
}
impl<S: SFusedMulAdd> Structure for DensePolynomial<S> {
    type V = Vec<S::V>;
    fn fmt_v(&self, coeffs: &Self::V, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (i, (deg, coeff)) in coeffs
            .iter()
            .enumerate()
            .filter(|(_, c)| *c != &*self.inner.zero())
            .rev()
            .enumerate()
        {
            if i != 0 {
                write!(f, " + ")?;
            }
            if deg == 0 || coeff != &*self.inner.one() {
                self.inner.fmt_v(coeff, f)?
            }
            match deg {
                0 => {}
                1 => write!(f, "{}", self.symbol)?,
                _ => write!(f, "{}^{deg}", self.symbol)?,
            }
        }
        Ok(())
    }
}
impl<S: SFusedMulAdd> std::fmt::Display for DensePolynomial<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(f, ", ")?;
        }
        write!(f, "poly {}{:#}", self.symbol, self.inner)
    }
}
impl<S: SFusedMulAdd> SuperStructure for DensePolynomial<S> {
    type Inner = S;
    fn lifted_from(&self, inner: El<'_, Self::Inner>) -> El<'_, Self> {
        let El { v, s } = inner;
        assert_eq!(&self.inner, s);
        El {
            v: Cow::Owned(vec![v.into_owned()]),
            s: &self,
        }
    }
}
/*
impl<T: SFusedMulAdd> std::ops::Add for DensePolynomial<T> {
    op_body_add!(DensePolynomial<T>);
}
impl<T: SFusedMulAdd + SSub> std::ops::Sub for DensePolynomial<T> {
    op_body_sub!(DensePolynomial<T>);
}
impl<T: SFusedMulAdd> std::ops::Mul for DensePolynomial<T> {
    op_body_mul!(DensePolynomial<T>);
}
impl<T: SFusedMulAdd> std::ops::AddAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_add_assign!(DensePolynomial<T>);
}
impl<T: SFusedMulAdd + SSub> std::ops::SubAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_sub_assign!(DensePolynomial<T>);
}
impl<T: SFusedMulAdd> std::ops::MulAssign<&DensePolynomial<T>> for DensePolynomial<T> {
    op_body_mul_assign!(DensePolynomial<T>);
}
*/

impl<S: SFusedMulAdd> DensePolynomial<S> {
    pub fn new_symb(symbol: impl AsRef<str>, inner: &S) -> Self {
        Self {
            symbol: symbol.as_ref().to_owned(),
            inner: inner.clone(),
        }
    }
    pub fn symb(&self) -> El<'_, Self> {
        El {
            v: Cow::Owned(vec![
                self.inner.zero().into_owned(),
                self.inner.one().into_owned(),
            ]),
            s: &self,
        }
    }
}
impl<S: SFusedMulAdd> SAdd for DensePolynomial<S> {
    fn zero(&self) -> Cow<'_, Self::V> {
        Cow::Owned(Vec::new())
    }
    fn add<'a>(&'a self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'a, Self::V> {
        let [lhs_cap, rhs_cap] = [&lhs, &rhs].map(|side| match side {
            Cow::Borrowed(_) => 0,
            Cow::Owned(v) => v.capacity(),
        });
        let (mut target, src) = if rhs_cap > lhs_cap {
            (rhs.into_owned(), &lhs)
        } else {
            (lhs.into_owned(), &rhs)
        };
        for (slot, item) in Iterator::zip(target.iter_mut(), src.iter()) {
            let mut slot2 = Cow::Owned(mem::take(slot));
            slot2 = self.inner.add(slot2, Cow::Borrowed(item));
            *slot = slot2.into_owned();
        }
        Cow::Owned(target)
    }
}
impl<S: SFusedMulAdd + SSub> SSub for DensePolynomial<S> {
    fn negate(&self, coeffs: &mut Cow<'_, Self::V>) {
        for c in coeffs.to_mut() {
            let mut item = Cow::Owned(mem::take(c));
            self.inner.negate(&mut item);
            *c = item.into_owned();
        }
    }
    fn sub(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        let [lhs_cap, rhs_cap] = [&lhs, &rhs].map(|side| match side {
            Cow::Borrowed(_) => 0,
            Cow::Owned(v) => v.capacity(),
        });
        if lhs_cap >= rhs_cap {
            let mut lhs = lhs.into_owned();
            for (lhs_i, rhs_i) in Iterator::zip(lhs.iter_mut(), rhs.iter()) {
                let mut lhs_ii = Cow::Owned(mem::take(lhs_i));
                lhs_ii = self.inner.sub(lhs_ii, Cow::Borrowed(rhs_i));
                *lhs_i = lhs_ii.into_owned();
            }
            Cow::Owned(lhs)
        } else {
            let mut rhs = rhs.into_owned();
            for (lhs_i, rhs_i) in Iterator::zip(lhs.iter(), rhs.iter_mut()) {
                let mut rhs_ii = Cow::Owned(mem::take(rhs_i));
                rhs_ii = self.inner.sub(Cow::Borrowed(lhs_i), rhs_ii);
                *rhs_i = rhs_ii.into_owned();
            }
            for rhs_i in rhs.iter_mut().skip(lhs.len()) {
                let mut rhs_ii = Cow::Owned(mem::take(rhs_i));
                self.inner.negate(&mut rhs_ii);
                *rhs_i = rhs_ii.into_owned();
            }
            Cow::Owned(rhs)
        }
    }
}
impl<S: SFusedMulAdd> SMul for DensePolynomial<S> {
    fn one(&self) -> Cow<'_, Self::V> {
        Cow::Owned(vec![self.inner.one().into_owned()])
    }
    fn mul(&self, lhs: Cow<'_, Self::V>, rhs: Cow<'_, Self::V>) -> Cow<'_, Self::V> {
        let [lhs_cap, rhs_cap] = [&lhs, &rhs].map(|side| match side {
            Cow::Borrowed(_) => 0,
            Cow::Owned(v) => v.capacity(),
        });
        let (mut target, src) = if lhs_cap >= rhs_cap {
            (lhs.into_owned(), &rhs)
        } else {
            (rhs.into_owned(), &lhs)
        };
        let n = target.len();
        let m = src.len();
        target.resize(n + m - 1, Default::default());
        for k in (0..(n + m - 1)).rev() {
            {
                let mut slot = Cow::Owned(mem::take(&mut target[k]));
                slot = self.inner.mul(slot, Cow::Borrowed(&src[0]));
                target[k] = slot.into_owned();
            }
            for j in (usize::saturating_sub(k, n) + 1)..usize::min(m, k + 1) {
                let i = k - j;
                assert!(i < k);
                let (pre, post) = target.split_at_mut(k);
                self.inner.fused_mul_add_ref(&mut post[0], &pre[i], &src[j]);
            }
        }
        Cow::Owned(target)
    }
}
impl<S: SFusedMulAdd> SFusedMulAdd for DensePolynomial<S> {
    fn fused_mul_add_ref(&self, acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V) {
        let n = lhs.len();
        let m = rhs.len();
        acc.resize(usize::max(acc.len(), n + m - 1), Default::default());
        for i in 0..n {
            for j in 0..m {
                self.inner
                    .fused_mul_add_ref(&mut acc[i + j], &lhs[i], &rhs[j]);
            }
        }
    }
}

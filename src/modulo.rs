use crate::{El, SAdd, SFusedMulAdd, SMul, SRem, SSub, Structure, SuperStructure};
use std::{borrow::Cow, fmt, mem};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mod<S: Structure + SRem> {
    mod_: S::V,
    inner: S,
}
impl<S: SRem> Structure for Mod<S> {
    type V = S::V;
    fn fmt_v(&self, v: &Self::V, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt_v(v, f)
    }
}
impl<S: SRem> std::fmt::Display for Mod<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if f.alternate() {
            write!(f, ", ")?;
        }
        write!(f, "mod ")?;
        self.inner.fmt_v(&self.mod_, f)?;
        write!(f, "{:#}", self.inner)
    }
}
impl<S: SRem> SuperStructure for Mod<S> {
    type Inner = S;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn lifted_from<'a>(&'a self, inner: El<'a, Self::Inner>) -> El<'a, Self> {
        let El { v, s } = inner;
        assert_eq!(&self.inner, s);
        El { v, s: self }
    }
}

impl<S: SRem> Mod<S> {
    pub fn new(mod_: El<'_, S>) -> Self {
        Self {
            mod_: mod_.v.into_owned(),
            inner: mod_.s.clone(),
        }
    }
}
impl<T: SAdd + SRem> SAdd for Mod<T> {
    fn zero(&self) -> Cow<'_, Self::V> {
        self.inner.zero()
    }
    fn add<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V> {
        self.inner
            .rem(self.inner.add(lhs, rhs), Cow::Borrowed(&self.mod_))
    }
}
impl<T: SSub + SRem> SSub for Mod<T> {
    fn negate<'a>(&'a self, x: &mut Cow<'a, Self::V>) {
        *x = self.inner.sub(Cow::Borrowed(&self.mod_), mem::take(x));
    }
    fn sub<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V> {
        self.inner
            .rem(self.inner.sub(lhs, rhs), Cow::Borrowed(&self.mod_))
    }
}
impl<T: SMul + SRem> SMul for Mod<T> {
    fn one(&self) -> Cow<'_, Self::V> {
        let ret = self.inner.one();
        assert_ne!(&self.mod_, &*ret);
        ret
    }
    fn mul<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V> {
        self.inner
            .rem(self.inner.mul(lhs, rhs), Cow::Borrowed(&self.mod_))
    }
}
impl<T: SFusedMulAdd + SRem> SFusedMulAdd for Mod<T> {
    fn fused_mul_add_ref(&self, acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V) {
        self.inner.fused_mul_add_ref(acc, lhs, rhs);

        let mut slot: Cow<'_, Self::V> = Cow::Owned(mem::take(acc));
        slot = self.inner.rem(slot, Cow::Borrowed(&self.mod_));
        *acc = slot.into_owned();
    }
}

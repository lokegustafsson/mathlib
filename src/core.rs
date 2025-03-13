use std::{borrow::Cow, fmt, mem};

pub trait ReqV: Default + Clone + Eq + std::fmt::Debug {}
impl<V: Default + Clone + Eq + std::fmt::Debug> ReqV for V {}

pub trait ReqS: Clone + Eq + std::fmt::Display + std::fmt::Debug {}
impl<S: Clone + Eq + std::fmt::Display + std::fmt::Debug> ReqS for S {}

#[derive(Debug, PartialEq, Eq)]
pub struct El<'a, S: Structure> {
    pub v: Cow<'a, S::V>,
    pub s: &'a S,
}

pub trait Structure: ReqS {
    type V: ReqV;
    fn fmt_v(&self, v: &Self::V, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
    fn el<'a>(&'a self, v: impl Into<Self::V>) -> El<'a, Self> {
        El {
            v: Cow::Owned(v.into()),
            s: self,
        }
    }
}
pub trait SuperStructure: Structure {
    type Inner: Structure;
    fn inner(&self) -> &Self::Inner;
    fn lifted_from<'a>(&'a self, inner: El<'a, Self::Inner>) -> El<'a, Self>;

    fn el1<'a>(&'a self, v: impl Into<<Self::Inner as Structure>::V>) -> El<'a, Self> {
        self.inner().el(v).lift(self)
    }
}
pub trait Super2Structure: SuperStructure<Inner: SuperStructure> {
    fn el2<'a>(
        &'a self,
        v: impl Into<<<Self::Inner as SuperStructure>::Inner as Structure>::V>,
    ) -> El<'a, Self> {
        self.inner().el1(v).lift(self)
    }
}
pub trait Super3Structure: SuperStructure<Inner: Super2Structure> {
    fn el3<'a>(
        &'a self,
        v: impl Into<
            <<<Self::Inner as SuperStructure>::Inner as SuperStructure>::Inner as Structure>::V,
        >,
    ) -> El<'a, Self> {
        self.inner().el2(v).lift(self)
    }
}
impl<S2: SuperStructure<Inner: SuperStructure>> Super2Structure for S2 {}
impl<S3: SuperStructure<Inner: SuperStructure<Inner: SuperStructure>>> Super3Structure for S3 {}

impl<'a, S: Structure> El<'a, S> {
    pub fn copy<'b>(&'b self) -> El<'b, S> {
        El {
            v: Cow::Borrowed(&*self.v),
            s: self.s,
        }
    }
    pub fn extend_lifetime<'b>(self, s: &'b S) -> El<'b, S> {
        assert_eq!(self.s, s);
        El {
            v: Cow::Owned(self.v.into_owned()),
            s,
        }
    }
    pub fn lift<S1: SuperStructure<Inner = S>>(self, s1: &'a S1) -> El<'a, S1> {
        s1.lifted_from(self)
    }
    pub fn lift2<S2: SuperStructure<Inner: SuperStructure<Inner = S>>>(
        self,
        s2: &'a S2,
    ) -> El<'a, S2> {
        self.lift(s2.inner()).lift(s2)
    }
    pub fn lift3<S3: SuperStructure<Inner: SuperStructure<Inner: SuperStructure<Inner = S>>>>(
        self,
        s3: &'a S3,
    ) -> El<'a, S3> {
        self.lift2(s3.inner()).lift(s3)
    }
    pub fn lift4<
        S4: SuperStructure<
            Inner: SuperStructure<Inner: SuperStructure<Inner: SuperStructure<Inner = S>>>,
        >,
    >(
        self,
        s4: &'a S4,
    ) -> El<'a, S4> {
        self.lift3(s4.inner()).lift(s4)
    }
}

impl<S: Structure> fmt::Display for El<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.s.fmt_v(&self.v, f)?;
        write!(f, " ({})", self.s)?;
        Ok(())
    }
}

// element binary operations traits

pub trait SAdd: Structure {
    fn zero(&self) -> Cow<'_, Self::V>;
    fn add<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V>;
}
pub trait SSub: Structure + SAdd {
    fn negate<'a>(&'a self, v: &mut Cow<'a, Self::V>);
    fn sub<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V>;
}
pub trait SMul: Structure {
    fn one(&self) -> Cow<'_, Self::V>;
    fn mul<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V>;
}
pub trait SRem: Structure {
    fn rem<'a>(&'a self, lhs: Cow<'a, Self::V>, rhs: Cow<'a, Self::V>) -> Cow<'a, Self::V>;
}
pub trait SFusedMulAdd: Structure + SAdd + SMul {
    fn fused_mul_add_ref(&self, acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V) {
        let mut slot = Cow::Owned(mem::take(acc));
        slot = self.add(slot, self.mul(Cow::Borrowed(lhs), Cow::Borrowed(rhs)));
        *acc = slot.into_owned();
    }
}

macro_rules! impl_op {
    ($StructTrait:ident, $OpTrait:path, $method:ident, $OpTraitAssign:path, $method_assign:ident) => {
        impl<'a, S: $StructTrait> $OpTrait for $crate::core::El<'a, S> {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                assert_eq!(self.s, rhs.s);
                $crate::core::El {
                    v: $crate::$StructTrait::$method(self.s, self.v, rhs.v),
                    s: self.s,
                }
            }
        }
        impl<'a, S: $StructTrait> $OpTraitAssign for $crate::core::El<'a, S> {
            fn $method_assign(&mut self, rhs: Self) {
                assert_eq!(self.s, rhs.s);
                self.v = $crate::$StructTrait::$method(self.s, ::std::mem::take(&mut self.v), rhs.v)
            }
        }
    };
}
impl_op!(
    SAdd,
    ::std::ops::Add,
    add,
    ::std::ops::AddAssign,
    add_assign
);
impl_op!(
    SSub,
    ::std::ops::Sub,
    sub,
    ::std::ops::SubAssign,
    sub_assign
);
impl_op!(
    SMul,
    ::std::ops::Mul,
    mul,
    ::std::ops::MulAssign,
    mul_assign
);
impl_op!(
    SRem,
    ::std::ops::Rem,
    rem,
    ::std::ops::RemAssign,
    rem_assign
);

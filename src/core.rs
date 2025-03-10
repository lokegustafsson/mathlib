use std::{fmt, mem};

pub trait ReqV: Default + Clone + Eq + std::fmt::Debug {}
impl<T: Default + Clone + Eq + std::fmt::Debug> ReqV for T {}

pub trait ReqTD: Clone + Eq + std::fmt::Debug {}
impl<T: Clone + Eq + std::fmt::Debug> ReqTD for T {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El<V: ReqV, TD: ReqTD> {
    pub v: V,
    pub td: TD,
}

pub trait ElT {
    type V: ReqV;
    type TD: ReqTD;
    fn take(self) -> El<Self::V, Self::TD>;
    fn of(el: El<Self::V, Self::TD>) -> Self;
    fn as_ref(&self) -> (&Self::V, &Self::TD);
    fn as_mut(&mut self) -> (&mut Self::V, &Self::TD);
    fn fmt_v(v: &Self::V, td: &Self::TD, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
    fn fmt_td(td: &Self::TD, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T: ElT> ElTExt for T {}
pub trait ElTExt: ElT + Sized {
    fn lift<U: ElTLift<Inner = Self>>(self, td: &U::TD) -> U {
        U::lifted_from(self, td.clone())
    }
    fn td(&self) -> &Self::TD {
        self.as_ref().1
    }
}
pub trait ElTLift: ElT {
    type Inner: ElT;
    fn lifted_from(inner: Self::Inner, td: Self::TD) -> Self;
}

// element binary operations traits

pub trait EAdd: ElT {
    fn zero(td: &Self::TD) -> Self::V;
    fn add_ref(lhs: &Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::add_ref_rhs(lhs.clone(), rhs, td)
    }
    fn add_ref_lhs(lhs: &Self::V, rhs: Self::V, td: &Self::TD) -> Self::V {
        Self::add(lhs.clone(), rhs, td)
    }
    fn add_ref_rhs(lhs: Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::add(lhs, rhs.clone(), td)
    }
    fn add(lhs: Self::V, rhs: Self::V, td: &Self::TD) -> Self::V;
}
pub trait ESub: ElT + EAdd {
    fn negate(v: &mut Self::V, td: &Self::TD);
    fn sub_ref(lhs: &Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::sub_ref_rhs(lhs.clone(), rhs, td)
    }
    fn sub_ref_lhs(lhs: &Self::V, rhs: Self::V, td: &Self::TD) -> Self::V {
        Self::sub(lhs.clone(), rhs, td)
    }
    fn sub_ref_rhs(lhs: Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::sub(lhs, rhs.clone(), td)
    }
    fn sub(lhs: Self::V, rhs: Self::V, td: &Self::TD) -> Self::V;
}
pub trait EMul: ElT {
    fn one(td: &Self::TD) -> Self::V;
    fn mul_ref(lhs: &Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::mul_ref_rhs(lhs.clone(), rhs, td)
    }
    fn mul_ref_lhs(lhs: &Self::V, rhs: Self::V, td: &Self::TD) -> Self::V {
        Self::mul(lhs.clone(), rhs, td)
    }
    fn mul_ref_rhs(lhs: Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::mul(lhs, rhs.clone(), td)
    }
    fn mul(lhs: Self::V, rhs: Self::V, td: &Self::TD) -> Self::V;
}
pub trait ERem: ElT {
    fn rem_ref(lhs: &Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::rem_ref_rhs(lhs.clone(), rhs, td)
    }
    fn rem_ref_lhs(lhs: &Self::V, rhs: Self::V, td: &Self::TD) -> Self::V {
        Self::rem(lhs.clone(), rhs, td)
    }
    fn rem_ref_rhs(lhs: Self::V, rhs: &Self::V, td: &Self::TD) -> Self::V {
        Self::rem(lhs, rhs.clone(), td)
    }
    fn rem(lhs: Self::V, rhs: Self::V, td: &Self::TD) -> Self::V;
}
pub trait EFusedMulAdd: ElT + EAdd + EMul {
    fn fused_mul_add_ref(acc: &mut Self::V, lhs: &Self::V, rhs: &Self::V, td: &Self::TD) {
        *acc = Self::add(mem::take(acc), Self::mul_ref(lhs, rhs, td), td);
    }
}

// std op helper macros

macro_rules! op_body {
    ($T:ty, $ETrait:path, $method:ident) => {
        type Output = Self;
        fn $method(self, rhs: Self) -> Self {
            let lhs = self.take();
            let rhs = rhs.take();
            assert_eq!(lhs.td, rhs.td);
            ElT::of($crate::core::El {
                v: <$T as $ETrait>::$method(lhs.v, rhs.v, &lhs.td),
                td: rhs.td,
            })
        }
    };
}
macro_rules! op_body_assign {
    ($T:ty, $ETrait:path, $method_assign:ident, $method_ref_rhs:ident) => {
        fn $method_assign(&mut self, rhs: &Self) {
            let (lhs_v, lhs_td) = self.as_mut();
            let (rhs_v, rhs_td) = rhs.as_ref();
            assert_eq!(lhs_td, rhs_td);
            *lhs_v = <$T as $ETrait>::$method_ref_rhs(::std::mem::take(lhs_v), rhs_v, rhs_td)
        }
    };
}

// std op

#[macro_export]
macro_rules! op_body_add {
    ($T:ty) => {
        op_body!($T, $crate::core::EAdd, add);
    };
}
#[macro_export]
macro_rules! op_body_sub {
    ($T:ty) => {
        op_body!($T, $crate::core::ESub, sub);
    };
}
#[macro_export]
macro_rules! op_body_mul {
    ($T:ty) => {
        op_body!($T, $crate::core::EMul, mul);
    };
}
#[macro_export]
macro_rules! op_body_rem {
    ($T:ty) => {
        op_body!($T, $crate::core::ERem, rem);
    };
}

// std op assign

#[macro_export]
macro_rules! op_body_add_assign {
    ($T:ty) => {
        op_body_assign!($T, $crate::core::EAdd, add_assign, add_ref_rhs);
    };
}
#[macro_export]
macro_rules! op_body_sub_assign {
    ($T:ty) => {
        op_body_assign!($T, $crate::core::ESub, sub_assign, sub_ref_rhs);
    };
}
#[macro_export]
macro_rules! op_body_mul_assign {
    ($T:ty) => {
        op_body_assign!($T, $crate::core::EMul, mul_assign, mul_ref_rhs);
    };
}
#[macro_export]
macro_rules! op_body_rem_assign {
    ($T:ty) => {
        op_body_assign!($T, $crate::core::ERem, rem_assign, rem_ref_rhs);
    };
}

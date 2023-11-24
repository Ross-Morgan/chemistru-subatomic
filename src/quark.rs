use crate::{traits::Fermion, force::StrongForce};

pub trait Quark: Fermion + StrongForce {}

struct Up;
struct Down;
struct Strange;
struct Charm;
struct Top;
struct Bottom;

struct AntiUp;
struct AntiDown;
struct AntiStrange;
struct AntiCharm;
struct AntiTop;
struct AntiBottom;

macro_rules! blank_impl {
    ($ty:ty, $($tr:ty),+ $(,)?) => {
        $(
            impl $tr for $ty {}
        )+
    };
}

macro_rules! subatomic_impl {
    ($ty:ty, $at:ty, $mass:literal) => {
        subatomic_single_impl!($ty, $mass);
        subatomic_single_impl!($at, $mass);
    };
}

macro_rules! subatomic_single_impl {
    ($ty:ty, $mass:literal) => {
        impl $crate::traits::SubAtomic for $ty {
            fn spin_quantum_number(&self) -> $crate::spin::Spin {
                $crate::spin::Spin::WholeHalf(0)
            }

            fn mass(&self) -> $crate::traits::MeVC2 { $mass }
        }
    }
}

subatomic_impl!(Up, AntiUp, 2.2);
subatomic_impl!(Down, AntiDown, 4.7);
subatomic_impl!(Charm, AntiCharm, 1280.0);
subatomic_impl!(Strange, AntiStrange, 96.0);
subatomic_impl!(Top, AntiTop, 173100.0);
subatomic_impl!(Bottom, AntiBottom, 4180.0);

blank_impl!(Up, Fermion, Quark);
blank_impl!(Down, Fermion, Quark);
blank_impl!(Charm, Fermion, Quark);
blank_impl!(Strange, Fermion, Quark);
blank_impl!(Top, Fermion, Quark);
blank_impl!(Bottom, Fermion, Quark);

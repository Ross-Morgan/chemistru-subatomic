#[macro_export]
macro_rules! blank_impl {
    ($ty:ty, $($tr:ty),+ $(,)?) => {
        $(
            impl $tr for $ty {}
        )+
    };
}


#[macro_export]
macro_rules! subatomic_impl {
    ($ty:ty, $at:ty, $mass:literal) => {
        $crate::subatomic_single_impl!($ty, $mass);
        $crate::subatomic_single_impl!($at, $mass);
    };
}

#[macro_export]
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
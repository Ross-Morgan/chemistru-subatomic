use crate::traits::{Boson, Fermion, SubAtomic};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Spin {
    Whole(i8),
    WholeHalf(i8),
}

pub trait RawSpin<const ADD_SPIN: bool>: SubAtomic {
    fn spin_quantum_number_raw(&self) -> f64;
}

impl<T: Fermion> RawSpin<true> for T {
    fn spin_quantum_number_raw(&self) -> f64 {
        if let Spin::WholeHalf(inner) = self.spin_quantum_number() {
            inner as f64 + 0.5
        } else {
            panic!("Fermion erroneously has integer spin")
        }
    }
}

impl<T: Boson> RawSpin<false> for T {
    fn spin_quantum_number_raw(&self) -> f64 {
        if let Spin::Whole(inner) = self.spin_quantum_number() {
            inner as f64
        } else {
            panic!("Boson erroneously has non-integer spin")
        }
    }
}

use crate::traits::SubAtomic;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spin(fraction::GenericFraction<u8>);


impl Spin {
    fn into_inner(self) -> f64 {
        let frac = self.spin_quantum_number().0;

        frac.numer().expect("Spin fraction has no numerator");
        frac.denom().expect("Spin fraction has no demonimator");
    }
}

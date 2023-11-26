use crate::{Boson, spin::RawSpin};

pub struct Gluon;
pub struct Photon;
pub struct Z0;
pub struct WPlus;
pub struct WMinus;

impl RawSpin for Gluon {
    fn spin_quantum_number_raw(&self) -> f64 {
        
    }
}

impl Boson for Gluon {

}
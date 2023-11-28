use crate::{Boson, blank_impl};

pub struct Gluon;
pub struct Photon;
pub struct Z0;
pub struct WPlus;
pub struct WMinus;

blank_impl!(Gluon, Boson);
blank_impl!(Photon, Boson);
blank_impl!(Z0, Boson);
blank_impl!(WPlus, Boson);
blank_impl!(WMinus, Boson);

use crate::{
    anti::{AntiMatter, Matter},
    quark::Quark,
    spin::RawSpin, force::StrongForce,
};

use super::spin::Spin;

/// MeV / c²
pub type MeVC2 = f64;

pub trait SubAtomic {
    fn spin_quantum_number(&self) -> Spin;
    fn mass(&self) -> MeVC2;
}

pub trait Fermion: RawSpin<true> {}

pub trait Boson: RawSpin<false> {}

pub trait Lepton: Fermion {}

pub trait Hadron: SubAtomic + StrongForce {
    type Composition;

    fn quark_composition(&self) -> Self::Composition;
}

pub trait Baryon<Q1, Q2, Q3>: Hadron<Composition = (Q1, Q2, Q3)>
where
    Q1: Quark,
    Q2: Quark,
    Q3: Quark,
{}

pub trait Meson<Q1, Q2>: Hadron<Composition = (Q1, Q2)>
where
    Q1: Quark + Matter,
    Q2: Quark + AntiMatter,
{}
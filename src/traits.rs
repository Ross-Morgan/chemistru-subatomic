use crate::{
    anti::{AntiMatter, Matter},
    decay::DecayEquation,
    force::StrongForce,
    quark::Quark,
};

use super::spin::Spin;

/// MeV / c²
pub type MeVC2 = f64;

pub trait SubAtomic {
    fn spin_quantum_number(&self) -> Spin;
    fn mass(&self) -> MeVC2;
}

pub trait Fermion {}

pub trait Boson {}

pub trait Lepton: Fermion {}

pub trait Hadron: SubAtomic + StrongForce {
    type Composition;
}

pub trait Baryon<Q1, Q2, Q3>: Hadron<Composition = (Q1, Q2, Q3)>
where
    Q1: Quark,
    Q2: Quark,
    Q3: Quark,
{
}

pub trait Meson<Q1, Q2>: Hadron<Composition = (Q1, Q2)>
where
    Q1: Quark + Matter,
    Q2: Quark + AntiMatter,
{
}

impl<T: Hadron<Composition = (Q1, Q2, Q3)>, Q1, Q2, Q3> Baryon<Q1, Q2, Q3> for T
where
    Q1: Quark,
    Q2: Quark,
    Q3: Quark,
{
}

impl<T: Hadron<Composition = (Q1, Q2)>, Q1, Q2> Meson<Q1, Q2> for T
where
    Q1: Quark + Matter,
    Q2: Quark + AntiMatter,
{
}

pub trait Decay: SubAtomic {
    fn all_possible_decays() -> Vec<DecayEquation>;
}

use crate::{anti::{Matter, AntiMatter}, quark, traits::Hadron, blank_impl, force::StrongForce, subatomic_single_impl};

pub struct Pion0;
pub struct PionMinus;
pub struct PionPlus;

impl AntiMatter for PionMinus {}
impl Matter for PionPlus {}

subatomic_single_impl!(Pion0, 497.611);
subatomic_single_impl!(PionMinus, 493.677);
subatomic_single_impl!(PionPlus, 493.677);

blank_impl!(Pion0, StrongForce);
blank_impl!(PionMinus, StrongForce);
blank_impl!(PionPlus, StrongForce);

impl Hadron for Pion0 {
    type Composition = ((quark::Up, quark::AntiUp), (quark::Down, quark::AntiDown));
}

impl Hadron for PionMinus {
    type Composition = (quark::Down, quark::AntiUp);
}

impl Hadron for PionPlus {
    type Composition = (quark::Up, quark::AntiDown);
}

impl ToString for Pion0 {
    fn to_string(&self) -> String {
        "π⁰".into()
    }
}

impl ToString for PionMinus {
    fn to_string(&self) -> String {
        "π⁻".into()
    }
}

impl ToString for PionPlus {
    fn to_string(&self) -> String {
        "π⁺".into()
    }
}

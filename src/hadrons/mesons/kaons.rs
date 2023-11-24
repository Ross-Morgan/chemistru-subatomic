use crate::{anti::{Matter, AntiMatter}, quark, traits::Hadron, blank_impl, force::StrongForce, subatomic_single_impl};

pub struct Kaon0;
pub struct KaonMinus;
pub struct KaonPlus;

impl AntiMatter for KaonMinus {}
impl Matter for KaonPlus {}

subatomic_single_impl!(Kaon0, 497.611);
subatomic_single_impl!(KaonMinus, 493.677);
subatomic_single_impl!(KaonPlus, 493.677);

blank_impl!(Kaon0, StrongForce);
blank_impl!(KaonMinus, StrongForce);
blank_impl!(KaonPlus, StrongForce);

impl Hadron for Kaon0 {
    type Composition = (quark::Down, quark::AntiStrange);
}

impl Hadron for KaonMinus {
    type Composition = (quark::Strange, quark::AntiUp);
}

impl Hadron for KaonPlus {
    type Composition = (quark::Up, quark::AntiStrange);
}

impl ToString for Kaon0 {
    fn to_string(&self) -> String {
        "K⁰".into()
    }
}

impl ToString for KaonMinus {
    fn to_string(&self) -> String {
        "K⁻".into()
    }
}

impl ToString for KaonPlus {
    fn to_string(&self) -> String {
        "K⁺".into()
    }
}
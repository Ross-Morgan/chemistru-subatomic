use crate::{traits::Fermion, force::StrongForce, anti::{AntiMatter, Matter}, blank_impl, subatomic_impl};

pub trait Quark: Fermion + StrongForce {}

pub struct Up;
pub struct Down;
pub struct Strange;
pub struct Charm;
pub struct Top;
pub struct Bottom;

pub struct AntiUp;
pub struct AntiDown;
pub struct AntiStrange;
pub struct AntiCharm;
pub struct AntiTop;
pub struct AntiBottom;

subatomic_impl!(Up, AntiUp, 2.2);
subatomic_impl!(Down, AntiDown, 4.7);
subatomic_impl!(Charm, AntiCharm, 1280.0);
subatomic_impl!(Strange, AntiStrange, 96.0);
subatomic_impl!(Top, AntiTop, 173100.0);
subatomic_impl!(Bottom, AntiBottom, 4180.0);

blank_impl!(Up, Fermion, StrongForce, Quark, Matter);
blank_impl!(Down, Fermion, StrongForce, Quark, Matter);
blank_impl!(Charm, Fermion, StrongForce, Quark, Matter);
blank_impl!(Strange, Fermion, StrongForce, Quark, Matter);
blank_impl!(Top, Fermion, StrongForce, Quark, Matter);
blank_impl!(Bottom, Fermion, StrongForce, Quark, Matter);

blank_impl!(AntiUp, Fermion, StrongForce, Quark, AntiMatter);
blank_impl!(AntiDown, Fermion, StrongForce, Quark, AntiMatter);
blank_impl!(AntiCharm, Fermion, StrongForce, Quark, AntiMatter);
blank_impl!(AntiStrange, Fermion, StrongForce, Quark, AntiMatter);
blank_impl!(AntiTop, Fermion, StrongForce, Quark, AntiMatter);
blank_impl!(AntiBottom, Fermion, StrongForce, Quark, AntiMatter);

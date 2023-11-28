pub mod anti;
pub mod force;
pub mod spin;
pub mod traits;
pub mod macros;

pub mod quark;

pub mod bosons;
pub mod fermions;
pub mod hadrons;
pub mod leptons;

use std::marker::PhantomData;

pub struct Electron;
pub struct Neutrino<T>(PhantomData<T>);
pub struct Kaon;
pub struct Pion;

pub trait Hadron {
    type Composition;
}

pub trait Meson: Hadron<Composition = [Quark; 2]> {}
pub trait Baryon: Hadron<Composition = [Quark; 3]> {}

pub trait Lepton {}

pub trait FundamentalParticle {
    fn charge(&self) {}
}

pub enum Quark {
    Up,
    Down,
    Top,
    Bottom,
    Strange,
    Charm,
    AntiUp,
    AntiDown,
    AntiTop,
    AntiBottom,
    AntiStrange,
    AntiCharm,
}

pub mod baryons {
    pub struct Proton;
    pub struct Neutron;
    pub struct Lambda;
    pub struct Omega;
}

pub mod mesons {
    pub enum Pion {
        Plus,
        Minus,
        Zero,
    }
    pub enum Kaon {
        Plus,
        Minus,
        Zero,
    }
    pub enum BZero {
        Plus,
        Minus,
        Zero,
    }

    pub enum EtaC {
        Plus,
        Minus,
        Zero,
    }
}

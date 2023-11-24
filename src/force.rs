use crate::quark::Quark;

pub trait StrongForce {}

impl<Q: Quark> StrongForce for Q {}
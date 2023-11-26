use crate::SubAtomic;

pub trait AntiMatter: SubAtomic {
    type ANTI: SubAtomic;
}

pub trait Matter {
    type ANTI: SubAtomic;
}

#[macro_export]
macro_rules! anti_particle_pair {
    ($t1:ty, $t2:ty) => {
        impl $crate::anti::Matter for $t1 {
            type ANTI = $t2;
        }

        impl $crate::anti::AntiMatter for $t2 {
            type ANTI = $t1;
        }
    };
}

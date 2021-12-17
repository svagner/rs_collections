use std::ops::AddAssign;

pub trait Incrementable: Copy + Default + AddAssign<Self> {
    fn increment(&mut self);
}

macro_rules! inc_int_impl {
    ($($t:ty)*) => ($(
        impl Incrementable for $t {
            fn increment(&mut self) {
                *self = (*self).checked_add(1).unwrap();
            }
        }
    )*)
}

inc_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

impl Incrementable for f32 {
    fn increment(&mut self) {
        *self += 1.0;
    }
}

impl Incrementable for f64 {
    fn increment(&mut self) {
        *self += 1.0;
    }
}

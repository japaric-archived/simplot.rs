pub trait Data {
    fn get(self) -> f64;
}

macro_rules! impl_data {
    ($ty:ty) => {
        impl Data for $ty {
            fn get(self) -> f64 {
                self.to_f64().unwrap()
            }
        }

        impl<'a> Data for &'a $ty {
            fn get(self) -> f64 {
                self.to_f64().unwrap()
            }
        }
    }
}

impl_data!(f32)
impl_data!(f64)
impl_data!(i16)
impl_data!(i32)
impl_data!(i64)
impl_data!(i8)
impl_data!(int)
impl_data!(u16)
impl_data!(u32)
impl_data!(u64)
impl_data!(u8)
impl_data!(uint)

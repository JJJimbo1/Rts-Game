use bevy::math::FloatExt;

pub trait T5FIntExt {
    fn pow_signum(self, n: u32) -> Self;
}

pub trait T5FFloatExt {
    fn deadzone(self, d: Self) -> Self;
    fn remap_clamped(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self;
    fn powf_signum(self, n: Self) -> Self;
    fn powi_signum(self, n: i32) -> Self;
}

macro_rules! impl_t5f_int_ext {
    ($type:ty) => {
        impl T5FIntExt for $type {
            #[inline]
            fn pow_signum(self, n: u32) -> Self {
                self.pow(n).abs() * self.signum()
            }
        }
    };
}

macro_rules! impl_t5f_float_ext {
    ($type:ty) => {
        impl T5FFloatExt for $type {
            #[inline]
            fn deadzone(self, d: Self) -> Self {
                if self.abs() > d { self } else { d }
            }

            #[inline]
            fn remap_clamped(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self {
                self.remap(in_start, in_end, out_start, out_end).clamp(out_start, out_end)
            }

            #[inline]
            fn powf_signum(self, n: Self) -> Self {
                self.powf(n).abs() * self.signum()
            }

            #[inline]
            fn powi_signum(self, n: i32) -> Self {
                self.powi(n).abs() * self.signum()
            }
        }
    };
}

impl_t5f_int_ext!(i8);
impl_t5f_int_ext!(i16);
impl_t5f_int_ext!(i32);
impl_t5f_int_ext!(i64);
impl_t5f_int_ext!(i128);

impl_t5f_float_ext!(f32);
impl_t5f_float_ext!(f64);
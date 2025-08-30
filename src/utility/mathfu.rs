use bevy::math::FloatExt;


pub trait T5FFloatExt {
    fn deadzone(self, d: Self) -> Self;
    fn remap_clamped(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self;
    fn powf_signum(self, n: Self) -> Self;
}

impl T5FFloatExt for f32 {
    #[inline]
    fn deadzone(self, d: Self) -> Self {
        if self.abs() > d {
            self
        } else {
            d
        }
    }

    #[inline]
    fn remap_clamped(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self {
        self.remap(in_start, in_end, out_start, out_end).clamp(out_start, out_end)
    }

    #[inline]
    fn powf_signum(self, n: Self) -> Self {
        self.powf(n).abs() * self.signum()
    }
}
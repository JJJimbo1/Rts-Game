pub mod d1 {
    pub fn approx(value : f32, value2 : f32) -> bool {
        (value - value2).abs() < 0.0001
    }

    pub fn approx_dif(value : f32, value2 : f32, dif : f32) -> bool {
        (value - value2).abs() < dif
    }

    pub fn farther_from_zero(value : f32, base : f32) -> bool {
        value.abs() > base.abs()
    }

    pub fn lerp(value : f32, target : f32, ramp : f32) -> f32 {
        value + (target - value) * ramp.clamp(0.0, 1.0)
    }

    pub fn more_than_or_zero(value : f32) -> f32 {
        if value > 0.0001 {
            value
        } else {
            0.0
        }
    }

    pub fn more_than_value_or_value(value : f32, min : f32, default : f32) -> f32 {
        if value > min {
            value
        } else {
            default
        }
    }

    pub fn more_than_value_or_zero(value : f32, min : f32) -> f32 {
        if value > min {
            value
        } else {
            0.0
        }
    }

    pub fn more_than_or_zero_pog(value : f32) -> f32 {
        if value.abs() > 0.0001 {
            value
        } else {
            0.0
        }
    }

    pub fn more_than_value_or_value_pog(value : f32, min : f32, default : f32) -> f32 {
        if value.abs() > min {
            value
        } else {
            default
        }
    }

    pub fn more_than_value_or_zero_pog(value : f32, min : f32) -> f32 {
        if value.abs() > min {
            value
        } else {
            0.0
        }
    }

    pub fn normalize_from_to(value : f32, from_min : f32, from_max : f32, to_min : f32, to_max : f32) -> f32 {
        (to_min.max(to_max) - to_min.min(to_max))
        * (value - from_min.min(from_max))
        / (from_min.max(from_max) - from_min.min(from_max))
        + to_min.min(to_max)
    }

    pub fn normalize_from_01(value : f32, min : f32, max : f32) -> f32 {
        value * (min.max(max) - min.min(max)) + min
    }

    pub fn normalize_to_01(value : f32, min : f32, max : f32) -> f32 {
        (value - min.min(max))/(min.max(max) - min.min(max))
    }

    pub fn powf_sign(value : f32, pow : f32) -> f32 {
        if value > 0.0 {
            value.abs().powf(pow)
        } else if value < 0.0 {
            -(value.abs().powf(pow))
        } else {
            value
        }
    }

    pub fn powi_sign(value : f32, pow : i32) -> f32 {
        if value > 0.0 {
            value.powi(pow)
        } else if value < 0.0 {
            -(value.abs().powi(pow))
        } else {
            value
        }
    }

    pub fn add_float_with_remainder(integer : &mut u64, float : &mut f32) {
        let remainder = *float % 1.0;
        *integer += (*float - remainder) as u64;
        *float = remainder;
    }
}

pub mod d2 {
    pub fn approx(value : [f32; 2], value2 : [f32; 2]) -> bool {
    (value[0] - value2[0]) < 0.0001
    && (value[1] - value2[1]) < 0.0001
    }

    pub fn approx_dif(value : [f32; 2], value2 : [f32; 2], dif : f32) -> bool {
        (value[0] - value2[0]) < dif
        && (value[1] - value2[1]) < dif
    }

    pub fn distance(this : (f32, f32), other : (f32, f32)) -> f32 {
        let a : f32 = (other.0 - this.0).powi(2);
        let b : f32 = (other.1 - this.1).powi(2);
        (a + b).sqrt()
    }

    pub fn distance_magnitude(this : (f32, f32), other : (f32, f32)) -> f32 {
        let a : f32 = (other.0 - this.0).powi(2);
        let b : f32 = (other.1 - this.1).powi(2);
        a + b
    }

    pub fn hypotenuse(side_a : f32, side_b : f32) -> f32 {
        (side_a.powi(2) + side_b.powi(2)).sqrt()
    }
}

pub mod d3 {
    #[inline]
    pub fn approx(value : [f32; 3], value2 : [f32; 3]) -> bool {
        (value[0] - value2[0]).abs() < 0.0001
        && (value[1] - value2[1]).abs() < 0.0001
        && (value[2] - value2[2]).abs() < 0.0001
    }

    #[inline]
    pub fn approx_dif(value : [f32; 3], value2 : [f32; 3], dif : f32) -> bool {
        (value[0] - value2[0]).abs() < dif
        && (value[1] - value2[1]).abs() < dif
        && (value[2] - value2[2]).abs() < dif
    }

    #[inline]
    pub fn cross_product(this : [f32; 3], other : [f32; 3]) -> [f32; 3] {
        [this[1] * other[2] - this[2] * other[1],
        this[2] * other[0] - this[0] * other[2],
        this[0] * other[1] - this[1] * other[0]]
    }

    #[inline]
    pub fn distance(this : (f32, f32, f32), other : (f32, f32, f32)) -> f32 {
        let a : f32 = (other.0 - this.0).powi(2);
        let b : f32 = (other.1 - this.1).powi(2);
        let c : f32 = (other.2 - this.2).powi(2);
        (a + b + c).sqrt()
    }

    #[inline]
    pub fn distance_magnitude(this : (f32, f32, f32), other : (f32, f32, f32)) -> f32 {
        let a : f32 = (other.0 - this.0).powi(2);
        let b : f32 = (other.1 - this.1).powi(2);
        let c : f32 = (other.2 - this.2).powi(2);
        a + b + c
    }

    #[inline]
    pub fn normalize(value : &mut [f32; 3]) {
        let len = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
        value[0] = value[0] / len;
        value[1] = value[1] / len;
        value[2] = value[2] / len;
    }
}

pub mod dx {
    pub fn distance_between(this : Vec<f32>, other : Vec<f32>) -> f32 {
        let mut distance_mag = 0.0;
        let (longest_vec, shortest_vec) = if other.len() > this.len() {
            (other, this)
        } else {
            (this, other)
        };

        for i in 0..longest_vec.len() {
            match shortest_vec.get(i) {
                Some(x) => {
                    distance_mag = distance_mag + (longest_vec[i] - *x).powi(2);
                },
                None => {
                    distance_mag = distance_mag + longest_vec[i].powi(2);
                }
            }
        };

        distance_mag.sqrt()
    }
}
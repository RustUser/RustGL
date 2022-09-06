pub mod color;
pub mod linear_algebra;
pub mod camera;

pub fn clamp_u32(value: u32, min: u32, max: u32) -> u32 {
    if value >= max {
        return max;
    } else if value <= min {
        return min;
    }
    value
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value >= max {
        return max;
    } else if value <= min {
        return min;
    }
    value
}

pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    if value >= max {
        return max;
    } else if value <= min {
        return min;
    }
    value
}


pub fn lerp(min: f32, max: f32, d: f32) -> f32 {
    min + (max - min) * d
}

pub fn lerp_f64(min: f64, max: f64, d: f64) -> f64 {
    min + (max - min) * d
}

pub fn inverse_lerp(min: f32, max: f32, l: f32) -> f32 {
    (l - min) / (max - min)
}

pub fn inverse_lerp_f64(min: f64, max: f64, l: f64) -> f64 {
    (l - min) / (max - min)
}
use core::f32::consts::PI;

/// 将角度规范化到 [0, 2π)
#[inline]
pub fn normalize_angle(mut angle: f32) -> f32 {
    let two_pi = 2.0 * PI;
    while angle < 0.0 {
        angle += two_pi;
    }
    while angle >= two_pi {
        angle -= two_pi;
    }
    angle
}

/// 将值限幅到 [min, max]
#[inline]
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// 符号函数
#[inline]
pub fn sign(val: f32) -> f32 {
    if val >= 0.0 { 1.0 } else { -1.0 }
}

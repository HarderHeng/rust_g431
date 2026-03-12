/// 电压（单位：V）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Voltage(pub f32);

/// 电流（单位：A）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Current(pub f32);

/// 电气角（单位：rad）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElecAngle(pub f32);

/// 机械角（单位：rad）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MechAngle(pub f32);

/// 转速（单位：rpm）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub f32);

/// PWM 占空比（0.0 ~ 1.0）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DutyCycle(pub f32);

impl DutyCycle {
    pub fn clamp(self) -> Self {
        Self(crate::common::math::clamp(self.0, 0.0, 1.0))
    }
}

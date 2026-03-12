/// 电机运行模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MotorState {
    Idle,
    OpenLoop,
    ClosedLoop,
    Fault,
}

/// 控制模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlMode {
    Torque,
    Velocity,
    Position,
}

/// FOC 单次控制周期输出的三相占空比（0.0 ~ 1.0）
#[derive(Debug, Clone, Copy)]
pub struct FocOutput {
    pub duty_a: f32,
    pub duty_b: f32,
    pub duty_c: f32,
}

impl FocOutput {
    pub const ZERO: Self = Self { duty_a: 0.5, duty_b: 0.5, duty_c: 0.5 };
}

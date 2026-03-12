use crate::common::types::{ControlMode, MotorState};

/// 电机控制指令（由 CLI / CAN 写入，由 foc_loop 读取）
pub struct MotorCommand {
    pub state: MotorState,
    pub mode: ControlMode,
    pub target: f32,
}

impl Default for MotorCommand {
    fn default() -> Self {
        Self {
            state: MotorState::Idle,
            mode: ControlMode::Velocity,
            target: 0.0,
        }
    }
}

// 后续通过 embassy_sync::mutex::Mutex<ThreadModeRawMutex, MotorCommand>
// 暴露为静态全局量，供各 task 读写

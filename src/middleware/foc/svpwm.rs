use crate::common::types::FocOutput;

/// 空间矢量脉宽调制（SVPWM）
///
/// 输入两相静止坐标参考电压 (Vα, Vβ) 和母线电压，输出三相占空比 [0.0, 1.0]
#[allow(dead_code)]
pub fn svpwm(_alpha: f32, _beta: f32, _vbus: f32) -> FocOutput {
    todo!()
}

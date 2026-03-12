/// PID 控制器骨架（电流环 / 速度环 / 位置环各实例化一个）
#[allow(dead_code)]
pub struct Pid {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

#[allow(dead_code)]
impl Pid {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self { kp, ki, kd }
    }

    /// 单步更新，返回控制输出
    pub fn update(&mut self, _error: f32, _dt: f32) -> f32 {
        todo!()
    }
}

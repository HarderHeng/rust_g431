use embassy_stm32::rcc::{
    Hse, HseMode, Pll, PllMul, PllPreDiv, PllRDiv, PllSource, Sysclk,
};
use embassy_stm32::time::Hertz;

/// 返回 embassy_stm32 初始化配置：
/// HSE 8MHz → PLL → 170MHz（Range1 Boost 模式）
///
/// PLL 计算：8MHz / 2(prediv) × 85(mul) / 2(divr) = 170MHz
pub fn configure() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });

    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL85,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });

    // 使用 PLL R 输出作为系统时钟
    config.rcc.sys = Sysclk::PLL1_R;

    // 170MHz 必须开启 Range1 Boost 模式，否则 Flash 延迟配置不正确
    config.rcc.boost = true;

    config
}

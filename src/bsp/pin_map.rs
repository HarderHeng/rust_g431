// ─── USART2 调试串口 ─────────────────────────────────────────────
/// USART2 TX 引脚：PB3（AF7）
pub type Usart2Tx = embassy_stm32::peripherals::PB3;
/// USART2 RX 引脚：PB4（AF7）
pub type Usart2Rx = embassy_stm32::peripherals::PB4;

// ─── TIM1 三相互补 PWM（预留）────────────────────────────────────
// pub type PwmUH = embassy_stm32::peripherals::PA8;   // TIM1_CH1
// pub type PwmUL = embassy_stm32::peripherals::PA7;   // TIM1_CH1N
// pub type PwmVH = embassy_stm32::peripherals::PA9;   // TIM1_CH2
// pub type PwmVL = embassy_stm32::peripherals::PB0;   // TIM1_CH2N
// pub type PwmWH = embassy_stm32::peripherals::PA10;  // TIM1_CH3
// pub type PwmWL = embassy_stm32::peripherals::PB1;   // TIM1_CH3N

// ─── TIM4 正交编码器（预留）──────────────────────────────────────
// pub type EncA = embassy_stm32::peripherals::PB6;    // TIM4_CH1
// pub type EncB = embassy_stm32::peripherals::PB7;    // TIM4_CH2

// ─── ADC 电流采样（预留）──────────────────────────────────────────
// pub type AdcIa = embassy_stm32::peripherals::PA0;   // ADC1_IN1
// pub type AdcIb = embassy_stm32::peripherals::PA1;   // ADC1_IN2

// ─── SPI 磁性编码器（预留）────────────────────────────────────────
// pub type SpiSck  = embassy_stm32::peripherals::PA5; // SPI1_SCK
// pub type SpiMiso = embassy_stm32::peripherals::PA6; // SPI1_MISO
// pub type SpiMosi = embassy_stm32::peripherals::PA7; // SPI1_MOSI
// pub type SpiNss  = embassy_stm32::peripherals::PA4; // SPI1_NSS (GPIO CS)

// ─── FDCAN（预留）────────────────────────────────────────────────
// pub type CanTx = embassy_stm32::peripherals::PA12;  // FDCAN1_TX
// pub type CanRx = embassy_stm32::peripherals::PA11;  // FDCAN1_RX

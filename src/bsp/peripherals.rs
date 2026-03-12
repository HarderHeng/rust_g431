use defmt::info;
use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart, Config};
use embassy_stm32::{bind_interrupts, peripherals};
use static_cell::StaticCell;

use crate::driver::serial::Serial;

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<peripherals::USART2>;
});

/// 板级外设聚合结构体，由 `init()` 组装后交给 app 层
pub struct BoardPeripherals {
    pub serial: Serial,
    // 后续扩展（按调试顺序取消注释）:
    // pub pwm:     driver::pwm::ThreePhasePwm,
    // pub adc:     driver::adc::CurrentSensor,
    // pub encoder: driver::encoder_timer::QepEncoder,
}

/// 完成 HAL 初始化并组装 BoardPeripherals：
/// 1. 调用 bsp::clock 获取时钟配置
/// 2. 调用 embassy_stm32::init 初始化 HAL，取得裸外设句柄
/// 3. 实例化各 driver，填充并返回 BoardPeripherals
pub fn init() -> BoardPeripherals {
    let p = embassy_stm32::init(crate::bsp::clock::configure());
    info!("STM32G431CBU6 @ 170MHz");
    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let mut usart_config = Config::default();
    usart_config.baudrate = 115200;
    info!("USART2 PB3(TX)/PB4(RX) @ 115200");

    // BufferedUart::new 参数顺序: (peri, rx, tx, tx_buffer, rx_buffer, irq, config)
    let usart = BufferedUart::new(
        p.USART2,
        p.PB4, // RX — AF7
        p.PB3, // TX — AF7
        TX_BUF.init([0u8; 256]),
        RX_BUF.init([0u8; 256]),
        Irqs,
        usart_config,
    )
    .unwrap();

    BoardPeripherals {
        serial: Serial::new(usart),
    }
}

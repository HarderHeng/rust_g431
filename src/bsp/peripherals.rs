use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart, Config};
use embassy_stm32::{bind_interrupts, peripherals, Peripherals};
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

/// 将 embassy 裸外设拆解并组装为 BoardPeripherals
pub fn init(p: Peripherals) -> BoardPeripherals {
    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let mut usart_config = Config::default();
    usart_config.baudrate = 115200;

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

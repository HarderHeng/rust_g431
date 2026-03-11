#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart, Config};
use embassy_stm32::{bind_interrupts, peripherals};
use static_cell::StaticCell;

mod cli;

// 引入 panic handler
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();

    // 配置 HSE 8MHz
    config.rcc.hse = Some(embassy_stm32::rcc::Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });

    // 配置 PLL: HSE 8MHz -> PLL -> 160MHz
    config.rcc.pll = Some(embassy_stm32::rcc::Pll {
        source: embassy_stm32::rcc::PllSource::HSE,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL20,
        divp: Some(embassy_stm32::rcc::PllPDiv::DIV2),
        divq: None,
        divr: None,
    });

    let p = embassy_stm32::init(config);

    info!("STM32G431 FOC initialized");

    // 配置 USART2: PB3 = TX, PB4 = RX, 115200 波特率
    let mut usart_config = Config::default();
    usart_config.baudrate = 115200;

    static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let usart = BufferedUart::new(
        p.USART2,
        p.PB4,  // RX
        p.PB3,  // TX
        TX_BUF.init([0u8; 256]),
        RX_BUF.init([0u8; 256]),
        Irqs,
        usart_config,
    ).unwrap();

    info!("USART2 initialized on PB3/PB4 @ 115200 baud");

    // 运行 CLI
    cli::run(usart).await;
}
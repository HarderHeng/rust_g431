#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::time::Hertz;
use defmt_rtt as _;

mod cli;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();

    // 配置 HSE 8MHz
    config.rcc.hse = Some(embassy_stm32::rcc::Hse {
        freq: Hertz(8_000_000),
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

    // 配置外设
    let p = embassy_stm32::init(config);

    // 配置 USART2: PB3 = TX, PB4 = RX, 1843200 波特率
    let mut usart_config = embassy_stm32::usart::Config::default();
    usart_config.baudrate = 115200;

    // 使用阻塞 UART (PB3=TX, PB4=RX)
    let mut usart = embassy_stm32::usart::Uart::new_blocking(
        p.USART2,
        p.PB4,
        p.PB3,
        usart_config,
    ).unwrap();
    let welcome_msg = b"Welcome to G431 CLI\r\n";
    usart.blocking_write(welcome_msg).unwrap();
    // 运行 CLI
    cli::run(usart).await;
}

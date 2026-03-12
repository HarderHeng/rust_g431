#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

mod app;
mod bsp;
mod common;
mod driver;
mod middleware;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // bsp::clock — 配置 HSE 8MHz → PLL → 170MHz（Range1 Boost）
    let config = bsp::clock::configure();
    let p = embassy_stm32::init(config);
    info!("STM32G431CBU6 initialized @ 170MHz");

    // bsp::peripherals — 拆解裸外设，组装板级结构体
    let board = bsp::peripherals::init(p);
    info!("Board peripherals initialized");

    // app::tasks — 启动各异步任务
    app::tasks::cli_task::run(board.serial).await;
}

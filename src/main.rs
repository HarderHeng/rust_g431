#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

mod app;
mod bsp;
mod common;
mod driver;
mod middleware;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = bsp::peripherals::init();
    app::run(board).await;
}

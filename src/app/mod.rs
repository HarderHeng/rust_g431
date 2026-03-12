pub mod cli;
pub mod state;
pub mod tasks;

use crate::bsp::peripherals::BoardPeripherals;

/// 应用层统一入口：接收板级外设，启动所有异步任务
pub async fn run(board: BoardPeripherals) {
    tasks::cli_task::run(board.serial).await;
}

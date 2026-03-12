use embedded_cli::cli::CliBuilder;
use embedded_io_async::Read as AsyncRead;
use static_cell::StaticCell;

use crate::app::cli::commands;
use crate::app::cli::writer::UartWriter;
use crate::driver::serial::Serial;

/// CLI Shell 任务入口
///
/// 接收 `Serial` 外设，拆分出 TX/RX，构建 embedded-cli 实例，
/// 在异步循环中逐字节处理输入。
pub async fn run(serial: Serial) {
    let (tx, mut rx) = serial.split();

    static CMD_BUF: StaticCell<[u8; 128]> = StaticCell::new();
    static HIST_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let mut cli = CliBuilder::default()
        .writer(UartWriter(tx))
        .command_buffer(CMD_BUF.init([0u8; 128]).as_mut_slice())
        .history_buffer(HIST_BUF.init([0u8; 256]).as_mut_slice())
        .prompt("G431> ")
        .build()
        .unwrap();

    let mut byte = [0u8; 1];

    loop {
        match rx.read(&mut byte).await {
            Ok(0) => continue,
            Ok(_) => commands::process(&mut cli, byte[0]),
            Err(_) => break,
        }
    }
}

use embassy_stm32::usart::BufferedUart;
use embedded_io_async::{Read, Write};
use heapless::String;
use core::fmt::Write as FmtWrite;

const CMD_BUF_SIZE: usize = 128;

// 定义 CLI 命令
enum Command<'a> {
    Help,
    Status,
    Clear,
    Speed(u16),
    Position,
    Velocity,
    History,
    Set { param: &'a str, value: u32 },
    Get { param: &'a str },
    Unknown,
}

pub async fn run(usart: BufferedUart<'static>) {
    let (mut tx, mut rx) = usart.split();

    // 发送欢迎信息
    let _ = tx.write_all(b"\r\nSTM32G431 FOC Shell\r\n").await;
    let _ = tx.write_all(b"Type 'help' for commands\r\n").await;

    let mut cmd_buf: String<CMD_BUF_SIZE> = String::new();
    let mut byte = [0u8; 1];

    let _ = write_prompt(&mut tx).await;

    loop {
        match rx.read(&mut byte).await {
            Ok(0) => continue,
            Ok(_) => {
                match byte[0] {
                    b'\r' | b'\n' => {
                        // 回车 - 处理命令
                        let _ = tx.write_all(b"\r\n").await;

                        if !cmd_buf.is_empty() {
                            let cmd = parse_command(cmd_buf.as_str());
                            handle_command(&mut tx, &cmd).await;
                            cmd_buf.clear();
                        }

                        let _ = write_prompt(&mut tx).await;
                    }
                    0x08 | 0x7F => {
                        // 退格
                        if cmd_buf.pop().is_some() {
                            let _ = tx.write_all(b"\x08 \x08").await;
                        }
                    }
                    b if b >= 0x20 && b <= 0x7E => {
                        // 可打印字符
                        if cmd_buf.push(b as char).is_ok() {
                            let _ = tx.write_all(&[b]).await;
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => break,
        }
    }
}

async fn write_prompt(tx: &mut embassy_stm32::usart::BufferedUartTx<'static>) -> Result<(), embassy_stm32::usart::Error> {
    tx.write_all(b"G431> ").await
}

fn parse_command(input: &str) -> Command<'_> {
    let input = input.trim();
    let parts: heapless::Vec<&str, 4> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["help"] => Command::Help,
        ["status"] => Command::Status,
        ["clear"] => Command::Clear,
        ["speed", val] => {
            if let Ok(v) = val.parse::<u16>() {
                Command::Speed(v)
            } else {
                Command::Unknown
            }
        }
        ["position"] => Command::Position,
        ["velocity"] => Command::Velocity,
        ["history"] => Command::History,
        ["set", param, val] => {
            if let Ok(v) = val.parse::<u32>() {
                Command::Set { param, value: v }
            } else {
                Command::Unknown
            }
        }
        ["get", param] => Command::Get { param },
        [] => Command::Unknown,
        _ => Command::Unknown,
    }
}

async fn handle_command(tx: &mut embassy_stm32::usart::BufferedUartTx<'static>, cmd: &Command<'_>) {
    match cmd {
        Command::Help => {
            let _ = tx.write_all(
                b"Available commands:\r\n\
                  help           - Show this help\r\n\
                  status         - Show system status\r\n\
                  clear          - Clear screen\r\n\
                  speed <0-100>  - Set motor speed\r\n\
                  position       - Get motor position\r\n\
                  velocity       - Get motor velocity\r\n\
                  history        - Show command history\r\n\
                  set <p> <v>    - Set parameter\r\n\
                  get <p>        - Get parameter\r\n"
            ).await;
        }
        Command::Status => {
            let _ = tx.write_all(
                b"System: STM32G431 FOC\r\n\
                  Clock: 160MHz\r\n\
                  UART: 115200 baud\r\n\
                  HSE: 8MHz\r\n\
                  FOC: Ready\r\n"
            ).await;
        }
        Command::Clear => {
            let _ = tx.write_all(b"\x1B[2J\x1B[H").await;
        }
        Command::Speed(value) => {
            if *value <= 100 {
                let mut resp: String<32> = String::new();
                write!(&mut resp, "Speed set to {}\r\n", value).ok();
                let _ = tx.write_all(resp.as_bytes()).await;
            } else {
                let _ = tx.write_all(b"Error: Speed must be 0-100\r\n").await;
            }
        }
        Command::Position => {
            let _ = tx.write_all(b"Position: 0\r\n").await;
        }
        Command::Velocity => {
            let _ = tx.write_all(b"Velocity: 0 rpm\r\n").await;
        }
        Command::History => {
            let _ = tx.write_all(b"Use up/down arrows (not implemented)\r\n").await;
        }
        Command::Set { param, value } => {
            let mut resp: String<64> = String::new();
            write!(&mut resp, "Set {} = {}\r\n", param, value).ok();
            let _ = tx.write_all(resp.as_bytes()).await;
        }
        Command::Get { param } => {
            let mut resp: String<64> = String::new();
            write!(&mut resp, "Get {}: 0\r\n", param).ok();
            let _ = tx.write_all(resp.as_bytes()).await;
        }
        Command::Unknown => {
            // 空输入时不显示错误
        }
    }
}
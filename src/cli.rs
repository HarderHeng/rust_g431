use embassy_stm32::usart::Uart;
use embassy_stm32::mode::Blocking;
use heapless::{String, Vec};
use embedded_hal_nb::serial::{Read, Write as NbWrite};
use nb::block;

const CMD_BUF_SIZE: usize = 128;
const HIST_SIZE: usize = 16;

pub async fn run(mut usart: Uart<'static, Blocking>) {
    let mut state = CliState::new();

    // 打印欢迎信息
    write_str(&mut usart, "STM32G431 FOC Shell\r\n").ok();
    write_str(&mut usart, "Type 'help' for commands\r\n\r\n").ok();
    write_prompt(&mut usart).ok();

    // CLI 主循环
    loop {
        // 读取一个字节 - 使用阻塞读取
        match block!(usart.read()) {
            Ok(byte) => {
                // 回显
                block!(usart.write(byte)).ok();

                // 处理字符
                match byte {
                    b'\r' => {
                        // 回车
                        block!(usart.write(b'\n')).ok();

                        if !state.cmd_buf.is_empty() {
                            let cmd_str: String<CMD_BUF_SIZE> = core::clone::Clone::clone(&state.cmd_buf);
                            let response = process_command(cmd_str.as_str());
                            state.add_to_history(cmd_str.as_str());

                            write_str(&mut usart, response).ok();

                            state.cmd_buf.clear();
                        }
                        write_prompt(&mut usart).ok();
                    }
                    b'\n' => {
                        // 忽略换行
                    }
                    b'\x08' | b'\x7f' => {
                        // 退格
                        if state.cmd_buf.pop().is_some() {
                            block!(usart.write(b' ')).ok();
                            block!(usart.write(b'\x08')).ok();
                        }
                    }
                    _ if byte >= 32 && byte <= 126 => {
                        if state.cmd_buf.push(byte as char).is_err() {
                            write_str(&mut usart, "\r\nBuffer overflow\r\n").ok();
                            state.cmd_buf.clear();
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => break,
        }
    }
}

fn write_str<W: NbWrite>(usart: &mut W, s: &str) -> Result<(), W::Error> {
    for &byte in s.as_bytes() {
        block!(usart.write(byte))?;
    }
    Ok(())
}

fn write_prompt<W: NbWrite>(usart: &mut W) -> Result<(), W::Error> {
    write_str(usart, "STM32G431 FOC> ")
}

// 命令处理函数
fn process_command(cmd: &str) -> &'static str {
    match cmd.trim() {
        "help" => {
            "Available commands:\r\n\
             help     - Show this help\r\n\
             status   - Show system status\r\n\
             clear    - Clear screen\r\n\
             speed    - Set motor speed (0-100)\r\n\
             position - Get motor position\r\n\
             velocity - Get motor velocity\r\n\
             history  - Show command history\r\n\
             set      - Set parameter (set <param> <value>)\r\n\
             get      - Get parameter (get <param>)\r\n"
        }
        "status" => {
            "System: STM32G431 FOC\r\n\
             Clock: 160MHz\r\n\
             UART: 1843200 baud\r\n\
             HSE: 8MHz\r\n\
             FOC: Ready\r\n"
        }
        "clear" => "\x1B[2J\x1B[H",
        "position" => "Position: 0\r\n",
        "velocity" => "Velocity: 0 rpm\r\n",
        "history" => "History: (not shown - use up/down)\r\n",
        s if s.starts_with("speed ") => {
            if let Ok(val) = s[6..].trim().parse::<u16>() {
                if val <= 100 {
                    return "Speed updated\r\n";
                }
            }
            "Usage: speed <0-100>\r\n"
        }
        s if s.starts_with("set ") => "Set parameter (not implemented)\r\n",
        s if s.starts_with("get ") => "Get parameter (not implemented)\r\n",
        "" => "",
        _ => "Unknown command. Type 'help' for available commands.\r\n",
    }
}

// CLI 状态
struct CliState {
    cmd_buf: String<CMD_BUF_SIZE>,
    history: Vec<String<CMD_BUF_SIZE>, HIST_SIZE>,
}

impl CliState {
    fn new() -> Self {
        Self {
            cmd_buf: String::new(),
            history: Vec::new(),
        }
    }

    fn add_to_history(&mut self, cmd: &str) {
        if cmd.is_empty() {
            return;
        }
        if let Some(last) = self.history.last() {
            if last == cmd {
                return;
            }
        }
        let mut new_cmd: String<CMD_BUF_SIZE> = String::new();
        if new_cmd.push_str(cmd).is_ok() {
            if self.history.push(new_cmd).is_err() {
                self.history.rotate_left(1);
                let _ = self.history.pop();
                let mut retry_cmd: String<CMD_BUF_SIZE> = String::new();
                let _ = retry_cmd.push_str(cmd);
                let _ = self.history.push(retry_cmd);
            }
        }
    }
}

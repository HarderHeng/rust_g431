use embassy_stm32::usart::BufferedUart;
use embedded_io_async::Read as AsyncRead;
use static_cell::StaticCell;

use embedded_cli::cli::CliBuilder;
use embedded_cli::Command;

// embedded-cli 使用 embedded_io 0.6，而 embassy-stm32 实现的是 embedded_io 0.7
// 需要一个 newtype 适配器将 0.7 的 Write 适配为 0.6 的 Write
struct UartWriter(embassy_stm32::usart::BufferedUartTx<'static>);

impl embedded_io_v06::ErrorType for UartWriter {
    type Error = embedded_io_v06::ErrorKind;
}

impl embedded_io_v06::Write for UartWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        embedded_io::Write::write(&mut self.0, buf)
            .map_err(|_| embedded_io_v06::ErrorKind::Other)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        embedded_io::Write::flush(&mut self.0)
            .map_err(|_| embedded_io_v06::ErrorKind::Other)
    }
}

// FOC Shell 命令定义
#[derive(Command)]
enum FocCommand<'a> {
    /// Show system status
    Status,
    /// Set motor speed (0-100)
    Speed {
        /// Speed value (0-100)
        value: u16,
    },
    /// Get motor position
    Position,
    /// Get motor velocity
    Velocity,
    /// Set a parameter value
    Set {
        /// Parameter name
        param: &'a str,
        /// Parameter value
        value: u32,
    },
    /// Get a parameter value
    Get {
        /// Parameter name
        param: &'a str,
    },
}

pub async fn run(usart: BufferedUart<'static>) {
    let (tx, mut rx) = usart.split();

    static CMD_BUF: StaticCell<[u8; 128]> = StaticCell::new();
    static HIST_BUF: StaticCell<[u8; 256]> = StaticCell::new();

    let writer = UartWriter(tx);

    let mut cli = CliBuilder::default()
        .writer(writer)
        .command_buffer(CMD_BUF.init([0u8; 128]).as_mut_slice())
        .history_buffer(HIST_BUF.init([0u8; 256]).as_mut_slice())
        .prompt("G431> ")
        .build()
        .unwrap();

    let mut byte = [0u8; 1];

    loop {
        match rx.read(&mut byte).await {
            Ok(0) => continue,
            Ok(_) => {
                let _ = cli.process_byte::<FocCommand, _>(
                    byte[0],
                    &mut FocCommand::processor(|cli, cmd| {
                        match cmd {
                            FocCommand::Status => {
                                cli.writer().write_str("System: STM32G431CBU6\r\n")?;
                                cli.writer().write_str("Clock:  170MHz (HSE 8MHz via PLL)\r\n")?;
                                cli.writer().write_str("UART:   USART2 PB3/PB4 @ 115200\r\n")?;
                                cli.writer().write_str("FOC:    Ready\r\n")?;
                            }
                            FocCommand::Speed { value } => {
                                if value <= 100 {
                                    cli.writer().write_str("Speed set to ")?;
                                    cli.writer().write_str(u16_to_str(value).as_str())?;
                                    cli.writer().write_str("\r\n")?;
                                } else {
                                    cli.writer().write_str("Error: speed must be 0-100\r\n")?;
                                }
                            }
                            FocCommand::Position => {
                                cli.writer().write_str("Position: 0 deg\r\n")?;
                            }
                            FocCommand::Velocity => {
                                cli.writer().write_str("Velocity: 0 rpm\r\n")?;
                            }
                            FocCommand::Set { param, value } => {
                                cli.writer().write_str("Set ")?;
                                cli.writer().write_str(param)?;
                                cli.writer().write_str(" = ")?;
                                cli.writer().write_str(u32_to_str(value).as_str())?;
                                cli.writer().write_str("\r\n")?;
                            }
                            FocCommand::Get { param } => {
                                cli.writer().write_str("Get ")?;
                                cli.writer().write_str(param)?;
                                cli.writer().write_str(": 0\r\n")?;
                            }
                        }
                        Ok(())
                    }),
                );
            }
            Err(_) => break,
        }
    }
}

// 避免使用 format! (需要 alloc)，手动实现简单的整数转字符串
fn u16_to_str(val: u16) -> heapless::String<8> {
    let mut s: heapless::String<8> = heapless::String::new();
    let mut n = val;
    if n == 0 {
        s.push('0').ok();
        return s;
    }
    let mut buf = [0u8; 8];
    let mut i = 8usize;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buf[i..] {
        s.push(b as char).ok();
    }
    s
}

fn u32_to_str(val: u32) -> heapless::String<12> {
    let mut s: heapless::String<12> = heapless::String::new();
    let mut n = val;
    if n == 0 {
        s.push('0').ok();
        return s;
    }
    let mut buf = [0u8; 12];
    let mut i = 12usize;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buf[i..] {
        s.push(b as char).ok();
    }
    s
}

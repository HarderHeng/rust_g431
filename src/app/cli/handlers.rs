use embedded_cli::cli::CliHandle;

use super::commands::FocCommand;
use super::writer::UartWriter;

// `embedded_io_v06::ErrorKind` 作为错误类型
type CliError = embedded_io_v06::ErrorKind;

/// 各命令的具体处理逻辑
pub fn handle(
    cli: &mut CliHandle<'_, UartWriter, CliError>,
    cmd: FocCommand<'_>,
) -> Result<(), CliError> {
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
                cli.writer()
                    .write_str(super::writer::u16_to_str(value).as_str())?;
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
            cli.writer()
                .write_str(super::writer::u32_to_str(value).as_str())?;
            cli.writer().write_str("\r\n")?;
        }
        FocCommand::Get { param } => {
            cli.writer().write_str("Get ")?;
            cli.writer().write_str(param)?;
            cli.writer().write_str(": 0\r\n")?;
        }
    }
    Ok(())
}

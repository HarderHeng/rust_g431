use embedded_cli::cli::Cli;
use embedded_cli::Command;

use super::handlers;
use super::writer::UartWriter;

type CliError = embedded_io_v06::ErrorKind;

/// FOC Shell 命令集
///
/// doc-comment 会自动生成 `help <cmd>` 说明
#[derive(Command)]
pub enum FocCommand<'a> {
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

/// 将一个字节送入 CLI，处理命令后路由到 handlers
pub fn process<CB: embedded_cli::buffer::Buffer, HB: embedded_cli::buffer::Buffer>(
    cli: &mut Cli<UartWriter, CliError, CB, HB>,
    byte: u8,
) {
    let _ = cli.process_byte::<FocCommand, _>(
        byte,
        &mut FocCommand::processor(|handle, cmd| handlers::handle(handle, cmd)),
    );
}

use embassy_stm32::usart::BufferedUartTx;

/// 将 embassy-stm32 的 `embedded_io 0.7` Write 适配为
/// `embedded-cli` 所需的 `embedded_io 0.6` Write
pub struct UartWriter(pub BufferedUartTx<'static>);

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

/// 将整数转换为字符串（no_std，无 alloc）
pub fn u16_to_str(val: u16) -> heapless::String<8> {
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

pub fn u32_to_str(val: u32) -> heapless::String<12> {
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

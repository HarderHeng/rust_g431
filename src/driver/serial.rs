use embassy_stm32::usart::BufferedUart;

/// 对 BufferedUart 的薄封装，向上层提供语义化接口
pub struct Serial {
    inner: BufferedUart<'static>,
}

impl Serial {
    pub fn new(uart: BufferedUart<'static>) -> Self {
        Self { inner: uart }
    }

    /// 拆分为独立的 TX / RX 端（消耗 self）
    pub fn split(
        self,
    ) -> (
        embassy_stm32::usart::BufferedUartTx<'static>,
        embassy_stm32::usart::BufferedUartRx<'static>,
    ) {
        self.inner.split()
    }
}

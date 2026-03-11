# STM32G431 FOC

基于 Rust 和 Embassy 框架的 STM32G431CB 微控制器 FOC（磁场定向控制）项目。

## 硬件配置

- **MCU**: STM32G431CB (Cortex-M4F, 128KB Flash, 32KB RAM)
- **外部晶振**: 8MHz HSE
- **系统时钟**: 160MHz (通过 PLL 配置)
- **调试接口**: SWD

## 内存布局

| 区域 | 起始地址 | 大小 |
|------|----------|------|
| FLASH | 0x08000000 | 128KB |
| RAM | 0x20000000 | 32KB |

## 外设配置

- **USART2**: PB3 (TX), PB4 (RX), 115200 波特率
- **定时器**: TIM2 (时间驱动)

## CLI 命令

通过串口终端可使用以下命令：

| 命令 | 说明 |
|------|------|
| `help` | 显示帮助信息 |
| `status` | 显示系统状态 |
| `clear` | 清屏 |
| `speed <0-100>` | 设置电机速度 |
| `position` | 获取电机位置 |
| `velocity` | 获取电机速度 |
| `history` | 显示命令历史 |
| `set <param> <value>` | 设置参数 |
| `get <param>` | 获取参数 |

## 依赖项

- `embassy-stm32` (0.5.0) - STM32 外设支持，异步驱动
- `embassy-executor` (0.6) - 异步任务执行器
- `embassy-time` (0.5) - 时间管理
- `defmt` / `defmt-rtt` / `panic-probe` - 高效日志输出和 panic 处理
- `embedded-io-async` - 异步 IO trait
- `heapless` - 无堆数据结构
- `cortex-m` / `cortex-m-rt` - Cortex-M 运行时支持
- `static_cell` - 静态内存分配

## 构建与烧录

### 构建

```bash
cargo build --release
```

### 生成 HEX 文件

```bash
cargo objcopy --release -- -O ihex main.hex
```

### 烧录 (使用 probe-run)

```bash
cargo run --release
```

## 项目结构

```
rust_g431/
├── Cargo.toml          # 项目配置和依赖
├── memory.x            # 链接器内存布局
├── main.hex            # 编译输出 HEX 文件
├── .cargo/
│   └── config.toml     # Cargo 构建配置
└── src/
    ├── main.rs         # 主程序入口
    └── cli.rs          # CLI 命令行接口
```

## 开发环境

- Rust nightly/stable (thumbv7em-none-eabihf target)
- probe-run (烧录和调试工具)
- defmt-print (日志查看)

## 许可证

MIT
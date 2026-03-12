# STM32G431 FOC

基于 Rust 和 Embassy 框架的 STM32G431CBU6 微控制器 FOC（磁场定向控制）项目。

## 硬件配置

- **MCU**: STM32G431CBU6 (Cortex-M4F, 128KB Flash, 32KB RAM)
- **外部晶振**: 8MHz HSE
- **系统时钟**: 170MHz（HSE 8MHz → PLL → 170MHz，Range1 Boost 模式）
- **调试接口**: SWD

## 内存布局

| 区域  | 起始地址   | 大小  |
|-------|------------|-------|
| FLASH | 0x08000000 | 128KB |
| RAM   | 0x20000000 | 32KB  |

## PLL 配置

```
HSE 8MHz  →  /2 (prediv)  →  4MHz
          →  ×85 (mul)    →  340MHz
          →  /2 (divr)    →  170MHz (SYSCLK)
```

- 时钟源：`PLL1_R`
- 必须启用 `boost = true`（Range1 Boost 模式，否则 170MHz 下闪存时序错误）

## 外设配置

| 外设   | 引脚         | 说明                    |
|--------|--------------|-------------------------|
| USART2 | PB3 TX / PB4 RX | AF7，115200 波特率，中断缓冲模式 |
| TIM2   | —            | Embassy 系统时间驱动      |

## CLI Shell

通过 USART2 串口（115200,8N1）连接终端，使用 `embedded-cli` 库实现交互式 Shell。

**提示符**: `G431> `

| 命令 | 说明 |
|------|------|
| `help` | 列出所有命令 |
| `help <cmd>` | 显示指定命令帮助 |
| `status` | 显示系统状态（时钟、UART 等） |
| `speed <0-100>` | 设置电机速度 |
| `position` | 查询电机位置 |
| `velocity` | 查询电机速度 |
| `set <param> <value>` | 设置参数 |
| `get <param>` | 查询参数 |

Shell 支持：方向键历史记录、Tab 自动补全、退格键行内编辑。

## 依赖项

| crate | 版本 | 用途 |
|-------|------|------|
| `embassy-stm32` | 0.5.0 | STM32 异步外设驱动 |
| `embassy-executor` | 0.6 | 异步任务执行器 |
| `embassy-time` | 0.5 | 时间管理 |
| `embedded-cli` | 0.2 | 交互式 CLI Shell |
| `embedded-io` | 0.7 | 同步 IO trait（embassy-stm32） |
| `embedded-io-v06` | 0.6 | 同步 IO trait（embedded-cli 适配） |
| `embedded-io-async` | 0.7 | 异步 IO trait |
| `heapless` | 0.8 | 无堆数据结构 |
| `static_cell` | 2.1 | 静态内存安全初始化 |
| `defmt` / `defmt-rtt` | 0.3/0.4 | 高效日志与 RTT 输出 |
| `panic-probe` | 0.3 | panic 处理 |
| `cortex-m` / `cortex-m-rt` | 0.7 | Cortex-M 运行时 |

> **注意**：`embassy-stm32 0.5.0` 依赖 `embedded-io 0.7`，而 `embedded-cli 0.2` 依赖
> `embedded-io 0.6`，两者不直接兼容。项目通过 `package` 别名同时引入两个版本，并在
> `cli.rs` 中使用 `UartWriter` newtype adapter 进行桥接。

## 构建与烧录

### 构建

```bash
cargo build --release
```

### 生成 HEX 文件

```bash
cargo objcopy --release -- -O ihex main.hex
```

### 烧录（probe-run）

```bash
cargo run --release
```

## 项目结构

```
rust_g431/
├── Cargo.toml          # 项目配置和依赖
├── memory.x            # 链接器内存布局
├── .cargo/
│   └── config.toml     # 构建目标与 runner 配置
└── src/
    ├── main.rs         # 主程序：RCC/PLL 初始化、USART2 配置
    └── cli.rs          # embedded-cli Shell 实现
```

## 开发环境

- Rust stable（`thumbv7em-none-eabihf` target）
- `probe-run` — 烧录与 RTT 日志查看

## 许可证

MIT

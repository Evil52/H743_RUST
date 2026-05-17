<img width="12288" height="16320" alt="743 (1)" src="https://github.com/user-attachments/assets/96e06c38-b271-4ac7-a001-33cfff09cdeb" />

*[Русская версия](README.md)*

# weact-h743 — RTC clock with button-driven LED on STM32H743

Firmware for the **WeAct Studio MiniSTM32H7xx** development board
(STM32H743VIT6), written in pure Rust with no RTOS. Displays the
current time and date on the onboard TFT and drives the onboard LED
through a state machine toggled by button K1.

---

## 1. Hardware platform

### 1.1. Why WeAct STM32H743VIT6 Mini

| Spec        | Value                                            |
|-------------|--------------------------------------------------|
| Core        | Cortex-M7 @ 480 MHz (we run it at 240 MHz)       |
| Flash       | 2 MB                                             |
| RAM         | 1 MB (DTCM 128K + AXI 512K + SRAM1..4)           |
| FPU         | Double precision (FPv5-D16)                      |
| Caches      | I-cache 16 KB, D-cache 16 KB                     |
| USB         | Type-C with native DFU support in ROM            |
| Display     | Onboard 0.96" 80×160 TFT (ST7735) on SPI4        |
| Buttons     | RESET, BOOT0, K1 (PC13)                          |
| LED         | LED1 on PE3 through an NPN transistor            |
| Price       | ~$15 per board                                   |

Rationale:

- **Performance without overkill** — Cortex-M7 is faster than any M0/M3/M4,
  while still cheaper than the official H7-Nucleo.
- **Built-in peripherals** — TFT, microSD, USB-C, W25Qxx flash are all
  already on the board; no shields needed.
- **DFU in ROM bootloader** — flashed over USB with `dfu-util`,
  no external programmer required (ST-Link/J-Link optional).
- **Well documented by the community** — schematic V12 is public, WeAct
  publishes a C SDK. We carried the pinout into the Rust firmware 1-to-1.

### 1.2. Pinout used in the firmware

| Signal     | Pin   | Note                                              |
|------------|-------|---------------------------------------------------|
| SPI4 SCK   | PE12  | Alternate function AF5                            |
| SPI4 MOSI  | PE14  | Alternate function AF5                            |
| LCD_CS     | PE11  | Held LOW (single device on the bus)               |
| LCD_DC     | PE13  | Data / Command                                    |
| LCD_BL     | PE10  | Backlight: **active-low** via SI2301 P-MOSFET     |
| LCD_RESET  | —     | Tied to system NRST on the board                  |
| LED1       | PE3   | **Active-high** via PDTC114E NPN                  |
| KEY K1     | PC13  | **Active-high**; internal pull-down               |
| VBAT       | PIN   | Optional CR2032 to keep RTC alive without USB     |

---

## 2. Why Rust instead of C

| Criterion                  | C (HAL/CubeMX)                  | Rust (`stm32h7xx-hal`)                          |
|----------------------------|---------------------------------|-------------------------------------------------|
| Memory safety              | Manual; easy to hit UB          | Guaranteed by the compiler                      |
| Concurrent access (ISR)    | Volatile + manual sync          | `Mutex<RefCell<…>>` + `Atomic*`, statically checked |
| Race conditions            | Possible and often silent       | Impossible in safe Rust (requires `unsafe`)     |
| Configuration errors       | Compile/runtime                 | Type-state: bad configs don't compile           |
| Firmware size              | ~30–50 KB for similar features  | ~40 KB (release, opt-level="s")                 |
| Project bootstrap          | CubeMX generates code           | `cargo new` + 5 lines in Cargo.toml             |
| Dependency management      | Submodules / manual copy        | `cargo` — single tool                           |
| API docs                   | Scattered across PDFs / headers | `cargo doc` locally, rust-analyzer in IDE       |
| Macros / code-gen          | C preprocessor                  | Declarative and procedural macros               |

**Concrete wins delivered by this firmware:**

1. **Type-state GPIO** — `gpioe.pe12.into_alternate::<5>()` returns
   `Pin<'E', 12, Alternate<5>>`. Using the wrong AF number or accessing
   a pin before it is configured is a compile error, not a silent
   peripheral failure.

2. **Safe shared state** — the LED and the timer live inside
   `Mutex<RefCell<Option<…>>>`. ISRs reach them only inside
   `cortex_m::interrupt::free`, which atomically disables interrupts
   for the critical section. In C this is a hand-rolled
   `__disable_irq()` pattern that's easy to forget.

3. **Lock-free shared atomics** — `AtomicU8` for `MODE` and `AtomicU32`
   for `TICKS` let the main loop read state without blocking ISRs.

4. **Explicit wrapping arithmetic** — `wrapping_add`, `wrapping_sub`
   are separate from regular operators; tick counter overflow can't
   produce undefined behaviour.

5. **No heap** — `#![no_std]` plus no `alloc`. Everything is allocated
   statically; OOM is impossible by construction.

6. **Cargo instead of Makefile/CMakeLists.txt** — one
   `cargo build --release` rebuilds everything including transitive
   deps (chrono, embedded-graphics, st7735-lcd).

---

## 2.1. About the single `unsafe` block in the firmware

The project contains **exactly one** `unsafe` block — enabling the NVIC
interrupt lines:

```rust
unsafe {
    pac::NVIC::unmask(interrupt::TIM2);
    pac::NVIC::unmask(interrupt::EXTI15_10);
}
```

**Why the API itself is `unsafe`:**
`cortex-m` marks `NVIC::unmask` as `unsafe` because enabling an interrupt
can break critical sections built on the "mask this specific interrupt"
pattern instead of `interrupt::free`. If some other piece of code is
relying on the fact that TIM2 won't fire — our `unmask` invalidates that
assumption.

**Why it is safe in this firmware:**

1. **The peripherals are already configured** by the time we call
   `unmask`. TIM2 is started and `listen` has subscribed it to
   `TimeOut`. PC13 is configured as a pull-down input and registered as
   an EXTI source on rising edges.

2. **The global resources are already published** to `G_LED`, `G_TIM2`,
   `G_BTN` inside the `interrupt::free` block on the line above. By the
   time the ISRs first run, those `Option`s are already `Some(...)`.

3. **We do NOT use mask-based critical sections** anywhere else. All
   critical sections go through `cortex_m::interrupt::free`, which
   disables interrupts globally for their duration — our `unmask` does
   not interfere with that.

**Why there is no alternative:**

In `cortex-m` 0.7 `NVIC::unmask` has no safe counterpart. The crate
authors deliberately require the caller to promise that enabling the
interrupt won't violate program invariants. The compiler cannot check
this statically — it has no way to know that we have initialised the
peripheral and published the shared state.

**What we get out of it:**

- One small, localised `unsafe` block with an explicit `// SAFETY:`
  comment in the source.
- Everything else (including the ISR handlers, shared-state access, and
  device drivers) is 100% safe Rust.
- In code review, only **one site** has to be checked for soundness of
  the unsafe invariant — instead of auditing the whole codebase, like
  you'd have to do in C.

That is the basic Rust principle in action: **`unsafe` is minimised and
encapsulated, not sprinkled across the program.**

---

## 3. Tech stack

### 3.0. Execution model: bare-metal + ISRs (NOT async / NOT Embassy)

This project **does not use async/await and does not use Embassy**.
The firmware follows the classic bare-metal pattern:

- **`#[entry] fn main() -> !`** — a single entry point with an infinite loop.
- **Hardware interrupts** (`#[interrupt] fn TIM2()`, `#[interrupt] fn EXTI15_10()`) —
  IRQ handlers from `cortex-m-rt`.
- **`cortex_m::asm::wfi()`** in the main loop — the CPU sleeps until
  the next interrupt fires.
- **Shared state** between main and ISRs — `AtomicU8/U32` plus
  `Mutex<RefCell<…>>` critical sections via `cortex_m::interrupt::free`.

Why not Embassy:

| Aspect                       | Bare-metal + ISRs (our choice)       | Embassy (async)                              |
|------------------------------|--------------------------------------|----------------------------------------------|
| Firmware size                | ~40 KB                               | ~60–100 KB (executor + futures)              |
| IRQ latency                  | Hardware, a few cycles               | Through executor poll — higher               |
| Complexity for a small task  | Minimal                              | Overkill — runtime, tasks, channels          |
| Composability as it grows    | Hand-rolled state machines           | Excellent (`select!`, `join!`)               |
| Learning curve               | Standard embedded Rust               | Async-specific + Embassy API                 |

**When Embassy would be worth it:**

- Several independent I/O tasks (USB + Ethernet + sensors).
- You want timeouts like `with_timeout(100.ms(), op).await`.
- You need task coordination through channels (`Channel`, `Signal`).

For our scenario (1 display + 1 button + 1 LED + RTC) bare-metal is
simpler, easier to read, and smaller. If the project grows to include
USB or Wi-Fi, migrating to Embassy will start to pay off.

### 3.1. Toolchain

| Tool                                    | Purpose                                  |
|-----------------------------------------|------------------------------------------|
| `rustc` 1.95                            | Rust compiler                            |
| `cargo` 1.95                            | Build system and dependency manager      |
| `rust-std` for `thumbv7em-none-eabihf`  | Standard library for Cortex-M7+FPU       |
| `arm-none-eabi-objcopy`                 | ELF → raw binary conversion for DFU      |
| `dfu-util` 0.11                         | Flashing the firmware over USB DFU       |

### 3.2. Crate dependencies

| Crate                   | Version | Role                                        |
|-------------------------|---------|---------------------------------------------|
| `cortex-m`              | 0.7     | Core peripherals access (NVIC, SysTick)     |
| `cortex-m-rt`           | 0.7     | Reset handler, vector table, `#[entry]`     |
| `panic-halt`            | 0.2     | Panic = infinite loop (no unwinding)        |
| `stm32h7xx-hal`         | 0.16    | HAL for STM32H7 (RCC, GPIO, SPI, RTC, TIM)  |
| `st7735-lcd`            | 0.9     | ST7735 driver (embedded-hal 0.2)            |
| `embedded-graphics`     | 0.8     | Fonts, primitives, text                     |
| `chrono`                | 0.4     | `NaiveDateTime`, date formatting (no_std)   |

### 3.3. Firmware architecture

```
┌───────────────────────────────────────────────────────────────────┐
│                            main loop                              │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  reads TICKS + MODE                                          │ │
│  │  if changed → draws time/date/mode on ST7735                 │ │
│  │  wfi() — sleeps until the next interrupt                     │ │
│  └──────────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────────┘
            ▲                                       ▲
            │ MODE/TICKS                            │ MODE
            │ (AtomicU8/U32)                        │ (AtomicU8)
            │                                       │
   ┌────────┴───────────┐                ┌──────────┴──────────┐
   │  TIM2 ISR (10 Hz)  │                │   EXTI15_10 ISR     │
   │ ─ TICKS += 1       │                │ ─ debounce via      │
   │ ─ drives LED       │                │   DEBOUNCE_UNTIL    │
   │   by current MODE  │                │ ─ MODE = MODE.next()│
   └────────────────────┘                └─────────────────────┘
```

**Interrupts (default, equal priority):**

- **TIM2** — periodic 100 ms tick. Increments `TICKS`, drives the LED pattern.
- **EXTI15_10** — rising edge on PC13 (K1 press). Advances `MODE`.

**Power management:** the main loop calls `cortex_m::asm::wfi()` —
the CPU halts until the next interrupt, saving power.

### 3.4. LED state machine

```
   ┌─────┐  press   ┌──────┐  press   ┌──────┐  press   ┌───────┐  press
   │ OFF │ ───────▶ │ FAST │ ───────▶ │ SLOW │ ───────▶ │ SOLID │ ──────▶ OFF
   └─────┘          └──────┘          └──────┘          └───────┘
                    100 ms /          500 ms /          always
                    100 ms            500 ms            on
```

### 3.5. RTC and persistence

- Clock source: **LSI** (internal RC, ~32 kHz) — no external crystal needed.
- The initial value lives in code (`NaiveDate::from_ymd_opt(...)`).
- **Backup register 0** is used as a magic flag `RTC_MAGIC = 0xCAFE_xxxx`.
  If the magic matches, `set_date_time` is skipped and the RTC keeps
  running from the previous boot. If the magic is missing (new firmware
  with a different value, or the backup domain lost power) the RTC is
  re-seeded.
- To survive a **full power cycle**, you need a CR2032 on the VBAT pad.
  Without it, the RTC resets every time USB is unplugged.

---

## 4. Build and flash

```bash
# Build the release binary
cargo build --release

# Convert ELF → raw binary
arm-none-eabi-objcopy -O binary \
  target/thumbv7em-none-eabihf/release/weact-h743 \
  firmware.bin

# Put the board in DFU: hold BOOT0, tap RESET, release BOOT0
# Flash
dfu-util -a 0 -s 0x08000000:leave -D firmware.bin
```

After `:leave` the board exits DFU and runs the new firmware automatically.

---

## 5. Repository layout

```
.
├── .cargo/config.toml   # target = thumbv7em-none-eabihf + link.x
├── Cargo.toml           # dependencies
├── Cargo.lock           # lock file (committed — this is a binary)
├── memory.x             # memory map (Flash 2M + AXI SRAM 512K)
├── src/main.rs          # the whole firmware, single file
├── README.md            # Russian version
└── README.en.md         # this file
```

---

## 6. References

- WeAct Studio Schematic V12 — https://github.com/WeActStudio/MiniSTM32H7xx
- STM32H743 Reference Manual (RM0433)
- `stm32h7xx-hal` docs — https://docs.rs/stm32h7xx-hal
- The Embedded Rust Book — https://docs.rust-embedded.org/book/

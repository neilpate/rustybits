# Example 01 - Hello World

A simple LED blinking example that blinks one LED on the micro:bit's LED matrix.

## What it does

This is the "Hello World" for embedded systems - it blinks an LED! The program:

1. Initializes the micro:bit board
2. Configures row 1 and column 1 of the LED matrix 
3. Continuously blinks the LED at the intersection (500ms on, 100ms off)

## Running this example

This example is completely self-contained - it includes all necessary configuration files:

### From Command Line
```bash
cd example_01_hello_world
cargo run
```

### From VS Code
1. Open `src/main.rs` in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function
3. Or use `Ctrl+Shift+P` → "Tasks: Run Task" → "Run Example 01"

> **💡 For detailed VS Code setup and visual guides, see [vscode_setup.md](../vscode_setup.md)**

### What Happens
- **Builds** the project for ARM Cortex-M4 (`thumbv7em-none-eabihf` target)
- **Flashes** the binary to your micro:bit via probe-rs  
- **Runs** the program on the micro:bit hardware
- **LED blinks** at row 1, column 1 position

## Project Structure

This example contains everything needed to build and run independently:

```
example_01_hello_world/
├── .cargo/
│   └── config.toml      # Build configuration (ARM target, probe-rs runner)
├── src/
│   └── main.rs          # Your Rust code
├── Cargo.toml           # Dependencies and project metadata
├── Cargo.lock           # Locked dependency versions (for reproducible builds)
└── Embed.toml           # probe-rs flashing configuration
```
## Code Overview

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(100);
        row1.set_high().unwrap();
        timer0.delay_ms(100);
        row1.set_low().unwrap();
    }
}
```

> **🦀 New to embedded Rust?** Check out the **[Embedded Rust Primer](../embedded_rust_primer.md)** to understand `#![no_std]`, `#[entry]`, and other embedded Rust essentials before diving into this example!

## How It Works

This example demonstrates the basics of micro:bit LED control:

1. **Board Initialization**: `microbit::Board::take().unwrap()` - Gets exclusive access to the micro:bit hardware
2. **LED Setup**: Configures row 1 and column 1 of the LED matrix - an LED lights when its row is HIGH and column is LOW
3. **Timer**: Uses the hardware timer for precise delays
4. **Blink Loop**: Toggles the LED every 100ms to create a blinking effect

### The micro:bit LED Matrix
The micro:bit's 5x5 LED display works as a matrix - each LED is controlled by setting its row HIGH and column LOW. This example controls just one LED at position (1,1).

### What the Crates Are Doing Behind the Scenes

While this code looks simple, here's the logical sequence of complex work the Rust embedded ecosystem handles for you:

#### 1. **Compilation Magic** (What Happens When You Build):

The crates do enormous work during compilation to transform your Rust code into efficient embedded machine code:

**Memory Layout Generation** (`cortex-m-rt`):
- Automatically generates `memory.x` linker script with nRF52833's exact flash/RAM layout
- Creates the ARM Cortex-M interrupt vector table with your reset handler
- Sets up proper memory sections (.text, .data, .bss, .rodata) for embedded systems

**Hardware Abstraction Compilation** (`microbit-v2`, `nrf52833-hal`):
- Your high-level `set_high()` calls compile to single ARM assembly instructions like `str r1, [r0]` (store register to memory)
- GPIO pin abstractions become direct register addresses (e.g., `0x50000000` for GPIO peripheral)
- Type safety checks happen at compile time - impossible to accidentally use wrong pin types

**Cross-Compilation Coordination**:
- Configures the Rust compiler for `thumbv7em-none-eabihf` (ARM Cortex-M4F with hardware float)
- Links against newlib-nano for minimal C library functions
- Strips out all standard library dependencies that assume an operating system
- Optimizes for code size (embedded systems have limited flash memory)

**Link-Time Optimization**:
- Dead code elimination removes unused functions (even from the HAL crates)
- Inlines small functions for maximum performance
- Resolves all hardware register addresses to compile-time constants
- Creates a single, self-contained binary with no external dependencies

#### 2. **Before Your `main()` Function Runs** (cortex-m-rt crate):
- Sets up the ARM Cortex-M4 processor after power on
- Initializes the stack pointer and memory layout
- Copies initial data from flash to RAM
- Zeros out uninitialized memory sections
- Calls your `main()` function with everything ready

#### 3. **Board Initialization** (`microbit::Board::take()`):
- **Clock System**: Configures the nRF52833's complex clock tree (high-frequency, low-frequency, and peripheral clocks)
- **Power Management**: Sets up voltage regulators and power domains
- **GPIO Configuration**: Maps the physical pins to their micro:bit functions (LED matrix, buttons, etc.)
- **Hardware Verification**: Ensures the chip is the expected nRF52833 variant
- **Resource Management**: Creates a singleton to prevent multiple access to hardware

#### 4. **Pin Configuration** (`into_push_pull_output()`):
- **Multiplexer Setup**: Configures the pin to be a GPIO (not UART, SPI, etc.)
- **Direction Setting**: Programs the GPIO direction register for output
- **Drive Strength**: Sets electrical characteristics (how much current the pin can supply)
- **Initial State**: Sets the pin to the specified starting voltage level
- **Safety Checks**: Ensures the pin isn't already in use elsewhere

#### 5. **Timer Initialization** (`let mut timer0 = timer::Timer::new(board.TIMER0)`):

**Code Breakdown:**
```rust
let mut timer0 = timer::Timer::new(board.TIMER0);
//  ↑    ↑        ↑                 ↑
//  │    │        │                 └─ Hardware peripheral (nRF52833 TIMER0)
//  │    │        └─ HAL constructor function
//  │    └─ Variable name (mutable because timer state changes)
//  └─ Let binding (creates owned timer instance)
```

**What `board.TIMER0` Actually Is:**
```rust
// board.TIMER0 is type: nrf52833_pac::TIMER0
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,  // Zero-size type marker
}

impl TIMER0 {
    // This gives access to hardware registers at memory address 0x40008000
    pub fn ptr() -> *const RegisterBlock {
        0x40008000 as *const _  // ← Actual nRF52833 TIMER0 base address
    }
}
```

**What `timer::Timer::new()` Does:**
```rust
// Inside nrf52833-hal crate:
impl Timer<TIMER0> {
    pub fn new(timer: TIMER0) -> Self {
        // Takes ownership of the TIMER0 peripheral - can't be used elsewhere!
        let timer_registers = unsafe { &*TIMER0::ptr() };
        
        // Hardware configuration at register level:
        timer_registers.mode.write(|w| w.mode().timer());     // Set to timer mode
        timer_registers.bitmode.write(|w| w.bitmode()._32bit()); // 32-bit mode
        timer_registers.prescaler.write(|w| unsafe { w.prescaler().bits(4) }); // 16MHz → 1MHz
        
        // Clear any previous state
        timer_registers.tasks_clear.write(|w| unsafe { w.bits(1) });
        timer_registers.tasks_stop.write(|w| unsafe { w.bits(1) });
        
        Timer {
            timer,           // Store the peripheral
            _phantom: PhantomData,
        }
    }
}
```

**The Hardware Registers Being Configured:**
```rust
// nRF52833 TIMER0 register map (memory addresses):
// 0x40008000: TASKS_START    - Start timer task
// 0x40008004: TASKS_STOP     - Stop timer task  
// 0x40008008: TASKS_COUNT    - Increment timer task
// 0x4000800C: TASKS_CLEAR    - Clear timer task
// 0x40008100: EVENTS_COMPARE - Timer compare event
// 0x40008504: MODE           - Timer/Counter mode register
// 0x40008508: BITMODE        - Timer bit width (16/24/32-bit)
// 0x40008510: PRESCALER      - Clock prescaler (divides 16MHz base clock)
// 0x40008540: CC[0]          - Compare/Capture register 0

// The initialization writes specific bit patterns to configure:
timer_registers.prescaler.write(|w| unsafe { w.prescaler().bits(4) });
// This sets prescaler to 4, giving us: 16MHz ÷ 2^4 = 1MHz timer clock
```

**Ownership and Type Safety:**
```rust
// After Timer::new(), the TIMER0 peripheral is "consumed":
let timer0 = timer::Timer::new(board.TIMER0);
// let another_timer = timer::Timer::new(board.TIMER0); // ← COMPILE ERROR!
// ERROR: use of moved value: `board.TIMER0`

// The HAL Timer wrapper provides safe methods:
timer0.delay_ms(100);  // ← This compiles to register operations
// vs unsafe direct access:
// (*TIMER0::ptr()).cc[0].write(|w| unsafe { w.cc().bits(100_000) }); // Dangerous!
```

**Why This Design Is Powerful:**
- **Zero-cost**: Timer wrapper has no runtime overhead
- **Type-safe**: Can't accidentally reuse the same timer hardware  
- **Hardware-optimal**: Direct register access with safety guarantees
- **Trait-based**: Implements `embedded-hal` DelayNs for portability

#### 6. **Runtime Operations** (Every `set_high()`, `delay_ms()` call):
- **GPIO Control**: Direct register writes to toggle pin voltage instantly
- **Timer Operations**: Hardware-based delays with microsecond precision
- **No OS Overhead**: Direct hardware access with zero operating system latency

---

## How The HAL Crates Make This Possible

**Without these crates**, you'd need to:
- Read the 400+ page nRF52833 reference manual
- Write hundreds of lines of register manipulation code  
- Debug timing issues and hardware conflicts with expensive equipment
- Handle ARM assembly startup code and linker scripts manually

**The Result**: Your simple `board.display_pins.row1.set_high()` compiles to just 2-3 ARM assembly instructions that directly flip hardware bits, but you get to write safe, high-level Rust code that's checked at compile time!

**Explanation of the HAL crates in use**

Embedded Rust strongly encourages the use of HAL (Hardware Abstraction Layer) crates, which provide a layered architecture for safe, portable hardware access:

#### **The HAL Ecosystem Layers:**

**1. embedded-hal (Universal Traits - Interfaces Only)**
```rust
use embedded_hal::{delay::DelayNs, digital::OutputPin};
```
- **Purpose**: Defines standard interfaces that work across different microcontrollers
- **What it provides**: **Only trait definitions** like `DelayNs`, `OutputPin`, `SpiDevice` - **no actual implementation**
- **Think of it as**: A contract that says "delay functions should look like this" but doesn't provide the actual delay code
- **Benefit**: Write code once, run on any chip that implements these traits

**2. microbit-v2 (Board Support Package + Chip HAL)**
```rust
use microbit::hal::{gpio, timer};        // ← nRF52833 HAL re-exported
let board = microbit::Board::take().unwrap();  // ← Board-specific configuration
```
- **Exposes**: Complete nRF52833 chip functionality mapped to micro:bit v2 board layout
- **How it works**: Internally uses `nrf52833-hal` for chip-specific implementations and adds micro:bit board configuration
- **What you get**: 
  - `microbit::hal` - All nRF52833 peripherals (GPIO, timers, SPI, etc.) implementing `embedded-hal` traits
  - `microbit::Board` - Pre-configured pin assignments, LED matrix mapping, button setup
- **Key insight**: You never import `nrf52833-hal` directly - everything comes through the microbit crate

**How the Re-export Works:**
```rust
// Inside microbit-v2 crate source code:
pub use nrf52833_hal as hal;  // ← Makes nRF HAL available as microbit::hal

// Your imports:
use microbit::hal::{gpio, timer};        // ← Gets nrf52833_hal functionality
let board = microbit::Board::take();     // ← Gets board-specific pin mappings

// You get both chip-level control AND board-level convenience in one crate!
```

**What `Board::take()` Actually Does:**
```rust
// Inside microbit-v2 source code:
pub fn take() -> Option<Self> {
    Some(Self::new(
        pac::Peripherals::take()?,      // ← Claims ALL nRF52833 peripherals (singleton pattern)
        pac::CorePeripherals::take()?,  // ← Claims ARM Cortex-M core peripherals
    ))
}
```

**Digging Into The `take()` Chain:**

**1. `pac::Peripherals::take()` - The nRF52833 Chip Level:**
```rust
// Inside nrf52833-pac crate (auto-generated from chip specification):
static mut DEVICE_PERIPHERALS: bool = false;  // ← THE SINGLETON GLOBAL VARIABLE

pub fn take() -> Option<Self> {
    cortex_m::interrupt::free(|_| {  // Critical section - interrupts disabled
        if unsafe { DEVICE_PERIPHERALS } {
            None  // Already taken!
        } else {
            unsafe { DEVICE_PERIPHERALS = true; }  // Mark as claimed
            Some(Peripherals {
                TIMER0: TIMER0 { _marker: PhantomData },  // Your hardware timer
                TIMER1: TIMER1 { _marker: PhantomData },  // 4 more timers available
                P0: P0 { _marker: PhantomData },          // GPIO Port 0 (32 pins)
                P1: P1 { _marker: PhantomData },          // GPIO Port 1 (16 pins)
                GPIOTE: GPIOTE { _marker: PhantomData },  // GPIO interrupt controller
                RADIO: RADIO { _marker: PhantomData },    // 2.4GHz radio (Bluetooth)
                TEMP: TEMP { _marker: PhantomData },      // Temperature sensor
                RNG: RNG { _marker: PhantomData },        // Random number generator
                ADC: SAADC { _marker: PhantomData },      // Analog-to-digital converter
                // ... 40+ more peripherals
            })
        }
    })
}
```

**The Global Variable Singleton Implementation:**

**`static mut DEVICE_PERIPHERALS: bool = false` Breakdown:**
- **`static`**: Stored in global memory, exists for the entire program lifetime
- **`mut`**: Mutable - can be changed from `false` to `true`  
- **`bool = false`**: Initially `false` (peripherals available), becomes `true` (peripherals taken)
- **Memory location**: Fixed address in RAM, typically around `0x20000000 + offset`
- **Size**: Just 1 byte - extremely efficient singleton tracking

**Why `static mut` Is Dangerous (But Controlled Here):**
```rust
// This would be unsafe in normal Rust:
static mut COUNTER: i32 = 0;
fn increment() {
    unsafe { COUNTER += 1; }  // Race condition possible!
}

// But PAC makes it safe with critical sections:
cortex_m::interrupt::free(|_| {  // Disables ALL interrupts
    unsafe { DEVICE_PERIPHERALS = true; }  // Atomic operation guaranteed
});
```

**Memory Layout of Singleton Variables:**
```rust
// These live somewhere in RAM (exact addresses determined by linker):
// Variable               | Size  | Purpose
// DEVICE_PERIPHERALS     | 1 byte| nRF52833 peripherals available?
// CORE_PERIPHERALS       | 1 byte| ARM core peripherals available?

// The linker places these in the .bss section of RAM
// (exact addresses depend on other static variables and linker script)

// After first Board::take():
// DEVICE_PERIPHERALS     = true   // nRF52833 peripherals CLAIMED
// CORE_PERIPHERALS       = true   // ARM core peripherals CLAIMED
```

**What This Hardware Claiming Prevents:**
```rust
// Without singleton pattern (dangerous):
let timer1 = unsafe { &*TIMER0::ptr() };  // Direct memory access
let timer2 = unsafe { &*TIMER0::ptr() };  // Same timer, different variable!
timer1.start();
timer2.stop();  // Conflicts with timer1! Hardware chaos!

// With singleton pattern (safe):
let peripherals1 = Peripherals::take().unwrap();  // Gets hardware
let peripherals2 = Peripherals::take();           // Returns None - prevented!
```

**The Singleton Pattern in Action:**
- **First call**: `Board::take()` returns `Some(Board)` with exclusive hardware access
- **Second call**: `Board::take()` returns `None` - hardware already claimed!
- **Why this matters**: Prevents multiple parts of code from interfering with the same hardware
- **Runtime cost**: Just a boolean check - zero performance overhead

---

## Understanding PhantomData: Zero-Cost Type Safety

You've probably noticed `PhantomData` throughout the embedded code. This is a crucial Rust concept that deserves explanation:

### What is PhantomData?

`PhantomData` is a zero-sized type marker that tells the Rust compiler about type relationships that don't exist at runtime but matter for type safety:

```rust
use std::marker::PhantomData;

// Without PhantomData - this won't compile:
pub struct Timer<T> {
    // ERROR: unused type parameter `T`
}

// With PhantomData - this compiles and uses zero memory:
pub struct Timer<T> {
    _marker: PhantomData<T>,  // ← Tells Rust we "use" the T type
}
```

### Why Embedded Rust Uses PhantomData Extensively

#### 1. Type Safety Without Memory Cost
```rust
let timer0 = Timer::<TIMER0>::new();   // Type: Timer<TIMER0>
let timer1 = Timer::<TIMER1>::new();   // Type: Timer<TIMER1>

// These are different types at compile time:
fn use_timer0(t: Timer<TIMER0>) { /* ... */ }
// use_timer0(timer1);  // ← COMPILE ERROR! Type mismatch!

// But at runtime, both are identical (zero-size):
assert_eq!(std::mem::size_of::<Timer<TIMER0>>(), 0);
assert_eq!(std::mem::size_of::<Timer<TIMER1>>(), 0);
```

#### 2. Hardware Resource Tracking
```rust
// Each peripheral type prevents mix-ups:
impl Timer<TIMER0> {
    fn ptr() -> *const RegisterBlock { 0x40008000 as *const _ }
}
impl Timer<TIMER1> {
    fn ptr() -> *const RegisterBlock { 0x40009000 as *const _ }
}

// PhantomData ensures you can't accidentally use wrong address
let timer0_wrapper = Timer::<TIMER0>::new();
// timer0_wrapper internally "knows" it's for TIMER0 hardware at 0x40008000
```

#### 3. Memory Layout Analysis
```rust
// The hardware peripheral structs:
struct TIMER0 { _marker: PhantomData<*const ()> }    // 0 bytes
struct TIMER1 { _marker: PhantomData<*const ()> }    // 0 bytes  
struct GPIO0  { _marker: PhantomData<*const ()> }    // 0 bytes

// A struct containing 40+ peripherals:
struct Peripherals {
    timer0: TIMER0,  // 0 bytes
    timer1: TIMER1,  // 0 bytes  
    gpio0: GPIO0,    // 0 bytes
    // ... 37 more peripherals, each 0 bytes
}
// Total size: 0 bytes! Pure type information.
```

#### 4. The Magic: Compile-Time Safety, Zero Runtime Cost
```rust
// What the programmer writes:
let mut timer = Timer::new(board.TIMER0);
timer.start();

// What the compiler generates (simplified assembly):
// mov r0, #0x40008000    ; Load TIMER0 base address  
// mov r1, #1             ; Load "start" value
// str r1, [r0, #0]       ; Write to TASKS_START register
// 
// No PhantomData overhead - it completely disappears!
```

### PhantomData In Action Throughout The Codebase
- **PAC peripherals**: `TIMER0 { _marker: PhantomData }` - tracks which hardware
- **HAL wrappers**: `Timer<T> { _marker: PhantomData<T> }` - prevents type confusion  
- **Pin types**: `Pin<P0_21> { _marker: PhantomData }` - prevents pin mix-ups
- **State tracking**: Different types for different hardware states (input vs output pins)

**The Result**: You get compile-time guarantees that you're using the right hardware, with absolutely zero runtime memory or performance cost!

---

**What Gets Pre-configured:**
```rust
// Inside Board::new() - configures 40+ pins and peripherals:
display_pins: DisplayPins {
    row1: p0_21.into_push_pull_output(Level::Low),  // ← Your LED row
    col1: p0_28.into_push_pull_output(Level::High), // ← Your LED column
    // ... all 25 LEDs pre-configured
},
buttons: Buttons {
    button_a: p0_14.into_floating_input(), // ← Left button ready
    button_b: p0_23.into_floating_input(), // ← Right button ready
},
TIMER0: p.TIMER0,  // ← Hardware timer ready for your use
// ... plus 30+ other peripherals
```

#### **Why This Layered Approach Works:**

**Portability**: Your `timer0.delay_ms(100)` works identically on ESP32, STM32, or nRF chips
**Safety**: Compile-time prevention of hardware conflicts (can't use same pin twice)
**Performance**: Zero-cost abstractions - high-level code compiles to optimal assembly
**Maintainability**: Hardware details abstracted away, focus on application logic

**The Trade-off**: More dependencies vs. raw register access, but you gain massive productivity and safety benefits while maintaining performance.

> **🔬 Want to See the Low-Level Details?** Check out [Example 02](../example_02_hello_world_minimal_dependencies/) which shows the same LED blinking functionality but using as few dependencies as possible and direct register manipulation. You'll see exactly what this high-level code is doing behind the scenes!



## Key Files

- **`src/main.rs`** - Your Rust code that blinks the LED
- **`.cargo/config.toml`** - Build configuration (ARM target, probe-rs runner)  
- **`Cargo.toml`** - Dependencies and project metadata
- **`Embed.toml`** - probe-rs flashing configuration

> **💡 Tip**: This example is completely self-contained. You can copy this entire directory and use it as a starting point for your own micro:bit projects!

## Additional Resources

- **[deep_dive.md](../deep_dive.md)** - Technical explanation of how Rust becomes running hardware code
- **[hardware.md](../hardware.md)** - How memory mapping physically works with address buses and internal memory
- **[vscode_setup.md](../vscode_setup.md)** - Complete VS Code configuration guide
# Embedded Rust Primer

A beginner-friendly guide to understanding the unique aspects of Rust for embedded systems development.

## Table of Contents

- [Why Embedded Rust is Different](#why-embedded-rust-is-different)
- [Essential Attributes and Macros](#essential-attributes-and-macros)
- [Memory Management in Embedded Systems](#memory-management-in-embedded-systems)
- [Hardware Abstraction Layers (HAL)](#hardware-abstraction-layers-hal)
- [Common Patterns](#common-patterns)
- [Troubleshooting Tips](#troubleshooting-tips)

---

## Why Embedded Rust is Different

Embedded Rust differs from regular Rust in several key ways:

- **No operating system** - Your code runs directly on bare metal
- **Limited resources** - Tight constraints on memory, processing power, and energy
- **Real-time requirements** - Predictable timing and response to hardware events
- **Direct hardware control** - Managing peripherals, interrupts, and memory-mapped registers
- **Different runtime model** - Programs run forever until power off or reset

These constraints require special language features and programming patterns.

---

## Essential Attributes and Macros

Every embedded Rust program uses several special attributes that look unusual to newcomers:

### `#![no_std]` - No Standard Library

```rust
#![no_std]  // ← Tells Rust: "Don't include the standard library"
```

**What this means:**
- **No `std::vec::Vec`**, `std::collections::HashMap`, `std::fs::File`, etc.
- **No heap allocation** - embedded systems often have very limited RAM
- **No operating system dependencies** - your code runs directly on bare metal
- **Much smaller binary size** - only includes what you actually use

**What you get instead:**
- **`core` library**: Basic types like `Option`, `Result`, iterators, math operations
- **Embedded-specific crates**: Hardware abstraction layers and embedded collections
- **Stack-only memory**: All variables live on the stack or in static memory

**Common `no_std` alternatives:**
```rust
// Instead of std::vec::Vec:
use heapless::Vec;  // Fixed-capacity vector

// Instead of std::collections::HashMap:
use heapless::FnvIndexMap;  // Fixed-capacity map

// Instead of std::string::String:
use heapless::String;  // Fixed-capacity string
```

### `#![no_main]` - No Standard Main Function

```rust
#![no_main]  // ← Tells Rust: "I'll provide my own program entry point"
```

**Why this is needed:**
- **Standard `main()`** assumes an operating system that can launch your program
- **Embedded systems** boot directly from reset vector - no OS to call `main()`
- **Different signature**: Embedded main never returns (runs forever), so it's `fn main() -> !`

### `#[entry]` - The Real Entry Point

```rust
use cortex_m_rt::entry;

#[entry]           // ← Macro that marks this as the program entry point
fn main() -> ! {   // ← Never returns (! = "never type")
    // Your code here
    loop {
        // Embedded programs typically run forever
    }
}
```

**What `#[entry]` does behind the scenes:**
1. **Creates reset handler**: Generates the ARM reset vector function
2. **Sets up stack**: Configures the processor stack pointer  
3. **Initializes memory**: Copies data from flash to RAM, zeros uninitialized memory
4. **Calls your function**: Jumps to your `main()` after hardware initialization

**The `-> !` return type:**
- **"Never type"**: Function never returns normally
- **Embedded reality**: Microcontrollers run forever until power off or reset
- **Compiler optimization**: Rust knows this function never exits, optimizes accordingly

### `use panic_halt as _;` - Panic Handler

```rust
use panic_halt as _;  // ← Links in a panic handler (what happens when code panics)
```

**Why this is required:**
- **No operating system** to handle crashes for you  
- **Must choose panic behavior**: halt, restart, print debug info, etc.
- **`panic_halt`**: Simply stops the processor - useful for development

**Other panic handler options:**
```rust
// Different panic behaviors:
use panic_halt as _;        // Stop processor (development)
use panic_reset as _;       // Restart microcontroller  
use panic_rtt_target as _;  // Print panic info over debug probe
use panic_abort as _;       // Just abort (minimal code size)
```

### Complete Template

```rust
#![no_std]        // ← Use minimal core library only
#![no_main]       // ← I'll provide my own entry point

use cortex_m_rt::entry;  // ← The #[entry] macro comes from here
use panic_halt as _;     // ← Choose panic behavior

#[entry]          // ← This becomes the reset vector handler
fn main() -> ! {  // ← Runs forever on bare metal
    // Your embedded code here
    
    loop {
        // Typical embedded pattern: infinite loop
    }
}
```

---

## Memory Management in Embedded Systems

### Stack vs Heap vs Static

**Stack Memory (Preferred):**
```rust
fn example() {
    let array = [0u8; 64];  // ← 64 bytes on stack, automatically freed
    // Stack is fast, predictable, and automatically managed
}
```

**Static Memory (Global, Lives Forever):**
```rust
static GLOBAL_COUNTER: AtomicU32 = AtomicU32::new(0);  // ← Lives for entire program
static mut BUFFER: [u8; 256] = [0; 256];              // ← Mutable global (requires unsafe)
```

**Heap Memory (Avoided in Embedded):**
```rust
// This WON'T work with #![no_std]:
// let vec = Vec::new();  // ← Requires heap allocation

// Use this instead:
use heapless::Vec;
let mut vec: Vec<u8, 32> = Vec::new();  // ← Fixed capacity, stack-allocated
```

### Why No Heap?

1. **Predictability**: Heap allocation can fail or fragment
2. **Real-time**: malloc/free have unpredictable timing
3. **Memory safety**: Easier to reason about memory usage
4. **Resource constraints**: Limited RAM (often 32KB-256KB)

---

## Hardware Abstraction Layers (HAL)

Embedded Rust uses a layered approach to hardware access:

### Layer 1: PAC (Peripheral Access Crate)
```rust
// Raw register access (auto-generated from chip specification):
use nrf52833_pac as pac;

let peripherals = pac::Peripherals::take().unwrap();
peripherals.GPIOTE.config[0].write(|w| unsafe { 
    w.mode().event()
     .psel().bits(21)
     .polarity().lo_to_hi()
});
```

### Layer 2: HAL (Hardware Abstraction Layer)  
```rust
// Safe, type-checked hardware access:
use nrf52833_hal as hal;

let gpio = hal::gpio::p0::Parts::new(peripherals.P0);
let pin = gpio.p0_21.into_push_pull_output(Level::Low);
```

### Layer 3: BSP (Board Support Package)
```rust  
// Board-specific pin mappings and configuration:
use microbit::Board;

let board = Board::take().unwrap();
let led = board.display_pins.row1.into_push_pull_output(Level::Low);
```

### Layer 4: embedded-hal Traits
```rust
// Generic traits that work across different chips:
use embedded_hal::digital::OutputPin;

fn blink_led<P: OutputPin>(mut led: P) {
    led.set_high().ok();
    // This function works with ANY microcontroller!
}
```

---

## Common Patterns

### Singleton Pattern (Hardware Resources)

```rust
// Hardware can only be accessed once:
let peripherals = pac::Peripherals::take().unwrap();  // ← First call succeeds
let more_peripherals = pac::Peripherals::take();      // ← Returns None!

// This prevents multiple parts of code from conflicting over the same hardware
```

### Critical Sections (Interrupt Safety)

```rust
use cortex_m::interrupt;

// Disable interrupts for atomic operations:
let result = interrupt::free(|_cs| {
    // Code here runs with interrupts disabled
    // Safe to access shared data
    GLOBAL_COUNTER.load(Ordering::Relaxed)
});
```

### Error Handling

```rust
// Embedded code often uses unwrap() for simplicity:
let pin = gpio.p0_21.into_push_pull_output(Level::Low);
pin.set_high().unwrap();  // ← Panic if this fails

// Or handle errors explicitly:
match pin.set_high() {
    Ok(()) => { /* success */ },
    Err(e) => { /* handle error */ },
}
```

### State Machines with Types

```rust
// Use Rust's type system to prevent invalid states:
struct Led<STATE> {
    pin: P0_21<Output<PushPull>>,
    _state: PhantomData<STATE>,
}

struct On;
struct Off;

impl Led<Off> {
    fn turn_on(self) -> Led<On> { /* ... */ }
}

impl Led<On> {  
    fn turn_off(self) -> Led<Off> { /* ... */ }
}

// Compiler prevents calling turn_off() on an already-off LED!
```

---



## Next Steps

Once you're comfortable with these concepts:

1. **Try the examples** in this repository
2. **Read chip reference manuals** to understand your hardware  
3. **Explore the embedded-hal ecosystem** for sensors and peripherals
4. **Join the community** - #rust-embedded on Matrix/Discord
5. **Build something cool!** 🦀

---

## Additional Resources

- **[The Embedded Rust Book](https://docs.rust-embedded.org/book/)** - Comprehensive guide
- **[embedded-hal Documentation](https://docs.rs/embedded-hal/)** - Standard traits
- **[awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)** - Curated list of crates
- **[Rust Embedded Working Group](https://github.com/rust-embedded/)** - Official organization
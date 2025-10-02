# Example 06 - Button-Controlled LED (Interrupt-Driven)

An interrupt-driven GPIO input demonstration that toggles an LED in response to button presses using hardware interrupts and atomic state management.

## What it does

This example demonstrates advanced embedded interrupt processing techniques. The program:

1. Initializes the micro:bit board and GPIO peripherals
2. Configures row 1 and column 1 of the LED matrix for output control
3. Configures button A as a pull-up input with GPIOTE interrupt capability
4. Implements hardware interrupt-driven input processing with atomic state sharing
5. Provides power-efficient operation using Wait-For-Interrupt (WFI) instruction
6. Toggles LED state on each button press with minimal interrupt handler complexity

## Running this example

This example is completely self-contained - it includes all necessary configuration files:

### From Command Line
```bash
cd example_06_buttons_interrupts
cargo run
```

### From VS Code
1. Open `src/main.rs` in VS Code
2. Click the ▶️ **Run** button above the `#[entry]` function

## Code Overview

```rust
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::hal::{
    gpio::Level,
    gpiote::Gpiote,
    pac::{self, interrupt},
};
use panic_halt as _;

// Simple atomic boolean for LED state shared between main loop and ISR
static LED_STATE: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    // Configure LED pins (top-left LED in 5x5 matrix)
    let row1 = board.display_pins.row1.into_push_pull_output(Level::High);
    let _col1 = board.display_pins.col1.into_push_pull_output(Level::Low);

    // Configure button A and GPIOTE for interrupt-driven input
    let button_a = board.buttons.button_a.into_pullup_input().degrade();
    let gpiote = Gpiote::new(board.GPIOTE);
    
    gpiote.channel0().input_pin(&button_a).hi_to_lo().enable_interrupt();
    
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE); }

    let mut led = row1;
    let mut last_state = false;

    loop {
        let current_state = LED_STATE.load(Ordering::Relaxed);
        
        if current_state != last_state {
            if current_state {
                led.set_low().ok(); // Turn LED on
            } else {
                led.set_high().ok(); // Turn LED off
            }
            last_state = current_state;
        }
        
        cortex_m::asm::wfi(); // Wait for interrupt (power-efficient)
    }
}

#[interrupt]
fn GPIOTE() {
    let gpiote = unsafe { &*pac::GPIOTE::ptr() };
    gpiote.events_in[0].write(|w| unsafe { w.bits(0) }); // Clear event
    
    // Toggle atomic state flag
    let current_state = LED_STATE.load(Ordering::Relaxed);
    LED_STATE.store(!current_state, Ordering::Relaxed);
}
```

## How It Works

This example demonstrates advanced interrupt-driven embedded programming techniques:

1. **Board Initialization**: `microbit::Board::take().unwrap()` - Gets exclusive access to the micro:bit hardware
2. **LED Configuration**: Configures row 1 and column 1 of the LED matrix - an LED lights when its row is LOW and column is LOW
3. **GPIOTE Setup**: Initializes GPIO Tasks and Events peripheral for hardware interrupt generation on button state changes
4. **Interrupt Configuration**: Maps button A to GPIOTE channel 0 with falling-edge detection (Hi-to-Lo transition on button press)
5. **Atomic State Management**: Uses `AtomicBool` for thread-safe communication between interrupt handler and main loop
6. **Power-Efficient Operation**: Main loop uses WFI (Wait-For-Interrupt) instruction to enter low-power mode between button events

### Advanced Embedded Programming Concepts

**Interrupt-Driven Architecture**: This implementation separates concerns between the interrupt service routine (ISR) and main application logic. The ISR performs minimal work (flag setting), while the main loop handles GPIO operations.

**GPIOTE Peripheral**: The nRF52833's GPIO Tasks and Events peripheral provides hardware-based interrupt generation, eliminating the need for software polling and reducing CPU overhead.

**Atomic Operations**: `AtomicBool` with `Ordering::Relaxed` provides lock-free, thread-safe communication between interrupt context and main thread without complex mutex patterns.

**Direct Register Access**: The interrupt handler uses PAC (Peripheral Access Crate) for direct hardware register manipulation, avoiding HAL object sharing complexity while maintaining memory safety.

**Power Efficiency**: WFI instruction allows the CPU to enter sleep mode between interrupts, significantly reducing power consumption compared to polling approaches.

### Performance Characteristics

- **Response Latency**: Sub-microsecond interrupt response time
- **Power Consumption**: Minimal - CPU sleeps between button events  
- **CPU Overhead**: Interrupt-driven processing eliminates continuous polling overhead
- **Scalability**: Architecture supports multiple interrupt sources with independent handling

## How the Interrupt Service Routine (ISR) is Called

Understanding the complete interrupt mechanism from hardware event to ISR execution:

### 1. Hardware Event Detection
```rust
// Button press creates falling edge on GPIO pin
gpiote.channel0().input_pin(&button_a).hi_to_lo().enable_interrupt();
```
When button A is pressed, the GPIO pin transitions from HIGH (3.3V) to LOW (0V). The GPIOTE peripheral detects this falling edge and sets the `events_in[0]` register bit.

### 2. NVIC Interrupt Request
```rust
unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE); }
```
The GPIOTE peripheral sends an interrupt request to the **NVIC** (Nested Vector Interrupt Controller). The NVIC manages interrupt priorities and determines when to interrupt the CPU.

### 3. CPU Context Switch
When the NVIC decides to service the interrupt:
1. **Current execution is suspended** (even if in WFI sleep mode)
2. **CPU registers are automatically saved** to the stack
3. **Program counter jumps** to the GPIOTE interrupt vector address
4. **Interrupt handler begins execution** in privileged mode

### 4. ISR Function Dispatch and Vector Table
```rust
#[interrupt]
fn GPIOTE() {
    // This function is automatically called by the interrupt vector
}
```
The `#[interrupt]` attribute tells the Rust compiler to:
- Place the function address in the **interrupt vector table**
- Generate proper **entry/exit sequences**
- Handle **register preservation** and **return-from-interrupt** instructions

#### The ARM Cortex-M Interrupt Vector Table

The **vector table** is a crucial data structure stored at the beginning of flash memory (address `0x0000_0000`) that contains function pointers for all possible interrupts:

```
Memory Address    Vector Entry              Description
0x0000_0000       Initial Stack Pointer     Top of RAM for stack
0x0000_0004       Reset Handler            Entry point after power-on/reset
0x0000_0008       NMI Handler              Non-Maskable Interrupt
0x0000_000C       Hard Fault Handler       Critical system fault
...               (System exceptions)       ...
0x0000_0040       IRQ[0] Handler           First peripheral interrupt
0x0000_0044       IRQ[1] Handler           Second peripheral interrupt
...               ...                      ...
0x0000_0080       GPIOTE Handler           GPIO Tasks & Events (IRQ[16])
...               (More peripherals)       Up to IRQ[47] on nRF52833
```

#### How `#[interrupt]` Works

1. **Compile-Time Registration**: The `#[interrupt]` macro registers your `GPIOTE` function with the linker
2. **Vector Table Population**: The linker places your function's address at offset `0x0000_0080` (GPIOTE's vector slot)
3. **Hardware Lookup**: When GPIOTE interrupt fires, the NVIC reads the function address from `0x0000_0080`
4. **Automatic Jump**: CPU jumps to your function address and begins execution

#### Vector Table Generation

The `cortex-m-rt` crate automatically generates the vector table at compile time:

#### Memory Layout Impact

The vector table affects the entire program's memory layout:
- **Flash Memory**: Vector table occupies first ~200 bytes of flash
- **Deterministic Addressing**: Each interrupt has a fixed, known memory location  
- **Boot Behavior**: CPU automatically loads stack pointer and reset vector on power-up
- **Real-Time Guarantees**: Hardware can jump to ISR without software lookup overhead

This hardware-software interface provides the critical link between GPIO events and your Rust code execution.

### 5. Hardware Event Clearing
```rust
gpiote.events_in[0].write(|w| unsafe { w.bits(0) }); // Clear event flag
```
The ISR **must clear the hardware event flag**, otherwise the interrupt will immediately fire again when the ISR returns.

### 6. Return to Main Execution
After the ISR completes:
1. **CPU registers are automatically restored** from the stack
2. **Program counter returns** to the interrupted instruction
3. **Main loop continues** from where it was interrupted
4. **WFI instruction allows** the CPU to sleep again until the next interrupt

### Key Technical Details

- **Interrupt Latency**: ~12-20 CPU cycles from GPIO edge to ISR entry on ARM Cortex-M4
- **Stack Usage**: Hardware automatically pushes 8 registers (32 bytes) onto the stack:
  - **R0-R3**: General-purpose registers (function arguments/return values)
  - **R12**: Intra-procedure call scratch register (also called "IP" - Intra-Procedure call)
    - Used by compiler for complex function calls and long branches
    - Temporary storage when calling functions through function pointers
    - May be corrupted by any function call, so must be preserved across interrupts
  - **LR**: Link Register (return address)
  - **PC**: Program Counter (current instruction address)
  - **xPSR**: Program Status Register (condition flags and execution state)
- **Atomic Operations**: The interrupt mechanism ensures atomic context switching
- **Priority Handling**: NVIC supports 16 interrupt priority levels with nested interrupts
- **Power Efficiency**: WFI instruction reduces power consumption to microamperes between events

This interrupt-driven architecture provides **deterministic**, **low-latency** response to hardware events while maintaining **power efficiency** and **CPU utilization** optimization.
# Hardware Deep Dive: Address Bus and Memory Systems

This document explains how the nRF52833 microcontroller's memory map physically works with the address bus, Flash memory, RAM, and peripheral registers.

## 🔌 How the Address Bus Physically Works

When your Rust code writes to `0x5000_0508`, here's what happens at the hardware level:

### **The ARM Cortex-M4 Address Bus**
The nRF52833's ARM Cortex-M4 processor has a **32-bit address bus** that can address 4GB of memory space (`2^32 = 4,294,967,296 bytes`). Each address line can be either HIGH (1) or LOW (0).

**Address `0x5000_0508` on the bus:**
```
Address: 0x5000_0508 = 0101 0000 0000 0000 0000 0101 0000 1000 (binary)

Physical Address Lines:
A31 A30 A29 A28 A27 A26 A25 A24 A23 A22 A21 A20 A19 A18 A17 A16
 0   1   0   1   0   0   0   0   0   0   0   0   0   0   0   0

A15 A14 A13 A12 A11 A10 A9  A8  A7  A6  A5  A4  A3  A2  A1  A0  
 0   0   0   0   0   1   0   1   0   0   0   0   1   0   0   0
```

### **Memory Map Decoding Hardware**
The nRF52833 has **address decoding logic** that routes addresses to different physical memory/peripheral blocks:

```
Memory Map (Physical Hardware Blocks):
┌─────────────────────┬─────────────────────┬─────────────────────┐
│ Address Range       │ Physical Hardware   │ What Lives There    │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ 0x0000_0000         │ Internal Flash      │ Your program code   │
│ to 0x0007_FFFF      │ Memory Controller   │ Vector table        │
│ (512KB)             │                     │ Constants           │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ 0x2000_0000         │ Internal SRAM       │ Variables           │
│ to 0x2001_FFFF      │ Memory Controller   │ Stack               │
│ (128KB)             │                     │ Heap                │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ 0x5000_0000         │ GPIO Peripheral     │ Pin control         │
│ to 0x5000_0FFF      │ Registers           │ Input/output        │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ 0x4000_0000         │ Timer Peripherals   │ Hardware timers     │
│ to 0x4FFF_FFFF      │ UART, SPI, I2C etc │ Communication       │
└─────────────────────┴─────────────────────┴─────────────────────┘
```

### **What Happens During a Memory Write**

When you execute `core::ptr::write_volatile(0x5000_0508 as *mut u32, value)`:

1. **🖥️ CPU Issues Transaction**: Cortex-M4 puts `0x5000_0508` on address bus
2. **🔍 Address Decoder**: Hardware logic examines address bits:
   - Bits [31:28] = `0x5` → "This is for GPIO peripheral block"
   - Bits [11:0] = `0x508` → "This is the OUTSET register within GPIO"
3. **📡 Chip Select**: Address decoder activates GPIO peripheral's chip select line
4. **💾 Register Write**: GPIO peripheral receives the data and updates the OUTSET register
5. **⚡ Pin Change**: GPIO hardware immediately changes the physical pin voltage

### **nRF52833 Internal Architecture & Address Bus**

```
┌─────────────────────────────────────────────────────────────────┐
│                        nRF52833 SoC                            │
│                                                                 │
│  ┌─────────────────┐    32-bit Address Bus   ┌─────────────────┐│
│  │   ARM Cortex-M4 │◄──────────────────────►│  Address        ││
│  │      Core       │    32-bit Data Bus     │  Decoder        ││
│  │                 │◄──────────────────────►│  Logic          ││
│  │ - 64MHz CPU     │                        │                 ││
│  │ - 32-bit arch   │                        │                 ││
│  └─────────────────┘                        └─────────────────┘│
│                               │                                 │
│                               ▼                                 │
│        ┌──────────────────────┼──────────────────────┐         │
│        │                      │                      │         │
│        ▼                      ▼                      ▼         │
│  ┌─────────────┐        ┌─────────────┐        ┌─────────────┐ │
│  │   Internal  │        │  Internal   │        │    GPIO     │ │
│  │    Flash    │        │    SRAM     │        │ Peripheral  │ │
│  │   512KB     │        │   128KB     │        │ Registers   │ │
│  │             │        │             │        │             │ │
│  │0x0000_0000  │        │0x2000_0000  │        │0x5000_0000  │ │
│  │    to       │        │    to       │        │    to       │ │
│  │0x0007_FFFF  │        │0x2001_FFFF  │        │0x5000_0FFF  │ │
│  └─────────────┘        └─────────────┘        └─────────────┘ │
│        │                        │                      │       │
│        ▼                        ▼                      ▼       │
│  ┌─────────────┐        ┌─────────────┐        ┌─────────────┐ │
│  │  Floating   │        │6-Transistor │        │   Pin       │ │
│  │   Gate      │        │ SRAM Cells  │        │ Drivers     │ │
│  │Transistors  │        │(Flip-flops) │        │             │ │
│  │(Non-vol.)   │        │(Volatile)   │        │             │ │
│  └─────────────┘        └─────────────┘        └─────────────┘ │
│                                                          │     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │Timer/UART/SPI   │  │   Bluetooth     │  │Other Peripherals│ │
│  │  Peripherals    │  │     Radio       │  │   (ADC, etc.)   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼ (GPIO pins only)
                        ┌─────────────────┐
                        │   micro:bit v2  │
                        │      PCB        │
                        │  - LED matrix   │
                        │  - Buttons      │
                        │  - Sensors      │
                        │  - Edge pins    │
                        └─────────────────┘
```

This diagram shows both the **logical address bus architecture** and the **physical nRF52833 SoC structure**:
- **Top level**: Everything inside the nRF52833 chip
- **Address bus**: How the CPU communicates with different blocks
- **Memory blocks**: Internal Flash and SRAM with their address ranges
- **Physical storage**: The actual transistor types used for storage
- **External interface**: Only GPIO pins connect to the micro:bit PCB

### **Why This Design is Powerful**

From your Rust code's perspective, **everything looks like memory**:
- `*0x0000_1000` → Reads from flash memory
- `*0x2000_1000` → Reads from RAM 
- `*0x5000_0508` → Reads from GPIO register

But physically, these go to completely different hardware blocks! This **unified memory architecture** makes embedded programming much simpler - you don't need special I/O instructions like on x86 processors.

### **The LED Blink Journey**

When you write `1 << 21` to `GPIO_P0_OUTCLR` at `0x5000_050C`:

1. **Address `0x5000_050C`** travels down the address bus
2. **Address decoder** routes it to GPIO peripheral 
3. **GPIO register** at offset `0x50C` (OUTCLR) receives the data
4. **Pin 21 driver** sees bit 21 is set and pulls P0.21 LOW
5. **Physical pin P0.21** changes voltage from 3.3V to 0V
6. **LED matrix row 1** becomes active (current can flow)
7. **Photons** emerge from the LED! 💡

This is the complete journey from Rust code to photons - all orchestrated by the address bus routing your memory write to the right physical hardware!

## 📚 Reading/Writing RAM (0x2000_0000 - 0x2001_FFFF)

RAM access works differently than peripheral registers - it's actual memory storage:

**Writing to RAM (`let mut counter: u32 = 42;`):**
```rust
// Rust compiler might place this variable at 0x2000_1000
let mut counter: u32 = 42;
```

**Physical Process:**
1. **🖥️ Address Generation**: CPU puts `0x2000_1000` on address bus
2. **🔍 Address Decoder**: Recognizes bits [31:17] = `0x1000_0` → "This is SRAM"  
3. **📡 SRAM Controller**: Gets activated with remaining address bits [16:0] = `0x1000`
4. **💾 Memory Array**: SRAM controller accesses the physical memory cell at row/column determined by address
5. **⚡ Storage**: Value `42` gets stored in the SRAM cell (6-transistor flip-flop circuit)

**SRAM Physical Structure:**
```
SRAM Memory Array (128KB):
┌─────────────────────────────────────────────────────────────┐
│  Row Address [16:7] selects one of 512 rows                │
│  ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐   │
│  │Cell │Cell │Cell │Cell │Cell │Cell │Cell │Cell │ ... │   │
│  │0,0  │0,1  │0,2  │0,3  │0,4  │0,5  │0,6  │0,7  │     │   │
│  ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤   │
│  │Cell │Cell │Cell │Cell │Cell │Cell │Cell │Cell │ ... │   │
│  │1,0  │1,1  │1,2  │1,3  │1,4  │1,5  │1,6  │1,7  │     │   │
│  └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘   │
│  Column Address [6:2] selects which 32-bit word            │
└─────────────────────────────────────────────────────────────┘
```

**Why RAM is Fast:**
- **⚡ Direct Access**: No protocol - just put address on bus, data appears
- **🔄 Volatile Storage**: Data stored in flip-flops (powered circuits)
- **⏱️ Single Cycle**: Read/write typically completes in 1 CPU cycle
- **🧠 Cache Friendly**: ARM can prefetch sequential addresses

**Reading from RAM:**
```rust
let value = counter; // Load from 0x2000_1000
```
1. Address `0x2000_1000` on bus
2. SRAM controller decodes row/column
3. Memory cell drives data onto data bus
4. CPU reads the value (42) in ~1 clock cycle

## 💾 Reading/Writing Flash (0x0000_0000 - 0x0007_FFFF)

Flash memory is fundamentally different - it's **non-volatile storage** using floating gate transistors:

**Reading from Flash (Normal Operation):**
```rust
// Your program code and constants live here
const GREETING: &str = "Hello, micro:bit!";  // Stored in flash
```

**Physical Process:**
1. **🖥️ Address Request**: CPU puts flash address on bus (e.g., `0x0000_1000`)
2. **🔍 Flash Controller**: Decodes address and activates flash memory array
3. **⚡ Cell Read**: Floating gate transistor's charge level determines if it's 0 or 1
4. **📤 Data Return**: Flash controller returns data on data bus
5. **🧠 CPU Execution**: CPU can execute code directly from flash (XIP - eXecute In Place)

**Flash Memory Physical Structure:**
```
Flash Memory Array (512KB):
┌─────────────────────────────────────────────────────────────┐
│ Each cell is a floating gate transistor                     │
│                                                             │
│  Control Gate (Word Line)                                   │
│       │                                                     │
│  ┌────▼────┐     ┌─────────┐     ┌─────────┐              │
│  │ Oxide   │ ──► │Floating │ ──► │ Oxide   │              │
│  │ Layer   │     │  Gate   │     │ Layer   │              │
│  └─────────┘     │(Stores  │     └─────────┘              │
│                  │ charge) │                               │
│  Source ◄────────┤  ▼▼▼▼   ├────────► Drain               │
│                  │ Channel │         (Bit Line)            │
│                  └─────────┘                               │
│                                                             │
│ Charged floating gate = 0                                  │
│ Uncharged floating gate = 1                                │
└─────────────────────────────────────────────────────────────┘
```

**Why Flash is Different:**
- **🔋 Non-volatile**: Data survives power loss (charge trapped in floating gates)
- **📖 Read-Only in Normal Operation**: Can't write during program execution
- **⏱️ Slower than RAM**: ~3-4 CPU cycles per read (need charge sensing)  
- **🗂️ Block Organized**: Erased in large blocks (4KB pages)

**Writing to Flash (Programming/Flashing):**
Flash writing requires special high-voltage operations and can only be done by external programmers or bootloaders:

```rust
// This happens during `cargo run` - probe-rs writes to flash
// Your compiled program gets stored here
#[no_mangle]
pub extern "C" fn Reset() -> ! {
    // This code physically lives in flash memory
    main();
}
```

**Programming Process (What `probe-rs` does):**
1. **🔓 Unlock Flash**: Send special command sequence to flash controller
2. **⚡ High Voltage**: Apply ~12V to control gate to force electrons onto floating gate
3. **🗂️ Page Programming**: Write entire 4KB pages at once
4. **✅ Verify**: Read back and confirm data was stored correctly
5. **🔒 Lock Flash**: Protect against accidental writes

## 🏁 Memory Access Speed Comparison

```
Memory Type    | Access Time | Volatility | Use Case
─────────────────────────────────────────────────────────
CPU Registers  | 0 cycles    | Volatile   | Active computation
Cache (L1)     | 1 cycle     | Volatile   | Recently used data  
SRAM          | 1-2 cycles  | Volatile   | Variables, stack
Flash         | 3-4 cycles  | Non-vol.   | Program code, constants
External Flash| 10+ cycles  | Non-vol.   | Large data storage
```

## 🔄 The Complete Memory Ecosystem

When your program runs:

1. **🚀 Boot**: CPU reads reset vector from flash address `0x0000_0000`
2. **📋 Code Execution**: Instructions fetched from flash, executed by CPU
3. **📊 Variable Access**: Data reads/writes go to SRAM for speed
4. **⚡ Register Operations**: GPIO writes go to peripheral address space
5. **🔄 Constant Access**: String literals and `const` values read from flash

**Example Memory Journey:**
```rust
const MESSAGE: &str = "LED ON!";     // Flash: 0x0000_2000
let mut counter: u32 = 0;            // SRAM:  0x2000_1004
loop {
    counter += 1;                    // SRAM read+write
    if counter % 1000 == 0 {
        unsafe {
            // Peripheral register write
            core::ptr::write_volatile(0x5000_0508, 1 << 21);
        }
        println!("{}", MESSAGE);     // Flash read for string
    }
}
```

Each memory access uses the same address bus but hits completely different physical hardware optimized for different purposes! 🎯

## 🔍 Internal vs External Memory on the micro:bit v2

**Important Clarification**: The BBC micro:bit v2 uses **internal memory only** - both Flash and RAM are built into the nRF52833 chip itself.

### **Key Architecture Points:**

- **Everything is internal**: Flash, SRAM, and peripherals are all built into the nRF52833 chip
- **Unified addressing**: Same address bus reaches Flash (0x0000_0000), SRAM (0x2000_0000), and peripherals (0x5000_0000)
- **Different storage types**: Flash uses floating gate transistors, SRAM uses flip-flop circuits
- **External connections**: Only GPIO pins connect to micro:bit PCB components

### **Why Internal Memory?**

**Benefits of Internal Memory:**
- **🚀 Speed**: Direct connection to CPU via internal buses (no external protocol overhead)
- **⚡ Power**: No external memory controller needed, lower power consumption
- **📦 Size**: Reduces PCB complexity and physical size
- **💰 Cost**: Cheaper than adding external memory chips
- **🔒 Reliability**: No external connections that can fail

**Comparison with External Memory:**
```
Internal Memory (nRF52833):     External Memory (hypothetical):
┌─────────────────────────┐     ┌─────────────────────────────┐
│ CPU ←→ Internal Bus     │     │ CPU ←→ SPI/I2C ←→ Ext Chip │
│     ←→ Flash (512KB)    │     │     ←→ Controller ←→ Flash  │
│     ←→ SRAM (128KB)     │     │                  ←→ SRAM   │
│                         │     │                            │
│ Access: 1-4 cycles      │     │ Access: 10-100+ cycles     │
│ Power: Low              │     │ Power: Higher              │
│ Pins: 0 external        │     │ Pins: 4-8+ external       │
└─────────────────────────┘     └─────────────────────────────┘
```

### **Other micro:bit Components**

The micro:bit v2 PCB does have other components, but they're sensors and peripherals, not memory:

**On the micro:bit PCB (external to nRF52833):**
- **🔈 Speaker** - Audio output
- **🎤 Microphone** - Audio input with LED indicator  
- **🧭 Magnetometer/Accelerometer** - LSM303AGR sensor (I2C)
- **💡 25x LED Matrix** - Directly driven by nRF52833 GPIO
- **🔘 2x Buttons** - Connected to nRF52833 GPIO
- **📡 Radio Antenna** - For Bluetooth/802.15.4
- **🔌 Edge Connector** - 25 pins for external connections
- **🔋 Battery Connector** - Power input
- **🔗 USB Connector** - Programming and power

**None of these are memory** - they're all input/output devices or sensors that the nRF52833 communicates with via GPIO, I2C, or SPI.

### **Memory Layout Reality**

So when we talk about the memory map:
```
0x0000_0000 - 0x0007_FFFF: Internal Flash (in nRF52833 silicon)
0x2000_0000 - 0x2001_FFFF: Internal SRAM  (in nRF52833 silicon)
0x5000_0000 - 0x5000_0FFF: GPIO Registers (in nRF52833 silicon)
```

All of these addresses refer to **circuits inside the nRF52833 chip itself**. There's no external memory bus going off-chip to separate Flash or RAM components.

This is typical for modern microcontrollers - they integrate everything needed for basic operation into a single chip to minimize size, cost, and complexity! 🎯

## Related Documents

- **[Example 02](example_02_hello_world_minimal_dependencies/)** - Direct register manipulation with minimal dependencies
- **[Example 03](example_03_hello_world_no_dependencies/)** - Zero dependencies with custom linker scripts
- **[deep_dive.md](deep_dive.md)** - Complete compilation pipeline from Rust to hardware
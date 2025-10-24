# Debugging BBC micro:bit v2 with Rust

This document provides comprehensive information about debugging Rust applications on the BBC micro:bit v2, covering debug protocols, hardware interfaces, and debugging tools.

> **Note**: For the main compilation pipeline and project setup, see [deep_dive.md](deep_dive.md).  
> **Hardware Focus**: For detailed hardware architecture information, see [hardware.md](hardware.md).

## Debug Interface Overview

The BBC micro:bit v2 uses a sophisticated debug architecture that enables powerful debugging capabilities without requiring external hardware debuggers.

### micro:bit v2 Debug Architecture

```
Development PC ←→ USB ←→ nRF52820 (Interface) ←→ SWD ←→ nRF52833 (Target)
                           │                              │
                    [Debug Firmware]              [Your Rust Application]
```

**Key Components:**
- **nRF52833 (Target MCU)**: Runs your Rust application
- **nRF52820 (Interface MCU)**: Dedicated debug interface controller
- **SWD (Serial Wire Debug)**: ARM's efficient debugging protocol
- **USB Interface**: Connects to your development machine

## SWD (Serial Wire Debug) Protocol

SWD is ARM's proprietary debugging protocol, designed as a more efficient alternative to JTAG for ARM Cortex-M processors.

### Physical Interface

The SWD interface uses a minimal pin configuration:

- **SWCLK**: Serial Wire Clock - provides timing for data transfers
- **SWDIO**: Serial Wire Data I/O - bidirectional data line
- **Ground**: Common reference voltage
- **VCC**: Power reference (3.3V on micro:bit v2)

### Protocol Architecture

```
Development Tool (probe-rs/GDB)
           ↕
    Debug Port (DP) - Controls the debug interface
           ↕  
   Access Port (AP) - Provides memory access
           ↕
  Target Memory Bus - CPU's internal buses
           ↕
    Target Processor (nRF52833)
```

### SWD Communication Process

The Serial Wire Debug protocol operates through a sophisticated packet-based communication system:

#### 1. Clock and Data Synchronization
- **SWCLK** provides precise timing for all data transfers
- **SWDIO** carries bidirectional data using clock-synchronized packets
- **Maximum frequency**: Up to 50MHz (much faster than JTAG's typical 10-20MHz)

#### 2. Packet Structure
Each SWD transaction consists of structured packet exchanges:

```
Request Packet (8 bits):
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│Start│ AP  │R/W  │ A2  │ A3  │Parity│Stop │Park │
│  1  │ 0/1 │ 0/1 │ 0/1 │ 0/1 │ 0/1 │  0  │  1  │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘

Response Packet:
┌─────┬─────┬─────┬──────────┬─────┬─────┐
│ ACK │ ACK │ ACK │   Data   │Parity│Trnr │
│  0  │  1  │  0  │ 32 bits  │  1  │  1  │
└─────┴─────┴─────┴──────────┴─────┴─────┘
```

#### 3. Turnaround Cycles
- **Purpose**: Prevents bus conflicts when switching between read and write operations
- **Implementation**: Brief periods where neither host nor target drives SWDIO
- **Timing**: Typically 1-2 clock cycles depending on implementation

#### 4. Error Detection and Recovery
- **Parity Checking**: Each packet includes parity bits for data integrity
- **ACK/NAK Responses**: Target confirms successful packet reception
- **Protocol Recovery**: Automatic retry mechanisms for failed transactions
- **Line Reset**: Special sequence to recover from protocol errors

### Advanced SWD Features

#### Multi-Drop Support
SWD supports multiple targets on a single interface:
- **Target Selection**: Each device has a unique ID
- **Dormant State**: Inactive devices enter low-power dormant mode
- **Wake Sequences**: Special patterns to activate specific targets

#### Security Extensions
Modern ARM processors (including nRF52833) implement security features:
- **Debug Authentication**: Cryptographic challenge-response authentication
- **Secure Debug Enable**: Different privilege levels for debug access
- **Access Port Protection**: Fine-grained control over memory regions accessible via debug
- **Debug Disable Fuses**: Permanent disabling of debug interface for production devices

## Debug Features and Capabilities

### Core Debug Operations

#### Flash Programming
- **Direct Memory Access**: SWD provides direct write access to flash memory controllers
- **Erase Operations**: Sector and mass erase capabilities
- **Verification**: Read-back verification of programmed data
- **Protection Override**: Ability to bypass certain flash protection mechanisms

#### Memory Inspection
- **Real-time Access**: Read any RAM or peripheral register while processor is running
- **Memory Maps**: Access to complete memory space including peripherals
- **Register Dumps**: Complete CPU register state capture
- **Stack Analysis**: Call stack unwinding and analysis

#### CPU Control
- **Halt/Resume**: Stop and restart processor execution
- **Single-Step**: Execute one instruction at a time
- **Reset Control**: Various reset types (system, core, debug)
- **Clock Control**: Manage processor clocking during debug sessions

#### Breakpoint System
The nRF52833 provides comprehensive hardware breakpoint support:

**Hardware Breakpoints (6 available):**
- **Instruction Breakpoints**: Stop execution at specific addresses
- **Data Breakpoints (Watchpoints)**: Monitor memory locations for read/write access
- **Conditional Breakpoints**: Break only when specific conditions are met
- **Breakpoint Chaining**: Combine multiple breakpoints for complex trigger conditions

**Software Breakpoints:**
- **Unlimited Quantity**: Limited only by available memory
- **Implementation**: Replace instruction with breakpoint instruction (BKPT)
- **Automatic Management**: Debugger handles instruction replacement and restoration

#### Watchpoint Capabilities
- **Address Matching**: Monitor specific memory addresses
- **Value Comparison**: Break when memory contains specific values
- **Access Type Filtering**: Separate triggers for read, write, or both
- **Size Matching**: Monitor byte, halfword, or word accesses

### Advanced Debugging Features

#### Real-time Trace (ETM)
While not available on nRF52833, higher-end Cortex-M processors support:
- **Instruction Trace**: Complete execution history
- **Data Trace**: Memory access logging
- **Profiling Support**: Performance analysis capabilities

#### ITM (Instrumentation Trace Macrocell)
The nRF52833 supports ITM for non-intrusive debugging:
- **printf Debugging**: Output debug messages without stopping execution
- **Timestamp Information**: Precise timing of debug events
- **Multiple Channels**: Separate streams for different debug information

## SWD vs JTAG Comparison

### Technical Advantages of SWD

| Feature | SWD | JTAG |
|---------|-----|------|
| **Pin Count** | 2 (SWCLK, SWDIO) | 4 (TCK, TMS, TDI, TDO) |
| **Maximum Speed** | Up to 50MHz | Typically 10-20MHz |
| **Protocol Complexity** | Optimized for ARM | Universal but complex |
| **Multi-target** | Native support | Requires TAP controller |
| **Power Efficiency** | Lower due to fewer pins | Higher due to more signals |
| **Noise Immunity** | Better (differential-like) | More susceptible |

### Performance Characteristics

#### Speed Comparison
- **SWD**: Theoretical maximum of 50MHz with typical implementations running at 10-20MHz
- **JTAG**: Usually limited to 10-20MHz due to protocol overhead and signal integrity

#### Bandwidth Utilization
- **SWD**: More efficient packet structure reduces protocol overhead
- **JTAG**: Higher overhead due to TAP state machine requirements

## Debugging Tools and Software

### probe-rs Integration

probe-rs provides comprehensive debugging capabilities for the micro:bit v2:

```bash
# List all connected debug probes
probe-rs list

# Flash application and start debugging session
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/main

# Interactive debugging with GDB compatibility
probe-rs gdb --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/main

# Real-time terminal output (if RTT is enabled)
probe-rs rtt --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/main
```

#### probe-rs Architecture
```
┌─────────────────┐    USB     ┌─────────────────┐    SWD     ┌─────────────────┐
│   probe-rs      │ ←────────→ │ nRF52820        │ ←────────→ │ nRF52833        │
│   (Rust tool)   │            │ (Interface MCU) │            │ (Target MCU)    │
└─────────────────┘            └─────────────────┘            └─────────────────┘
```

### GDB Integration

For traditional debugging workflows, probe-rs provides GDB server functionality:

```bash
# Start GDB server
probe-rs gdb --chip nRF52833_xxAA --protocol swd

# In separate terminal, connect with GDB
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/main
(gdb) target remote localhost:1337
(gdb) load
(gdb) break main
(gdb) continue
```

### VS Code Debugging Configuration

Integration with VS Code provides a graphical debugging experience:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "microbit-debug",
            "cwd": "${workspaceFolder}",
            "chip": "nRF52833_xxAA",
            "flashingConfig": {
                "flashingEnabled": true,
                "resetAfterFlashing": true,
                "haltAfterReset": true
            },
            "program": "${workspaceFolder}/target/thumbv7em-none-eabihf/debug/main",
            "runtimeArgs": [
                "--chip",
                "nRF52833_xxAA"
            ]
        }
    ]
}
```

## RTT (Real-Time Transfer) Debugging

RTT provides a non-intrusive method for getting debug output from your running application.

### RTT Configuration

Enable RTT in your Rust application:

```rust
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    rprintln!("Hello, RTT!");
    
    loop {
        rprintln!("Debug value: {}", some_variable);
        // Your application logic
    }
}
```

### RTT vs Traditional Serial Output

| Feature | RTT | UART Serial |
|---------|-----|-------------|
| **Setup Complexity** | Minimal | Requires pin configuration |
| **Performance Impact** | Very low | Higher due to interrupt handling |
| **Bidirectional** | Yes | Yes |
| **External Hardware** | None (uses debug interface) | USB-to-serial adapter |
| **Speed** | Very fast (~1 MB/s) | Limited by baud rate (9600-115200 bps) |

### RTT Protocol Architecture

For developers interested in understanding how RTT works internally, this section explains the underlying protocol architecture and communication mechanisms.

RTT leverages the standard ARM debug infrastructure already present on the micro:bit v2, requiring no special hardware or firmware support from the target application.

#### Communication Stack

```
Development PC              Interface MCU            Target MCU
┌──────────────┐           ┌──────────────┐        ┌──────────────┐
│  probe-rs    │           │  nRF52820    │        │  nRF52833    │
│              │           │              │        │              │
│ ┌──────────┐ │  USB      │ ┌──────────┐ │  SWD   │ ┌──────────┐ │
│ │RTT Client│◄├───────────┤►│CMSIS-DAP │◄├────────┤►│RAM Buffers│ │
│ └──────────┘ │           │ │ Firmware │ │        │ │(RTT Ctrl)│ │
└──────────────┘           │ └──────────┘ │        │ └──────────┘ │
                           └──────────────┘        └──────────────┘
```

#### CMSIS-DAP Protocol

The nRF52820 interface MCU implements **CMSIS-DAP** (Cortex Microcontroller Software Interface Standard - Debug Access Port), an ARM-standardized protocol for USB-based debug adapters.

**Key Characteristics:**
- **Vendor-Neutral**: Open standard implemented by multiple vendors
- **USB HID Transport**: Uses standard USB Human Interface Device class
- **Command-Based**: Simple packet-based request/response protocol
- **SWD Translation**: Converts USB commands into SWD transactions

**Protocol Layers:**
```
Application (probe-rs)
       ↓
CMSIS-DAP Commands (USB HID packets)
       ↓
USB Controller Driver
       ↓
[nRF52820 Interface MCU]
       ↓
SWD Protocol Implementation
       ↓
Physical SWD Interface (SWCLK + SWDIO)
       ↓
Target MCU Debug Port
```

#### Interface MCU Transparency

The nRF52820 interface MCU functions as a **transparent protocol bridge** with no knowledge of higher-level debug protocols like RTT:

**What the Interface MCU Does:**
- Receives CMSIS-DAP commands via USB
- Translates commands into SWD transactions
- Executes memory read/write operations on target
- Returns results via USB

**What the Interface MCU Does NOT Do:**
- Parse or interpret RTT data structures
- Buffer or process RTT messages
- Require special RTT firmware support
- Know anything about the target application logic

This transparency means:
- The interface MCU works with any ARM Cortex-M target
- No firmware updates needed for new debug protocols
- The target application has full control over its memory

#### RTT Discovery and Operation

The host debugging tool (probe-rs) performs all RTT-specific operations:

**1. RTT Control Block Discovery**

When probe-rs starts an RTT session, it scans the target's RAM to locate the RTT control block:

```
Target RAM (nRF52833)
┌─────────────────────────┐ 0x20000000
│                         │
│  Application Data       │
│                         │
├─────────────────────────┤
│  RTT Control Block      │ ← probe-rs scans for this
│  ┌───────────────────┐  │
│  │ "SEGGER RTT"      │  │ (16-byte signature)
│  │ Buffer Count      │  │
│  │ Up Buffer 0 ptr   │  │
│  │ Down Buffer 0 ptr │  │
│  └───────────────────┘  │
├─────────────────────────┤
│  RTT Buffer Memory      │
│  ┌───────────────────┐  │
│  │ Output Buffer     │  │ (application writes here)
│  │ Input Buffer      │  │ (application reads here)
│  └───────────────────┘  │
└─────────────────────────┘ 0x20040000

Process:
1. probe-rs reads RAM in chunks via SWD
2. Searches for "SEGGER RTT" signature
3. Parses control block structure
4. Identifies buffer locations and sizes
```

**2. Runtime Communication**

Once the control block is located, communication occurs through continuous polling:

```
┌──────────────┐                    ┌──────────────┐
│  probe-rs    │                    │  Application │
│  (Host PC)   │                    │  (nRF52833)  │
└──────┬───────┘                    └──────┬───────┘
       │                                   │
       │  1. Read buffer write pointer ────┤
       │◄──────────────────────────────────│
       │                                   │
       │  2. Read buffer read pointer      │
       │◄──────────────────────────────────│
       │                                   │
       │  3. Calculate available data      │
       │     (write_ptr - read_ptr)        │
       │                                   │
       │  4. Read buffer contents ─────────┤
       │◄──────────────────────────────────│
       │                                   │
       │  5. Update read pointer           │
       ├──────────────────────────────────►│
       │                                   │
       │  6. Display data to user          │
       │                                   │
       │  ```
       │  [Repeat at ~1kHz polling rate]   │
```

**Memory Operations via SWD:**
- All operations use standard SWD memory read/write commands
- No special RTT-specific commands required at the protocol level
- Interface MCU simply forwards memory access requests
- Polling occurs continuously at approximately 1 kHz   │
```

**Memory Operations via SWD:**
- All operations use standard SWD memory read/write commands
- No special RTT commands required
- Interface MCU simply forwards memory access requests
- Polling occurs continuously (typically 1000 Hz)

**Performance Characteristics:**
- **Throughput**: Up to ~1 MB/s with optimal configuration
- **Latency**: ~1 ms typical (limited by polling interval)
- **CPU Impact**: ~0% on target (lock-free ring buffer)
- **Determinism**: Non-intrusive, no interrupts required

#### Zero-Copy Architecture

RTT achieves high performance through a zero-copy design:

**Application Side (Target):**
```rust
rprintln!("Value: {}", x);
// 1. Formats string into local buffer
// 2. Copies to RTT buffer in single operation
// 3. Updates write pointer atomically
// 4. Returns immediately (no blocking)
```

**Host Side (probe-rs):**
```
// Continuous background thread polling RTT buffers:
loop {
    read_write_pointer();      // SWD read
    read_read_pointer();       // SWD read
    if (write_ptr != read_ptr) {
        read_buffer_data();    // SWD read (bulk transfer)
        update_read_pointer(); // SWD write
        output_to_terminal();
    }
    sleep(1ms);                // ~1 kHz polling rate
}
```

**Benefits:**
- No interrupts or context switches on target
- No additional buffer copying in target application
- Lock-free synchronization using atomic pointer updates
- Continues operating even when no debugger is connected (buffer wraps)

#### Comparison with Traditional Debug Methods

| Aspect | RTT | Semihosting | UART |
|--------|-----|-------------|------|
| **Transport** | SWD memory access | SWD breakpoint | Dedicated serial pins |
| **CPU Impact** | ~0% | 100% (halts CPU) | ~5% (interrupt overhead) |
| **Speed** | ~1 MB/s | Very slow | 9600-115200 bps |
| **Pins Required** | 0 (uses debug pins) | 0 (uses debug pins) | 2 (TX/RX) |
| **Debugger Required** | Yes | Yes | No |
| **Bidirectional** | Yes | Yes | Yes |
| **Real-time Safe** | Yes | No | Yes (with care) |

### CMSIS-DAP vs Proprietary Debug Protocols

The micro:bit v2 uses CMSIS-DAP, an open standard for debug adapters. Understanding how it compares to proprietary alternatives helps contextualize the design choices:

| Protocol | Vendor | Advantages | Limitations |
|----------|--------|------------|-------------|
| **CMSIS-DAP** | ARM (Open) | Universal, no drivers needed | Standard USB speeds |
| **J-Link** | SEGGER | Very fast, advanced features | Proprietary, expensive |
| **ST-Link** | STMicroelectronics | Optimized for STM32 | Primarily for ST devices |
| **Black Magic Probe** | Open source | Integrated GDB server | Limited to GDB workflow |

The micro:bit's choice of CMSIS-DAP ensures broad tool compatibility and requires no special drivers on modern operating systems.

## Debug Security Considerations

### Production Security

#### Debug Disable Mechanisms
- **Software Disable**: Runtime disabling of debug interface
- **Fuse Programming**: Permanent hardware disabling
- **Access Control**: Cryptographic authentication requirements

#### Secure Debug
- **Debug Authentication**: Challenge-response protocols
- **Privilege Levels**: Different access levels for different debug operations
- **Region Protection**: Limiting debug access to specific memory regions

### Development vs Production

#### Development Configuration
```rust
#[cfg(debug_assertions)]
{
    // RTT initialization
    rtt_init_print!();
    
    // Debug-specific initialization
    init_debug_features();
}
```

#### Production Configuration
```toml
[profile.release]
debug = false           # Remove debug symbols
debug-assertions = false # Disable debug assertions
lto = true             # Link-time optimization
panic = "abort"        # Smaller panic handler
```

## Troubleshooting Debug Issues

### Common Connection Problems

#### USB Enumeration Failures
```bash
# Check if micro:bit is detected
lsusb | grep "0d28:0204"  # Linux/macOS
# or use Windows Device Manager

# Verify probe-rs can see the device
probe-rs list
```

#### SWD Communication Errors
- **Clock Speed**: Reduce SWD frequency if experiencing communication errors
- **Power Supply**: Ensure stable 3.3V supply to target
- **Cable Quality**: Use short, high-quality USB cables
- **Interference**: Keep setup away from sources of electromagnetic interference

### Debugging Rust-Specific Issues

#### Optimization Problems
- **Variable Optimization**: Variables may be optimized away in release builds
- **Function Inlining**: Functions may not appear in stack traces
- **Dead Code Elimination**: Unused code removed by linker

#### Stack Overflow Detection
```rust
// Enable stack overflow detection in debug builds
#[cfg(debug_assertions)]
use cortex_m_rt::{exception, ExceptionFrame};

#[cfg(debug_assertions)]
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("HardFault: {:#?}", ef);
    loop {}
}
```

## Performance Impact of Debugging

### Debug vs Release Builds

#### Code Size Impact
```bash
# Compare binary sizes
arm-none-eabi-size target/thumbv7em-none-eabihf/debug/main
arm-none-eabi-size target/thumbv7em-none-eabihf/release/main

# Typical results:
# Debug:   text: 15KB, data: 200B, bss: 2KB
# Release: text:  8KB, data: 100B, bss: 1KB
```

#### Runtime Performance
- **Debug Builds**: Include bounds checking, overflow checks, and debug symbols
- **Release Builds**: Optimized for size and speed with debug features removed
- **Profiling**: Use release builds for accurate performance measurements

### Debugging Impact on Real-time Systems
- **Breakpoints**: Completely halt system operation
- **Watchpoints**: Minimal performance impact
- **RTT Output**: Very low impact, suitable for real-time debugging
- **Single Stepping**: Significantly alters timing characteristics

This comprehensive debugging guide provides the foundation for effective development and troubleshooting of Rust applications on the BBC micro:bit v2 platform.
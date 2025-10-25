# Example 08 - RTT Bidirectional Communication

Demonstrates bidirectional real-time communication between the micro:bit and your development PC using RTT (Real-Time Transfer). This example shows how to receive input from your PC terminal and respond to it in real-time.

## What it does

This program creates a bidirectional RTT terminal that:
- Receives text input from your PC terminal
- Converts the input to uppercase
- Sends the converted text back to your PC
- Operates in real-time with no blocking or polling loops

## Running this example

### Important: Using `cargo embed` for RTT

**Unlike previous examples**, this example requires `cargo embed` instead of `cargo run`.

**Why `cargo embed`?**
- `cargo run` (via probe-rs) only flashes the binary and exits
- `cargo embed` flashes the binary **and** opens an interactive RTT terminal
- The RTT terminal stays connected, allowing bidirectional communication
- You can type input and see output in real-time

### Quick Start

1. Connect your micro:bit via USB
2. Open a terminal and run:
```bash
cd example_08_rtt_bidirectional
cargo embed
```

3. Wait for the build and flash to complete
4. The RTT terminal will open automatically and display:
```
Ready! Type on the host and press ENTER to send to the target. It will then respond in uppercase.
```

5. Type text and press ENTER to see the uppercase response:

<img width="913" height="576" alt="image" src="https://github.com/user-attachments/assets/96b00725-22a7-4e30-a35e-6b8b7302d9cf" />

<img width="981" height="579" alt="image" src="https://github.com/user-attachments/assets/44af5779-0af4-4865-9d8c-1457ab287e2c" />


Press `Ctrl+C` to exit the RTT terminal.

## The Code

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::timer;
use panic_halt as _;
use rtt_target::{rtt_init, DownChannel, UpChannel};

#[entry]
fn main() -> ! {
    // Create one up (MCU -> host) and one down (host -> MCU) channel.
    let channels = rtt_init! {
        up:   { 0: { size: 1024, name: "log" } }
        down: { 0: { size:   64, name: "stdin" } }
    };

    let mut up: UpChannel = channels.up.0;
    let mut down: DownChannel = channels.down.0;

    let _ = up.write(b"Ready! Type on the host and press ENTER to send to the target. It will then respond in uppercase.\n");

    let mut buf = [0u8; 32];
    loop {
        // Non-blocking read; returns 0 if nothing available.
        let n = down.read(&mut buf);
        if n > 0 {
            // Echo back in uppercase
            for &b in &buf[..n] {
                let _ = up.write(&[b.to_ascii_uppercase()]);
            }
        }
    }
}
```

## How It Works

### RTT Bidirectional Overview

RTT (Real-Time Transfer) is a high-performance debug communication protocol developed by SEGGER. It leverages the debug probe connection (SWD) already present on the micro:bit, enabling bidirectional communication without consuming any GPIO pins.

**Architecture:**
```
Microcontroller (nRF52833)    Debug Probe (nRF52820)    Development PC
┌──────────────────────┐      ┌──────────────────┐      ┌────────────┐
│  Up Buffer (1KB)     │─────►│                  │─────►│  Terminal  │
│  Down Buffer (64B)   │◄─────│  SWD Interface   │◄─────│  (input)   │
│  (RAM circular)      │      │                  │      │            │
└──────────────────────┘      └──────────────────┘      └────────────┘
```

### Code Breakdown

#### Channel Initialization
```rust
let channels = rtt_init! {
    up:   { 0: { size: 1024, name: "log" } }
    down: { 0: { size:   64, name: "stdin" } }
};
```
- **Up channel**: Microcontroller → PC (1KB buffer for output)
- **Down channel**: PC → Microcontroller (64B buffer for input)
- Both use channel 0 and are linked together in a terminal "tab"

#### Extracting Channel Handles
```rust
let mut up: UpChannel = channels.up.0;
let mut down: DownChannel = channels.down.0;
```
- Creates mutable handles for reading and writing
- `up` is used to send data to the PC
- `down` is used to receive data from the PC

#### Sending Initial Message
```rust
let _ = up.write(b"Ready! Type on the host...\n");
```
- Uses `up.write()` to send raw bytes to the host
- The `b"..."` prefix creates a byte string literal
- Must include `\n` for newline (not added automatically)
- Returns a `Result` which we ignore with `let _`

#### Non-blocking Input Reading
```rust
let n = down.read(&mut buf);
```
- **Non-blocking**: Returns immediately with the number of bytes read
- Returns `0` if no data is available (no waiting)
- Maximum read size is limited by buffer capacity (32 bytes in this example)
- Efficient for real-time applications that can't block

#### Processing and Responding
```rust
for &b in &buf[..n] {
    let _ = up.write(&[b.to_ascii_uppercase()]);
}
```
- Converts each received byte to uppercase
- Writes back to the host via the up channel
- Each write is a separate operation (no buffering here)
- Non-blocking writes ensure responsive behavior

**Note**: This implementation removes the explicit `\n` after echoing. The newline from the user's input (when they press ENTER) is already included in the buffer and gets echoed back uppercase.

## Configuration Notes

The `Embed.toml` file configures RTT behavior:

```toml
[default.rtt]
enabled = true

# Up channels: Output from microcontroller to PC
up_channels = [
    { channel = 0, name = "Terminal", up_mode = "NoBlockSkip", format = "String" },
]

# Down channels: Input from PC to microcontroller
down_channels = [
    { channel = 0, name = "Terminal", format = "String" }
]

# Tabs: Link channels together for unified terminal interface
tabs = [
    { up_channel = 0, down_channel = 0, name = "term" }
]
```

**up_mode Options:**
- `NoBlockSkip`: Drop new data if buffer full (prevents blocking)
- `NoBlockTrim`: Overwrite oldest data if buffer full
- `BlockIfFull`: Wait for space (can hang your application!)

## Additional Resources

- **[RTT Target Documentation](https://docs.rs/rtt-target/)** - Complete API reference
- **[probe-rs Documentation](https://probe.rs/)** - probe-rs tool and debugging guide
- **[Example 07: RTT Output Only](../example_07_rtt_output/)** - Simpler output-only example
- **[Debugging Guide](../debugging.md)** - Deep dive into RTT protocol architecture

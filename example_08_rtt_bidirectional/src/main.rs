#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::timer;
use panic_halt as _;
use rtt_target::{rtt_init, DownChannel, UpChannel};

#[entry]
fn main() -> ! {
    // Create one up (MCU -> host) and one down (host -> MCU) channel.
    // Sizes are up to you; these are reasonable starters.
    let channels = rtt_init! {
        up:   { 0: { size: 1024, name: "log" } }
        down: { 0: { size:   64, name: "stdin" } }
    };

    let mut up: UpChannel = channels.up.0;
    let mut down: DownChannel = channels.down.0;

    let _ = up
        .write(b"Ready! Type on the host and press ENTER to send to the target. It will then respond in uppercase.\n");

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

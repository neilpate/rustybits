#![no_main]
#![no_std]

// ============================================================================
// VECTOR TABLE & RESET HANDLER - Replacing cortex-m-rt
// ============================================================================

// External symbols from linker script
extern "C" {
    static mut _sbss: u32; // Start of .bss section
    static mut _ebss: u32; // End of .bss section
    static mut _sdata: u32; // Start of .data section in RAM
    static mut _edata: u32; // End of .data section in RAM
    static _sidata: u32; // Initial values for .data (in flash)
    static _stack_start: u32; // Initial stack pointer
}

// Reset handler - this is where execution begins after power-on
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // 1. Initialize RAM (.data and .bss sections)

    // Copy .data section from flash to RAM
    let mut src = core::ptr::addr_of!(_sidata);
    let mut dest = core::ptr::addr_of_mut!(_sdata);
    let end_data = core::ptr::addr_of_mut!(_edata);

    while dest < end_data {
        core::ptr::write_volatile(dest, core::ptr::read_volatile(src));
        dest = dest.offset(1);
        src = src.offset(1);
    }

    // Zero out .bss section
    let mut dest = core::ptr::addr_of_mut!(_sbss);
    let end_bss = core::ptr::addr_of_mut!(_ebss);

    while dest < end_bss {
        core::ptr::write_volatile(dest, 0);
        dest = dest.offset(1);
    }

    // 2. Call our main function
    main();
}

// Default handler for unused interrupts
#[no_mangle]
pub extern "C" fn DefaultHandler() -> ! {
    loop {}
}

// ARM Cortex-M Vector Table - using function pointers
// This MUST be placed at address 0x00000000 (start of flash)
#[repr(C)]
pub struct VectorTable {
    pub stack_pointer: u32,
    pub reset: unsafe extern "C" fn() -> !,
    pub nmi: unsafe extern "C" fn() -> !,
    pub hard_fault: unsafe extern "C" fn() -> !,
    pub mem_manage: unsafe extern "C" fn() -> !,
    pub bus_fault: unsafe extern "C" fn() -> !,
    pub usage_fault: unsafe extern "C" fn() -> !,
    pub reserved1: [u32; 4],
    pub sv_call: unsafe extern "C" fn() -> !,
    pub debug_monitor: unsafe extern "C" fn() -> !,
    pub reserved2: u32,
    pub pend_sv: unsafe extern "C" fn() -> !,
    pub sys_tick: unsafe extern "C" fn() -> !,
}

#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    stack_pointer: 0x20020000, // End of 128K RAM
    reset: Reset,
    nmi: DefaultHandler,
    hard_fault: DefaultHandler,
    mem_manage: DefaultHandler,
    bus_fault: DefaultHandler,
    usage_fault: DefaultHandler,
    reserved1: [0; 4],
    sv_call: DefaultHandler,
    debug_monitor: DefaultHandler,
    reserved2: 0,
    pend_sv: DefaultHandler,
    sys_tick: DefaultHandler,
};

// Custom panic handler - replaces panic-halt crate
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // Just halt - minimal panic handling
    loop {}
}

// GPIO registers for nRF52833
const GPIO_P0_OUTSET: *mut u32 = 0x5000_0508 as *mut u32;
const GPIO_P0_OUTCLR: *mut u32 = 0x5000_050C as *mut u32;
const GPIO_P0_PIN_CNF: *mut u32 = 0x5000_0700 as *mut u32;

// micro:bit LED matrix pins
const ROW1_PIN: u32 = 21; // P0.21
const COL1_PIN: u32 = 28; // P0.28

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Configure P0.21 (Row 1) as output
        let pin_cnf_21 = GPIO_P0_PIN_CNF.add(ROW1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_21, 1); // DIR=1 (output)

        // Configure P0.28 (Col 1) as output and set low (column active)
        let pin_cnf_28 = GPIO_P0_PIN_CNF.add(COL1_PIN as usize);
        core::ptr::write_volatile(pin_cnf_28, 1); // DIR=1 (output)
        core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << COL1_PIN); // Set column active
    }

    loop {
        unsafe {
            // Turn LED on (set row low)
            core::ptr::write_volatile(GPIO_P0_OUTCLR, 1 << ROW1_PIN);
        }

        // Delay ~1s (on time)
        for _ in 0..400_00 {
            unsafe {
                core::arch::asm!("nop");
            }
        }

        unsafe {
            // Turn LED off (set row high)
            core::ptr::write_volatile(GPIO_P0_OUTSET, 1 << ROW1_PIN);
        }

        // Delay ~1s (off time)
        for _ in 0..800_00 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

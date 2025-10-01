#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{Error, Write};
use core::panic::PanicInfo;

// --- UART Driver ---

const UART_BASE: usize = 0x0900_0000;

// Register offsets from the PL011 Technical Reference Manual
const UART_DR:   usize = 0x00; // Data Register
const UART_FR:   usize = 0x18; // Flag Register
const UART_IBRD: usize = 0x24; // Integer Baud Rate Divisor
const UART_FBRD: usize = 0x28; // Fractional Baud Rate Divisor
const UART_LCRH: usize = 0x2C; // Line Control Register
const UART_CR:   usize = 0x30; // Control Register

struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    /// Initializes the UART. This sequence is from the PL011 technical manual and includes
    /// necessary memory barriers to prevent instruction reordering.
    pub fn init(&self) {
        let cr = (self.base_address + UART_CR) as *mut u32;
        let ibrd = (self.base_address + UART_IBRD) as *mut u32;
        let fbrd = (self.base_address + UART_FBRD) as *mut u32;
        let lcrh = (self.base_address + UART_LCRH) as *mut u32;

        unsafe {
            // Data Synchronization Barrier
            asm!("dsb sy");

            // 1. Disable the UART.
            cr.write_volatile(0x0);

            // 2. Set the baud rate (115200) for a 24MHz clock.
            ibrd.write_volatile(13);
            fbrd.write_volatile(1);

            // 3. Set line control: 8 data bits, enable FIFOs.
            lcrh.write_volatile((1 << 4) | (0b11 << 5));

            // 4. Enable the UART, transmitter, and receiver.
            cr.write_volatile((1 << 0) | (1 << 8) | (1 << 9));

            // Instruction Synchronization Barrier
            asm!("isb");
        }
    }

    /// Writes a single byte to the UART.
    pub fn putc(&self, c: u8) {
        let dr = (self.base_address + UART_DR) as *mut u8;
        let fr = (self.base_address + UART_FR) as *const u32;

        unsafe {
            // Wait until the transmit FIFO is not full.
            while fr.read_volatile() & (1 << 5) != 0 {}
            dr.write_volatile(c);
        }
    }
}

// Implement the `Write` trait for our Uart struct to allow using the `write!` macro.
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

// --- Kernel ---

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut uart = Uart::new(UART_BASE);
    uart.init();

    // Now we can use the `write!` macro for formatted printing!
    write!(uart, "Hello from lrOS!\n").unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = Uart::new(UART_BASE);
    // No need to initialize again if it's already done, but it's safe to do so.
    uart.init();

    write!(uart, "Kernel Panic!\n").unwrap();
    if let Some(location) = info.location() {
        write!(
            uart,
            "Panic in file {} at line {}\n",
            location.file(),
            location.line()
        )
        .unwrap();
    }

    loop {}
}
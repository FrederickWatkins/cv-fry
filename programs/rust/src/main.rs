#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("../../../programs/entry.S"));

const WIDTH: usize = 80;
const LENGTH: usize = 25;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let buffer = unsafe { &mut *(0xb8000 as *mut [u16; WIDTH*LENGTH])};
    for (i, &b) in b"Hello World!".iter().enumerate() {
        buffer[i] = (0x02 << 8) | b as u16; // light grey on black
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
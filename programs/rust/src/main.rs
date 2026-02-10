#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("../../../programs/entry.S"));

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let a: u32 = 10;
    let b: u32 = 32;
    let _c = a + b;

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
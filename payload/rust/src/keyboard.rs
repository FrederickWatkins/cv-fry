// MIPS memory mapped keyboard

use core::ptr;

pub struct Keyboard {
    control: *mut u8,
    keycode: *const u8,
}

impl Keyboard {
    pub fn new(offset: usize) -> Self {
        Self {
            control: offset as *mut u8,
            keycode: (offset + 4) as *const u8,
        }
    }

    pub fn read_keycode(&mut self) -> Option<u8> {
        //println!("Attempted to read keycode {} {}", self.control, self.keycode);
        unsafe {
            if ptr::read_volatile(self.control) & 0b1 == 1 {
                ptr::write_volatile(self.control, ptr::read_volatile(self.control) ^ 0b1);
                Some(*self.keycode)
            } else {
                None
            }
        }
    }
}

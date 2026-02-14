// MIPS memory mapped keyboard

pub struct Keyboard {
    control: &'static mut u8,
    keycode: &'static u8
}

impl Keyboard {
    pub fn new(offset: usize) -> Self {
        Self {control: unsafe {&mut *(offset as *mut u8)}, keycode: unsafe {&*((offset + 4) as *const u8)}}
    }

    pub fn read_keycode(&mut self) -> Option<char> {
        if *self.control & 0b1 == 1 {
            *self.control = *self.control ^ 0b1;
            Some(*self.keycode as char)
        } else {
            None
        }
    }
}
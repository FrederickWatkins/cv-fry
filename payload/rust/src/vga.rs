use core::fmt::Write;
use core::arch::asm;
use core::panic::PanicInfo;

#[derive(Clone, Copy)]
pub enum Colour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagneta,
    Yellow,
    White,
}

pub struct Buffer {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    slice: &'static mut [u16],
    fg_colour: Colour,
    bg_colour: Colour,
    cursor_colour: Colour,
}

impl Buffer {
    pub const fn new(
        width: usize,
        height: usize,
        offset: usize,
        fg_colour: Colour,
        bg_colour: Colour,
        cursor_colour: Colour,
    ) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
            slice: unsafe { core::slice::from_raw_parts_mut(offset as *mut u16, width * height) },
            fg_colour,
            bg_colour,
            cursor_colour,
        }
    }

    fn write_ascii(&mut self, c: u8) {
        if c != b'\n' {
            self.slice[self.x + self.y * self.width] =
                c as u16 | (self.bg_colour as u16) << 12 | (self.fg_colour as u16) << 8;
        } else {
            self.slice[self.x + self.y * self.width] =
                (self.bg_colour as u16) << 12 | (self.fg_colour as u16) << 8;
        }
        if self.x == self.width - 1 || c == b'\n' {
            if self.y == self.height - 1 {
                self.scroll();
            } else {
                self.y += 1;
            }
            self.x = 0;
        } else {
            self.x += 1;
        }
        self.slice[self.x + self.y * self.width] = (self.cursor_colour as u16) << 12;
        for _ in 0..300 {
            unsafe {
                asm!(
                    // Assembly instructions go here
                    "nop",
                );
            }
        }
    }

    fn scroll(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                self.slice[x + (y - 1) * self.width] = self.slice[x + y * self.width];
                if y == self.height - 1 {
                    self.slice[x + y * self.width] = 0;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.slice[x + y * self.width] = 0;
            }
        }
        self.x = 0;
        self.y = 0;
    }
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            self.write_ascii(*c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


static mut BUFFER: Option<Buffer> = None;

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        if (&raw mut BUFFER).as_mut().unwrap().as_mut().is_none() {
            BUFFER = Some(Buffer::new(80, 25, 0xb8000, Colour::Green, Colour::Black, Colour::Cyan))
        }
        (&raw mut BUFFER)
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap()
            .write_fmt(args)
            .unwrap();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe{(&raw mut BUFFER)
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap().clear();}
    print!("panic! {}", info.message());
    if let Some(location) = info.location() {
        println!(" @ {location}");
    }
    loop {}
}

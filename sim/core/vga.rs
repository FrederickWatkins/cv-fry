use bracket_lib::prelude::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

struct State {
    vga_buffer: Vec<u8>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Clear the screen
        
        // 1. Render the buffer
        for (i, chunk) in self.vga_buffer.chunks(2).enumerate() {
            let x = (i % WIDTH) as i32;
            let y = (i / WIDTH) as i32;

            let glyph = chunk[0];
            let attr = chunk[1];

            // Extract foreground (low 4 bits) and background (high 4 bits)
            let fg_idx = attr & 0x0F;
            let bg_idx = (attr & 0xF0) >> 4;

            // Map palette indices to actual RGB colors
            let fg = u8_to_cp437_color(fg_idx);
            let bg = u8_to_cp437_color(bg_idx);

            ctx.set(x, y, fg, bg, glyph);
        }

        // 2. Handle Keypresses
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => ctx.quit(),
                VirtualKeyCode::A => println!("The 'A' key was pressed!"),
                _ => {}
            }
        }
    }
}

// Helper to map 4-bit VGA palette to bracket-lib colors
fn u8_to_cp437_color(idx: u8) -> RGB {
    match idx {
        0 => RGB::from_u8(0, 0, 0),       // Black
        1 => RGB::from_u8(0, 0, 170),     // Blue
        2 => RGB::from_u8(0, 170, 0),     // Green
        3 => RGB::from_u8(0, 170, 170),   // Cyan
        4 => RGB::from_u8(170, 0, 0),     // Red
        5 => RGB::from_u8(170, 0, 170),   // Magenta
        6 => RGB::from_u8(170, 85, 0),    // Brown
        7 => RGB::from_u8(170, 170, 170), // Light Gray
        // ... indices 8-15 (Bright colors)
        15 => RGB::from_u8(255, 255, 255), // White
        _ => RGB::from_u8(85, 85, 85),     // Default Gray
    }
}

fn main() -> BError {
    unsafe {std::env::set_var("WINIT_UNIX_BACKEND", "x11");}
    // We create an 80x25 window using the built-in 8x16 VGA font
    let context = BTermBuilder::new()
        .with_title("VGA Text Mode Emulator")
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(16, 32)
        .with_font("vga8x16.png", 8, 16) // This filename is internal to the crate
        .with_simple_console(WIDTH, HEIGHT, "vga8x16.png")
        .with_advanced_input(true)
        .with_vsync(true)
        .build()?;

    // Dummy buffer: Fill with '!' in light green on black
    let mut fake_buffer = vec![0; WIDTH * HEIGHT * 2];
    for i in 0..(WIDTH * HEIGHT) {
        fake_buffer[i * 2] = 33;    // ASCII '!'
        fake_buffer[i * 2 + 1] = 0x02; // Green foreground, Black background
    }

    let gs = State { vga_buffer: fake_buffer };
    main_loop(context, gs)
}
use cv_fry_sim::core::Core;

use cv_fry_sim::utils::c2c_r::C2cR;
use cv_fry_sim::utils::c2c_w::C2cW;
use cv_fry_sim::utils::dut::DutSync;

use bracket_lib::prelude::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

struct State {
    memory: Vec<u8>,
    core: Core,
    instr_bus: C2cR,
    data_bus_r: C2cR,
    data_bus_w: C2cW,
    cycles_per_refresh: usize,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Clear the screen
        for _ in 0..self.cycles_per_refresh {
            let (instr_ack, instr) = self.instr_bus.respond(
                &self.memory,
                self.core.get_instr_re() == 1,
                self.core.get_instr_sel(),
                self.core.get_instr_addr(),
            );
            self.core.set_instr_ack(instr_ack as u8);
            self.core.set_instr_data(instr);
            let (data_r_ack, data_r) = self.data_bus_r.respond(
                &self.memory,
                self.core.get_dr_re() == 1,
                self.core.get_dr_sel(),
                self.core.get_dr_addr(),
            );
            self.core.set_dr_ack(data_r_ack as u8);
            self.core.set_dr_data(data_r);
            let data_w_ack = self.data_bus_w.respond(
                &mut self.memory,
                self.core.get_dw_we() == 1,
                self.core.get_dw_sel(),
                self.core.get_dw_addr(),
                self.core.get_dw_data(),
            );
            self.core.set_dw_ack(data_w_ack as u8);
            self.core.tick();
        }
        
        // 1. Render the buffer
        for (i, chunk) in self.memory[0xb8000..0xb8000 + WIDTH * HEIGHT * 2].chunks(2).enumerate() {
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
    let context = BTermBuilder::new()
        .with_title("VGA Text Mode Emulator")
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(16, 32)
        .with_font("vga8x16.png", 8, 16) // This filename is internal to the crate
        .with_simple_console(WIDTH, HEIGHT, "vga8x16.png")
        .with_advanced_input(true)
        .build()?;

    let mut core = Core::new();
    core.trace_init("core.vcd");
    let binary = env!("PAYLOAD_CV-FRY-PAYLOAD-RS");
    let mut memory = std::fs::read(binary).unwrap();
    memory.resize(0x1000000, 0);
    let instr_bus = C2cR::new(0);
    let data_bus_r = C2cR::new(0);
    let data_bus_w = C2cW::new(0);
    core.reset();
    let gs = State { memory, core, instr_bus, data_bus_r, data_bus_w, cycles_per_refresh: 10000 };
    main_loop(context, gs)
}
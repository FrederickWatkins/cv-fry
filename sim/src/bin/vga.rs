use cv_fry_sim::core::Core;
use cv_fry_sim::bus::c2c_instr::C2cInstr;
use cv_fry_sim::bus::c2c_data::C2cData;
use cv_fry_sim::utils::dut::DutSync;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::time::Duration;
use sdl2::hint;

// VGA Constants
const COLS: usize = 80;
const ROWS: usize = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 16;
const SCREEN_WIDTH: u32 = COLS as u32 * CHAR_WIDTH;
const SCREEN_HEIGHT: u32 = ROWS as u32 * CHAR_HEIGHT;

// VGA Palette (Standard 16 colors)
const PALETTE: [Color; 16] = [
    Color::RGB(0, 0, 0),       // 0: Black
    Color::RGB(0, 0, 170),     // 1: Blue
    Color::RGB(0, 170, 0),     // 2: Green
    Color::RGB(0, 170, 170),   // 3: Cyan
    Color::RGB(170, 0, 0),     // 4: Red
    Color::RGB(170, 0, 170),   // 5: Magenta
    Color::RGB(170, 85, 0),    // 6: Brown
    Color::RGB(170, 170, 170), // 7: Light Gray
    Color::RGB(85, 85, 85),    // 8: Dark Gray
    Color::RGB(85, 85, 255),   // 9: Light Blue
    Color::RGB(85, 255, 85),   // 10: Light Green
    Color::RGB(85, 255, 255),  // 11: Light Cyan
    Color::RGB(255, 85, 85),   // 12: Light Red
    Color::RGB(255, 85, 255),  // 13: Light Magenta
    Color::RGB(255, 255, 85),  // 14: Yellow
    Color::RGB(255, 255, 255), // 15: White
];

struct EmulatorState {
    memory: Vec<u8>,
    core: Core,
    instr_bus: C2cInstr,
    data_bus: C2cData,
    cycles_per_refresh: usize,
}

impl EmulatorState {
    fn new(binary: &[u8]) -> Self {
        let mut core = Core::new();
        core.trace_init("core.vcd");
        
        let mut memory = binary.to_vec();
        memory.resize(0x1000000, 0); // 16MB memory
        
        core.reset();

        Self {
            memory,
            core,
            instr_bus: C2cInstr::new(0),
            data_bus: C2cData::new(0),
            cycles_per_refresh: 2000,
        }
    }

    fn run_cycles(&mut self) {
        for _ in 0..self.cycles_per_refresh {
            // Instruction Bus
            let (instr_ack, instr) = self.instr_bus.respond(
                &self.memory,
                self.core.get_instr_re() == 1,
                self.core.get_instr_sel(),
                self.core.get_instr_addr(),
            );
            self.core.set_instr_ack(instr_ack as u8);
            self.core.set_instr_data(instr);

            // Data Read Bus
            let (data_ack, data_r) = self.data_bus.respond(
                &mut self.memory,
                self.core.get_data_we() == 1,
                self.core.get_data_re() == 1,
                self.core.get_atomic() == 1,
                self.core.get_amo_op(),
                self.core.get_data_sel(),
                self.core.get_data_addr(),
                self.core.get_data_w(),
            );
            self.core.set_data_ack(data_ack as u8);
            self.core.set_data_r(data_r);

            // Tick Core
            self.core.tick();
        }
    }
}

fn main() -> Result<(), String> {
    // 1. Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    hint::set("SDL_RENDER_SCALE_QUALITY", "0");

    let window = video_subsystem
        .window("VGA Text Mode Emulator", SCREEN_WIDTH*2, SCREEN_HEIGHT*2)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    // 2. Load VGA Font
    // We expect a 16x16 grid image. 
    // We use load_surface so we can set the Color Key (transparency) before creating a texture.
    use sdl2::image::LoadSurface;
    let mut font_surface = sdl2::surface::Surface::from_file("vga8x16.png")
        .map_err(|e| format!("Failed to load font: {}", e))?;

    // Set Black (0,0,0) as transparent color key
    font_surface.set_color_key(true, Color::RGB(255, 0, 255))?;
    
    let mut font_texture = texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;

    // 3. Initialize Emulator State
    let binary = env!("PAYLOAD_CV-FRY-PAYLOAD-RS");
    let binary_data = std::fs::read(binary).map_err(|_| "Failed to read binary payload")?;
    let mut state = EmulatorState::new(&binary_data);

    // 4. Main Loop
    let mut event_pump = sdl_context.event_pump()?;
    
    'running: loop {
        // --- Input Handling ---
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(c), .. } => {
                    state.memory[0xb0000] = state.memory[0xb0000] | 1;
                    state.memory[0xb0004] = c.into_i32() as u8;
                }
                _ => {}
            }
        }

        // --- Simulation ---
        state.run_cycles();

        // --- Rendering ---
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Iterate over VGA buffer at 0xB8000
        // Buffer format: [Character, Attribute] repeated
        let vga_buffer = &state.memory[0xb8000..0xb8000 + (COLS * ROWS * 2)];

        for (i, chunk) in vga_buffer.chunks(2).enumerate() {
            let glyph_idx = chunk[0];
            let attr = chunk[1];

            // Decode Colors
            let fg_idx = attr & 0x0F;
            let bg_idx = (attr & 0xF0) >> 4;
            let fg_color = PALETTE[fg_idx as usize];
            let bg_color = PALETTE[bg_idx as usize];

            // Calculate Grid Position
            let col = (i % COLS) as i32;
            let row = (i / COLS) as i32;
            let x = col * CHAR_WIDTH as i32;
            let y = row * CHAR_HEIGHT as i32;

            let dest_rect = Rect::new(x, y, CHAR_WIDTH, CHAR_HEIGHT);

            // 1. Draw Background
            canvas.set_draw_color(bg_color);
            canvas.fill_rect(dest_rect)?;

            // 2. Draw Glyph (Foreground)
            // Calculate source rect in font sheet (assuming 16 columns of chars)
            let src_col = (glyph_idx % 16) as i32;
            let src_row = (glyph_idx / 16) as i32;
            let src_rect = Rect::new(
                src_col * CHAR_WIDTH as i32, 
                src_row * CHAR_HEIGHT as i32, 
                CHAR_WIDTH, 
                CHAR_HEIGHT
            );

            // Tint the texture with the foreground color
            font_texture.set_color_mod(fg_color.r, fg_color.g, fg_color.b);
            canvas.copy(&font_texture, src_rect, dest_rect)?;
        }

        canvas.present();
        
        // Cap framerate roughly (~60 FPS) to prevent 100% CPU usage on the render thread
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
use cv_fry::utils::c2c_r::C2cR;
use cv_fry::utils::c2c_w::C2cW;
use cv_fry::utils::dut::DUT;

fn main() {
    let mut core = Core::new();
    let binary = concat!(env!("OUT_DIR"), "/stresstest.bin");
    let mut memory = std::fs::read(binary).unwrap();
    memory.resize(0x1000000, 0);
    let mut instr_bus = C2cR::new(0);
    let mut data_bus_r = C2cR::new(0);
    let mut data_bus_w = C2cW::new(0);
    let mut instr_ack;
    let mut data_r_ack;
    let mut data_r;
    let mut data_w_ack;
    let mut instr;
    core.reset();
    for _ in 0..1000 {
        (instr_ack, instr) = instr_bus.respond(
            &memory,
            core.get_instr_re() == 1,
            core.get_instr_sel(),
            core.get_instr_addr(),
        );
        core.set_instr_ack(instr_ack as u8);
        core.set_instr_data(instr);
        (data_r_ack, data_r) = data_bus_r.respond(
            &memory,
            core.get_dr_re() == 1,
            core.get_dr_sel(),
            core.get_dr_addr(),
        );
        core.set_dr_ack(data_r_ack as u8);
        core.set_dr_data(data_r);
        data_w_ack = data_bus_w.respond(
            &mut memory,
            core.get_dw_we() == 1,
            core.get_dw_sel(),
            core.get_dw_addr(),
            core.get_dw_data(),
        );
        core.set_dw_ack(data_w_ack as u8);
        core.tick();
    }
    for i in 0..23 {
        println!("{:08x} 0x{:02x}{:02x}{:02x}{:02x}", i*4 +0x00002000, memory[i*4 +0x2003], memory[i*4 +0x2002], memory[i*4 +0x2001], memory[i*4 +0x2000]);
    }
}

unsafe extern "C" {
    fn vcore_init() -> *mut std::ffi::c_void;
    fn vcore_destroy(dut: *mut std::ffi::c_void);
    fn vcore_eval(dut: *mut std::ffi::c_void);

    fn vcore_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_reset_n(dut: *mut std::ffi::c_void, val: u8);

    // Inputs from Memory/Peripherals
    fn vcore_set_instr_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_instr_data(dut: *mut std::ffi::c_void, val: u32);
    fn vcore_set_dr_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_dr_data(dut: *mut std::ffi::c_void, val: u32);
    fn vcore_set_dw_ack(dut: *mut std::ffi::c_void, val: u8);

    // Outputs to Memory/Peripherals
    fn vcore_get_instr_re(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_instr_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_instr_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dr_re(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dr_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dr_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dw_we(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dw_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dw_data(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dw_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_trace_init(
        dut: *mut std::ffi::c_void,
        filename: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
    fn vcore_trace_dump(dut: *mut std::ffi::c_void, time: u64);
    fn vcore_trace_close(dut: *mut std::ffi::c_void);
}

pub struct Core {
    ptr: *mut std::ffi::c_void,
    vcd: *mut std::ffi::c_void,
    pub time: u64,
}

impl Core {
    pub fn new() -> Self {
        let ptr = unsafe { vcore_init() };
        let str = std::ffi::CString::new("core.vcd").unwrap();
        Self {
            ptr: ptr,
            vcd: unsafe { vcore_trace_init(ptr, str.as_ptr()) },
            time: 0,
        }
    }

    // Bus Input Setters
    pub fn set_instr_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_instr_ack(self.ptr, val);
        }
    }
    pub fn set_instr_data(&mut self, val: u32) {
        unsafe {
            vcore_set_instr_data(self.ptr, val);
        }
    }
    pub fn set_dr_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_dr_ack(self.ptr, val);
        }
    }
    pub fn set_dr_data(&mut self, val: u32) {
        unsafe {
            vcore_set_dr_data(self.ptr, val);
        }
    }
    pub fn set_dw_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_dw_ack(self.ptr, val);
        }
    }

    // Bus Output Getters
    pub fn get_instr_re(&self) -> u8 {
        unsafe { vcore_get_instr_re(self.ptr) }
    }
    pub fn get_instr_addr(&self) -> u32 {
        unsafe { vcore_get_instr_addr(self.ptr) }
    }
    pub fn get_instr_sel(&self) -> u8 {
        unsafe { vcore_get_instr_sel(self.ptr) }
    }
    pub fn get_dr_re(&self) -> u8 {
        unsafe { vcore_get_dr_re(self.ptr) }
    }
    pub fn get_dr_addr(&self) -> u32 {
        unsafe { vcore_get_dr_addr(self.ptr) }
    }
    pub fn get_dr_sel(&self) -> u8 {
        unsafe { vcore_get_dr_sel(self.ptr) }
    }
    pub fn get_dw_we(&self) -> u8 {
        unsafe { vcore_get_dw_we(self.ptr) }
    }
    pub fn get_dw_addr(&self) -> u32 {
        unsafe { vcore_get_dw_addr(self.ptr) }
    }
    pub fn get_dw_data(&self) -> u32 {
        unsafe { vcore_get_dw_data(self.ptr) }
    }
    pub fn get_dw_sel(&self) -> u8 {
        unsafe { vcore_get_dw_sel(self.ptr) }
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe {
            vcore_trace_close(self.vcd);
            vcore_destroy(self.ptr)
        };
    }
}

impl DUT for Core {
    fn set_clk(&mut self, val: u8) {
        unsafe {
            vcore_set_clk(self.ptr, val);
        }
    }

    fn eval(&mut self) {
        unsafe {
            vcore_eval(self.ptr);
            vcore_trace_dump(self.vcd, self.time);
        }
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn reset(&mut self) {
        unsafe {
            self.set_clk(0);
            self.eval();
            vcore_set_reset_n(self.ptr, 0);
            self.tick();
            vcore_set_reset_n(self.ptr, 1);
            self.eval();
        }
    }
}

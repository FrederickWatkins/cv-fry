use cv_fry_sim::core::Core;
use cv_fry_sim::utils::c2c_r::C2cR;
use cv_fry_sim::utils::c2c_w::C2cW;
use cv_fry_sim::utils::dut::DUT;

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
        println!(
            "{:08x} 0x{:02x}{:02x}{:02x}{:02x}",
            i * 4 + 0x00002000,
            memory[i * 4 + 0x2003],
            memory[i * 4 + 0x2002],
            memory[i * 4 + 0x2001],
            memory[i * 4 + 0x2000]
        );
    }
}

use cv_fry::{lsu::Lsu, utils::dut::DUT};

fn main() {
    let mut lsu = Lsu::new();
    lsu.reset();
    lsu.set_mm_re(1);
    lsu.tick();
    println!("Hello world! {}", lsu.get_stall());
}
pub trait DUT {
    fn set_clk(&mut self, val: u8);
    fn reset(&mut self);
    fn eval(&mut self);
    fn timestep(&mut self);
    fn dump_trace(&self);

    fn tick(&mut self) {
        self.eval();
        self.dump_trace();
        // Falling edge
        self.set_clk(0);
        self.eval();
        self.timestep();
        self.dump_trace();

        // 1. Rising edge
        self.set_clk(1);
        self.eval();
        self.timestep();
    }
}
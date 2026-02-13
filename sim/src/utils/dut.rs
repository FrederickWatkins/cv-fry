pub trait DutComb {
    fn eval(&mut self);
}

pub trait DutSync: DutComb {
    fn trace_init(&mut self, filename: &str);

    fn trace_dump(&mut self);

    fn trace_close(&mut self);

    fn set_clk(&mut self, val: u8);

    fn reset(&mut self);

    fn timestep(&mut self);

    fn tick(&mut self) {
        // Falling edge
        self.set_clk(0);
        self.eval();
        self.trace_dump();
        self.timestep();

        // Rising edge
        self.set_clk(1);
        self.eval();
        self.trace_dump();
        self.timestep();
    }
}

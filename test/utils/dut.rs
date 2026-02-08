pub trait DUT {
    fn set_clk(&mut self, val: u8);
    fn reset(&mut self);
    fn eval(&mut self);
    fn timestep(&mut self);

    fn tick(&mut self) {
        // Falling edge
        self.set_clk(0);
        self.eval();
        self.timestep();

        // Rising edge
        self.set_clk(1);
        self.eval();
        self.timestep();
    }
}
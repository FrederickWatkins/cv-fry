pub struct C2cR {
    delay: u32,
    request_latency: u32,
}

impl C2cR {
    pub fn new(delay: u32) -> Self {
        Self {delay: delay, request_latency: delay}
    }

    pub fn respond(&mut self, memory: &[u8], re: bool, sel: u8, addr: u32) -> (bool, u32) {
        if re {
            if self.request_latency == 0 {
                let mut response = 0;
                for i in 0..4 {
                    if (sel >> i) & 1 == 1 {
                        response |= (memory[addr as usize + i] as u32) << (i*8);
                    }
                }
                self.request_latency = self.delay;
                (true, response)
            } else {
                self.request_latency -= 1;
                (false, 0xDEADBEEF)
            }
        } else {
            self.request_latency = self.delay;
            (false, 0xDEADBEEF)
        }
    }
}

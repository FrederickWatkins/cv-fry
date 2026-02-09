pub struct C2cW {
    delay: u32,
    request_latency: u32,
}

impl C2cW {
    pub fn new(delay: u32) -> Self {
        Self {delay: delay, request_latency: delay}
    }

    pub fn respond(&mut self, memory: &mut [u8], we: bool, sel: u8, addr: u32, data: u32) -> bool {
        if we {
            if self.request_latency == 0 {
                for i in 0..4 {
                    if (sel >> i) & 1 == 1 {
                        memory[addr as usize + i] = ((data >> i) & 0xFF) as u8;
                    }
                }
                self.request_latency = self.delay;
                true
            } else {
                self.request_latency -= 1;
                true
            }
        } else {
            self.request_latency = self.delay;
            true
        }
    }
}

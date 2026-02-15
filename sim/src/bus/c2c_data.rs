pub struct C2cData {
    delay: u64,
    request_latency: u64,
}

impl C2cData {
    pub fn new(delay: u64) -> Self {
        Self {delay: delay, request_latency: delay}
    }

    pub fn respond(&mut self, memory: &mut [u8], we: bool, re: bool, sel: u8, addr: u64, data: u64) -> (bool, u64) {
        if we {
            if self.request_latency == 0 {
                for i in 0..8 {
                    if (sel >> i) & 1 == 1 {
                        memory[addr as usize + i] = ((data >> i * 8) & 0xFF) as u8;
                    }
                }
                self.request_latency = self.delay;
                (true, 0xDEADBEEF)
            } else {
                self.request_latency -= 1;
                (false, 0xDEADBEEF)
            }
        } else if re {
            if self.request_latency == 0 {
                let mut response = 0;
                for i in 0..8 {
                    if (sel >> i) & 1 == 1 {
                        response |= (memory[addr as usize + i] as u64) << (i*8);
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

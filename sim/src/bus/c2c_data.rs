const LR: u8 = 0b00010;
const SC: u8 = 0b00011;
const SWAP: u8 = 0b00001;
const ADD: u8 = 0b00000;
const XOR: u8 = 0b00100;
const AND: u8 = 0b01100;
const OR: u8 = 0b01000;
const MIN: u8 = 0b10000;
const MAX: u8 = 0b10100;
const MINU: u8 = 0b11000;
const MAXU: u8 = 0b11100;

pub struct C2cData {
    delay: u64,
    request_latency: u64,
}

impl C2cData {
    pub fn new(delay: u64) -> Self {
        Self {
            delay: delay,
            request_latency: delay,
        }
    }

    pub fn respond(
        &mut self,
        memory: &mut [u8],
        we: bool,
        re: bool,
        atomic: bool,
        amo_op: u8,
        sel: u8,
        addr: u64,
        data: u64,
    ) -> (bool, u64) {
        // We don't implement atomics properly but I can't be bothered to write any more testbench
        // code, so I'll do it in the cache controller when I get to it. This IS a hardware project
        if atomic {
            if self.request_latency == 0 {
                let mut response = 0;
                for i in 0..8 {
                    if (sel >> i) & 1 == 1 {
                        response |= (memory[addr as usize + i] as u64) << (i * 8);
                    }
                }
                self.request_latency = self.delay;
                match amo_op {
                    LR => (true, response),
                    SC => {
                        write_buffer(memory, sel, addr, data);
                        (true, 0) // Just always say store was successful
                    }
                    SWAP => {
                        write_buffer(memory, sel, addr, data);
                        (true, response)
                    }
                    ADD => {
                        write_buffer(memory, sel, addr, data + response);
                        (true, response)
                    }
                    XOR => {
                        write_buffer(memory, sel, addr, data ^ response);
                        (true, response)
                    }
                    AND => {
                        write_buffer(memory, sel, addr, data & response);
                        (true, response)
                    }
                    OR => {
                        write_buffer(memory, sel, addr, data | response);
                        (true, response)
                    }
                    MIN => {
                        write_buffer(
                            memory,
                            sel,
                            addr,
                            i64::min(data as i64, response as i64) as u64,
                        );
                        (true, response)
                    }
                    MAX => {
                        write_buffer(
                            memory,
                            sel,
                            addr,
                            i64::max(data as i64, response as i64) as u64,
                        );
                        (true, response)
                    }
                    MINU => {
                        write_buffer(memory, sel, addr, u64::min(data, response));
                        (true, response)
                    }
                    MAXU => {
                        write_buffer(memory, sel, addr, u64::max(data, response));
                        (true, response)
                    }
                    _ => {
                        panic!()
                    }
                }
            } else {
                self.request_latency -= 1;
                (false, 0xDEADBEEF)
            }
        } else if re {
            if self.request_latency == 0 {
                let mut response = 0;
                for i in 0..8 {
                    if (sel >> i) & 1 == 1 {
                        response |= (memory[addr as usize + i] as u64) << (i * 8);
                    }
                }
                self.request_latency = self.delay;
                (true, response)
            } else {
                self.request_latency -= 1;
                (false, 0xDEADBEEF)
            }
        } else if we {
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
        } else {
            self.request_latency = self.delay;
            (false, 0xDEADBEEF)
        }
    }
}

fn write_buffer(memory: &mut [u8], sel: u8, addr: u64, data: u64) {
    for i in 0..8 {
        if (sel >> i) & 1 == 1 {
            memory[addr as usize + i] = ((data >> i * 8) & 0xFF) as u8;
        }
    }
}

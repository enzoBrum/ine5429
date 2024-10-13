use rug::{integer::Order, Integer};

pub trait PRNG {
    fn generate(&mut self, num_bytes: u32) -> Integer;
}

pub struct Xorshift128 {
    x: u64,
    y: u64,
    z: u64,
    w: u64
}

impl Xorshift128 {
    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self {
        Xorshift128 {
            x,
            y,
            z,
            w
        }
    }
}

impl PRNG for Xorshift128 {
    fn generate(&mut self, num_bytes: u32) -> Integer {
        let num_ints = if num_bytes % 8 == 0 { num_bytes/8 } else { num_bytes/8 + 1 };

        let mut generations: Vec<u64> = Vec::new();
        for _ in 0..num_ints {
            let t = self.x^(self.x << 11);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;

            self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
            generations.push(self.w);
        }

        return Integer::from_digits(&generations, Order::Msf).keep_bits(num_bytes/8);
    }
}


pub struct Isaac {
    state: [u32; 256],
    acc: u32,
    brs: u32,
    cnt: u32
}

impl Isaac {
    fn f(&self, a: u32, i: i32) -> u32 {
        match i % 4 {
            0 => a << 13,
            1 => a >> 6,
            2 => a << 2,
            3 => a >> 16,
            _ => unreachable!()
        }
    }

    fn gen256(&mut self) -> [u32; 256] {
        self.cnt += 1;
        self.brs += c;
        
        let mut result: [u32; 256] = [0; 256];
        
        result.iter_mut().for_each(|(i, r)| {
            let x = s;
            self.acc = self.f(self.acc, i) + (s + 128) % 256;

        })
    }
}


//impl PRNG for Isaac {
//    fn generate(&mut self, num_bytes: u32) -> Integer {
//        let num_ints = if num_bytes % 4 == 0 { num_bytes/4 } else { num_bytes/4 + 1 };
//
//        self.cnt += 1;
//        self.
//
//        let mut result: [u32; 256] = [0; 256];
//        for i in 0..256 {
//             
//        }
//    }
//}

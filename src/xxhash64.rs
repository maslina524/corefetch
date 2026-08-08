const PRIME_1: u64 = 11_400_714_785_074_694_791;
const PRIME_2: u64 = 14_029_467_366_897_019_727;
const PRIME_3: u64 = 1_609_587_929_392_839_161;
const PRIME_4: u64 = 9_650_029_242_287_828_579;
const PRIME_5: u64 = 2_870_177_450_012_600_261;

const MAX_BUF_SIZE: usize = 32;
pub struct XXHash64 {
    state: [u64; 4],
    buf: [u8; MAX_BUF_SIZE],
    buf_size: usize,
    total_len: u64,
}

impl XXHash64 {
    pub const fn new(seed: u64) -> Self {
        Self {
            state: [
                seed.wrapping_add(PRIME_1).wrapping_add(PRIME_2),
                seed.wrapping_add(PRIME_2),
                seed,
                seed.wrapping_sub(PRIME_1),
            ],
            buf: [0; MAX_BUF_SIZE],
            buf_size: 0,
            total_len: 0,
        }
    }

    const fn process_single(previous: u64, input: u64) -> u64 {
        previous.wrapping_add(input.wrapping_mul(PRIME_2))
            .rotate_left(31)
            .wrapping_mul(PRIME_1)
    }

    fn process_block(&mut self, data: &[u8; 32]) {
        let block0 = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let block1 = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let block2 = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let block3 = u64::from_le_bytes(data[24..32].try_into().unwrap());

        self.state[0] = Self::process_single(self.state[0], block0);
        self.state[1] = Self::process_single(self.state[1], block1);
        self.state[2] = Self::process_single(self.state[2], block2);
        self.state[3] = Self::process_single(self.state[3], block3);
    }

    pub fn add(&mut self, input: &[u8]) {
        if input.is_empty() {
            return;
        }

        self.total_len += input.len() as u64;
        let mut data = input;
        let mut buf_used = self.buf_size;

        if buf_used + data.len() < MAX_BUF_SIZE {
            self.buf[buf_used..buf_used + data.len()].copy_from_slice(data);
            self.buf_size = buf_used + data.len();
            return;
        }

        if buf_used > 0 {
            let need = MAX_BUF_SIZE - buf_used;
            self.buf[buf_used..MAX_BUF_SIZE].copy_from_slice(&data[..need]);
            data = &data[need..];
            let buf = self.buf;
            self.process_block(&buf);
            self.buf_size = 0;
        }

        while data.len() >= MAX_BUF_SIZE {
            let block = &data[..MAX_BUF_SIZE];
            let block_arr: [u8; 32] = block.try_into().unwrap();
            self.process_block(&block_arr);
            data = &data[MAX_BUF_SIZE..];
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_size = data.len();
        }
    }

    pub fn hash(&self) -> u64 {
        let mut result;

        if self.total_len >= MAX_BUF_SIZE as u64 {
            result = self.state[0].rotate_left(1)
                + self.state[1].rotate_left(7)
                + self.state[2].rotate_left(12)
                + self.state[3].rotate_left(18);

            result ^= Self::process_single(0, self.state[0]);
            result = result.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

            result ^= Self::process_single(0, self.state[1]);
            result = result.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

            result ^= Self::process_single(0, self.state[2]);
            result = result.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

            result ^= Self::process_single(0, self.state[3]);
            result = result.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);
        } else {
            result = self.state[2].wrapping_add(PRIME_5);
        }

        result = result.wrapping_add(self.total_len);

        let mut idx = 0;
        let stop = self.buf_size;

        while idx + 8 <= stop {
            let value = u64::from_le_bytes(
                self.buf[idx..idx + 8].try_into().unwrap()
            );
            result ^= Self::process_single(0, value);
            result = result.rotate_left(27);
            result = result.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);
            idx += 8;
        }

        if idx + 4 <= stop {
            let value = u32::from_le_bytes(
                self.buf[idx..idx + 4].try_into().unwrap()
            ) as u64;
            result ^= value.wrapping_mul(PRIME_1);
            result = result.rotate_left(23);
            result = result.wrapping_mul(PRIME_2).wrapping_add(PRIME_3);
            idx += 4;
        }

        while idx < stop {
            result ^= (self.buf[idx] as u64).wrapping_mul(PRIME_5);
            result = result.rotate_left(11);
            result = result.wrapping_mul(PRIME_1);
            idx += 1;
        }

        result ^= result >> 33;
        result = result.wrapping_mul(PRIME_2);
        result ^= result >> 29;
        result = result.wrapping_mul(PRIME_3);
        result ^= result >> 32;

        result
    }

    pub fn hash_bytes(data: &[u8], seed: u64) -> u64 {
        let mut hasher = Self::new(seed);
        hasher.add(data);
        hasher.hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash64() {
        let data = b"Hello World";
        let seed = 0;
        let hash = XXHash64::hash_bytes(data, seed);
        assert_eq!(hash, 0x63_34_d2_07_19_24_5b_c2);
        println!("Hash: {hash:016x}");
    }
}
pub struct Sha256 {
    state: [u32; 8],
    data: Vec<u8>,
    data_len: usize,
    bit_len: u64,
}

impl Sha256 {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    pub fn new() -> Self {
        Sha256 {
            state: [
                0x6a09e667,
                0xbb67ae85,
                0x3c6ef372,
                0xa54ff53a,
                0x510e527f,
                0x9b05688c,
                0x1f83d9ab,
                0x5be0cd19,
            ],
            data: Vec::new(),
            data_len: 0,
            bit_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        for &byte in data {
            self.data.push(byte);
            self.data_len += 1;
            if self.data_len == 64 {
                self.transform();
                self.bit_len += 512;
                self.data_len = 0;
            }
        }
    }

    fn transform(&mut self) {
        let mut m = [0u32; 64];
        let mut i = 0;
        let mut j = 0;

        while i < 16 {
            m[i] = ((self.data[j] as u32) << 24) | ((self.data[j + 1] as u32) << 16)
                | ((self.data[j + 2] as u32) << 8) | (self.data[j + 3] as u32);
            i += 1;
            j += 4;
        }

        while i < 64 {
            let s0 = Self::right_rotate(m[i - 15], 7)
                ^ Self::right_rotate(m[i - 15], 18)
                ^ (m[i - 15] >> 3);
            let s1 = Self::right_rotate(m[i - 2], 17)
                ^ Self::right_rotate(m[i - 2], 19)
                ^ (m[i - 2] >> 10);
            m[i] = m[i - 16]
                .wrapping_add(s0)
                .wrapping_add(m[i - 7])
                .wrapping_add(s1);
            i += 1;
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..64 {
            let s1 = Self::right_rotate(e, 6) ^ Self::right_rotate(e, 11) ^ Self::right_rotate(e, 25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(Self::K[i])
                .wrapping_add(m[i]);
            let s0 = Self::right_rotate(a, 2) ^ Self::right_rotate(a, 13) ^ Self::right_rotate(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);

        self.data.clear();
    }

    fn right_rotate(n: u32, d: u32) -> u32 {
        (n >> d) | (n << (32 - d))
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        let mut i = self.data_len;
        let mut len = self.data_len;
        if self.data_len < 56 {
            self.data.push(0x80);
            i += 1;
        }

        while i < 56 {
            self.data.push(0x00);
            i += 1;
        }

        self.bit_len += (len * 8) as u64;
        for i in 0..8 {
            self.data.push(((self.bit_len >> (56 - (i * 8))) & 0xff) as u8);
            len += 1;
        }

        self.transform();

        let mut hash = [0; 32];
        for i in 0..8 {
            hash[i * 4] = (self.state[i] >> 24) as u8;
            hash[i * 4 + 1] = (self.state[i] >> 16) as u8;
            hash[i * 4 + 2] = (self.state[i] >> 8) as u8;
            hash[i * 4 + 3] = self.state[i] as u8;
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let mut hasher = Sha256::new();
        hasher.update(b"");
        let result = hasher.finalize();
        assert_eq!(
            hex::encode(result),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hello_world() {
        let mut hasher = Sha256::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        assert_eq!(
            hex::encode(result),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
use std::hash::Hasher;

#[derive(Debug, PartialEq)]
pub struct Glowworm {
    s: [u64; 32],
    n: usize,
    /// Temporaries
    t: u64,
    h: u64,
}

impl Default for Glowworm {
    fn default() -> Self {
        Glowworm {
            s: [
                14745948531085624800,
                16120642418826990911,
                9275000239821960485,
                4476743750426018428,
                741912412851399944,
                17767459926458528083,
                2469478127305654386,
                6225995753166195692,
                4750461123551357503,
                10555348896626929070,
                14572814704552083992,
                2824678681307928227,
                8198425675642015085,
                3315257422098907176,
                13762405054855287671,
                15186990245784674763,
                5015234624788364844,
                8462123041350221017,
                9974233762935842858,
                11502466225357323772,
                17531649588530077495,
                8670185664686319238,
                4707560773883213848,
                10843017560065197706,
                17676146699030180721,
                17194224147714809490,
                4745306135015590921,
                11298931964348737593,
                14067901419238702746,
                15452291037738416485,
                591116246257296967,
                15728077675183395515,
            ],
            n: 0,
            t: 5699370651900549022,
            h: 14745948531085624800,
        }
    }
}

impl Glowworm {
    /// Call add_bit to add a new bit to the end and return the hash.
    #[inline]
    pub fn add_bit(&mut self, bit: u64) -> u64 {
        let xor = if bit == 0 { 0 } else { 0xffffffff };
        self.t = self.s[self.n % 32] ^ xor;
        self.t = (self.t | (self.t >> 1)) ^ (self.t << 1);
        self.t ^= (self.t >> 4) ^ (self.t >> 8) ^ (self.t >> 16) ^ (self.t >> 32);
        self.n += 1;
        self.s[self.n % 32] ^= self.t;
        self.s[self.n % 32]
    }

    /// del_bit deletes the last bit; must be passed the last bit of the most recent hash.
    #[inline]
    pub fn del_bit(&mut self, bit: u64) -> u64 {
        self.n -= 1;
        self.add_bit(bit);
        self.n -= 1;
        self.s[self.n % 32]
    }
}

impl Hasher for Glowworm {
    fn finish(&self) -> u64 {
        self.h
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            let byte = *byte as u64;
            self.add_bit(byte & 1);
            self.add_bit(byte & 2);
            self.add_bit(byte & 4);
            self.add_bit(byte & 8);
            self.add_bit(byte & 16);
            self.add_bit(byte & 32);
            self.add_bit(byte & 64);
            self.add_bit(byte & 128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Empty state hash.
    const CHECKVALUE: u64 = 0xCCA4220FC78D45E0;

    #[test]
    fn same_as_default() {
        // Uses the algorithm outline in the paper to initialize the hash.
        let mut a = Glowworm {
            s: [0; 32],
            n: 0,
            t: 1,
            h: 1,
        };

        for _ in 0..4096 {
            a.h = a.add_bit(a.h & 1);
        }
        a.n = 0;
        assert_eq!(a.h, CHECKVALUE);

        let b = Glowworm::default();

        assert_eq!(a, b);
    }

    #[test]
    fn add_delete_is_empty() {
        let mut gw = Glowworm::default();
        assert_eq!(gw.h, CHECKVALUE);

        let hash = gw.add_bit(1);
        assert_eq!(hash, 12612998674271867816);

        let hash = gw.add_bit(0);
        assert_eq!(hash, 3140184078842265393);

        let hash = gw.del_bit(0);
        assert_eq!(hash, 12612998674271867816);

        let hash = gw.del_bit(1);
        assert_eq!(hash, CHECKVALUE);
    }

    #[test]
    fn hasher() {
        let mut gw = Glowworm::default();
        gw.write_u8(u8::MAX);
        let hash = gw.finish();

        // Check that removing all the bits from u8::MAX resets to the empty state.
        for _ in 0..8 {
            gw.del_bit(1);
        }
        assert_eq!(gw.h, CHECKVALUE);

        // check that setting all bits to 1 is the same as adding u8::MAX.
        let mut gw = Glowworm::default();
        for _ in 0..8 {
            gw.add_bit(1);
        }
        assert_eq!(gw.h, hash);

        // check that order of bit insertion is low to high bits.
        let mut gw = Glowworm::default();
        gw.write_u64(u32::MAX as u64);
        let hash = gw.finish();

        let mut gw = Glowworm::default();
        for _ in 0..32 {
            gw.add_bit(1);
        }
        for _ in 0..32 {
            gw.add_bit(0);
        }

        assert_eq!(gw.h, hash);
    }
}

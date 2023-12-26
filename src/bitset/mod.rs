#[derive(Debug, Clone)]
pub struct BitSet {
    num_bits: usize,
    bits: Vec<u8>,
}

impl BitSet {
    pub fn new(v: Option<Vec<bool>>) -> Self {
        let mut bitset = BitSet {
            num_bits: 0,
            bits: Vec::new(),
        };
        if let Some(v) = v {
            bitset.append_bool(v);
        }
        bitset
    }

    pub fn len(&self) -> usize {
        self.num_bits
    }

    pub fn append(&mut self, other: BitSet) {
        self.ensure_capacity(other.num_bits);
        for index in 0..other.bits.len() {
            if other.at(index) {
                println!("bit before: {}", self.bits[self.num_bits/8]);
                self.bits[self.num_bits/8] |= 0x80 >> (self.num_bits%8);
                println!("bit after: {}", self.bits[self.num_bits/8]);
            }
            self.num_bits += 1;
        }
    }

    /// append_bool appends bits to the BitSet
    fn append_bool(&mut self, bits: Vec<bool>) {
        self.ensure_capacity(bits.len());
        for item in bits {
            if item {
                self.bits[self.num_bits/8] |= 0x80 >> (self.num_bits%8);
            }
            self.num_bits += 1;
        }
    }

    /// at returns the value of the bit at |index|
    fn at(&self, index: usize) -> bool {
        (self.bits[index/8] & (0x80 >> (index%8) as u8)) != 0
    }

    /// ensure_capacity ensures the Bitset can store an additional |number_of_bits|.
    ///
    /// The underlying array is expanded if necessary. To prevent frequent
    /// reallocation, expanding the underlying array at least doubles its capacity.
    fn ensure_capacity(&mut self, mut number_of_bits: usize) {
        number_of_bits += self.num_bits;

        let mut new_num_bytes = number_of_bits / 8;
        if number_of_bits%8 != 0 {
            new_num_bytes += 1;
        }

        if self.bits.len() >= new_num_bytes {
            return
        }

        self.bits.resize(new_num_bytes + 2 * self.bits.len(), 0);
    }
}

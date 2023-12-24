#[derive(Debug, Clone)]
pub struct BitSet {
    num_bytes: i32,
    bits: Vec<u8>,
}

impl BitSet {
    pub fn new(v: Option<Vec<bool>>) -> Self {
        let mut bitset = BitSet {
            num_bytes: 0,
            bits: Vec::new(),
        };
        if let Some(v) = v {
            bitset.append_bool(v);
        }
        bitset
    }

    fn append_bool(&mut self, bools: Vec<bool>) {
        self.bits.resize(bools.len(), 0);
        for item in bools {
            if item {
                self.bits[self.num_bytes as usize/8] |= 0x80 >> self.num_bytes%8;
            }
            self.num_bytes += 1
        }
    }

    pub fn len(&self) -> i32 {
        self.num_bytes
    }
}

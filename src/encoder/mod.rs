mod data_mode;

use crate::bitset::BitSet;
use crate::constants::{B0, B1};
use crate::encoder::DataEncoderType::{
    Version1TO9,
    Version10TO26,
    Version27TO40,
};
use data_mode::DataMode;

// A dataEncoder encodes data for a particular QR Code version.
#[derive(Debug)]
pub struct DataEncoder {
    // Minimum & maximum versions supported.
    min_version: i32,
    max_version: i32,

    // Mode indicator bit sequences.
    numeric_mode_indicator: BitSet,
    alphanumeric_mode_indicator: BitSet,
    byte_mode_indicator: BitSet,

    // Character count lengths.
    num_numeric_char_count_bits: i32,
    num_alphanumeric_char_count_bits: i32,
    num_byte_char_count_bits: i32,

    // The raw input data.
    data: Vec<u8>,

    // The data classified into unoptimised segments.
    actual: Vec<Segment>,

    // The data classified into optimised segments.
    optimised: Vec<Segment>
}

#[derive(Debug)]
pub enum DataEncoderType {
    Version1TO9,
    Version10TO26,
    Version27TO40,
}

#[derive(Debug)]
struct Segment {
    // Data Mode (e.g. numeric).
    data_mode: DataMode,

    // segment data (e.g. "abc").
    data: Vec<u8>
}

impl DataEncoder {
    pub fn new(t: DataEncoderType) -> Self {
        match t {
            Version1TO9 => DataEncoder {
                min_version:                      1,
                max_version:                      9,
                numeric_mode_indicator:           BitSet::new(vec![B0, B0, B0, B1]),
                alphanumeric_mode_indicator:      BitSet::new(vec![B0, B0, B1, B0]),
                byte_mode_indicator:              BitSet::new(vec![B0, B1, B0, B0]),
                num_numeric_char_count_bits:      10,
                num_alphanumeric_char_count_bits: 9,
                num_byte_char_count_bits:         8,
                data: vec![],
                actual: vec![],
                optimised: vec![],
            },
            Version10TO26 => DataEncoder{
                min_version:                      10,
                max_version:                      26,
                numeric_mode_indicator:           BitSet::new(vec![B0, B0, B0, B1]),
                alphanumeric_mode_indicator:      BitSet::new(vec![B0, B0, B1, B0]),
                byte_mode_indicator:              BitSet::new(vec![B0, B1, B0, B0]),
                num_numeric_char_count_bits:      12,
                num_alphanumeric_char_count_bits: 11,
                num_byte_char_count_bits:         16,
                data: vec![],
                actual: vec![],
                optimised: vec![],
            },
            Version27TO40 => DataEncoder{
                min_version:                      27,
                max_version:                      40,
                numeric_mode_indicator:           BitSet::new(vec![B0, B0, B0, B1]),
                alphanumeric_mode_indicator:      BitSet::new(vec![B0, B0, B1, B0]),
                byte_mode_indicator:              BitSet::new(vec![B0, B1, B0, B0]),
                num_numeric_char_count_bits:      14,
                num_alphanumeric_char_count_bits: 13,
                num_byte_char_count_bits:         16,
                data: vec![],
                actual: vec![],
                optimised: vec![],
            },
        }
    }

    // mode_indicator returns the segment header bits for a segment of type DataMode.
    fn mode_indicator(&self, mode: DataMode) -> Result<BitSet, String> {
        match mode {
            DataMode::Numeric(_) => Ok(self.numeric_mode_indicator.clone()),
            DataMode::Alphanumeric(_) => Ok(self.alphanumeric_mode_indicator.clone()),
            DataMode::Byte(_) => Ok(self.byte_mode_indicator.clone()),
            _ => Err(String::from("Unknown data mode")),
        }
    }

    // char_count_bits returns the number of bits used to encode the length of a data
    // segment of type DataMode.
    fn char_count_bits(&self, mode: DataMode) -> Result<i32, String> {
        match mode {
            DataMode::Numeric(_) => Ok(self.num_numeric_char_count_bits.clone()),
            DataMode::Alphanumeric(_) => Ok(self.num_alphanumeric_char_count_bits.clone()),
            DataMode::Byte(_) => Ok(self.num_byte_char_count_bits.clone()),
            _ => Err(String::from("Unknown data mode")),
        }
    }

    fn encoded_length(&self, data_mode: DataMode, n: i32) -> Result<i32, String> {
        let mode = self.mode_indicator(data_mode);
        let char_count_bits = self.char_count_bits(data_mode);

        if mode.is_err() {
            return Err(String::from("mode not supported"))
        }

        // TODO: code can panic, will review later
        let max_length = (1 << char_count_bits.clone().unwrap()) - 1;
        if n > max_length {
            return Err(String::from("length too long to be represented"))
        }

        let mut length = mode.unwrap().len() + char_count_bits.unwrap();
        match data_mode {
            // Number: 10 x (Quotient that divides digit number by three) + (Odd=0 then 0, Odd=1 then 4, Odd=2 then 7)
            DataMode::Numeric(_) => {
                length += 10 * (n / 3);
                if n % 3 != 0 {
                    length += 1 + 3 * (n % 3);
                }
            },
            // Alphanumeric: 11 x (Quotient that divides digit number by two) + (Odd=0 then 0, Odd=1then 6)
            DataMode::Alphanumeric(_) => {
                length += 11 * (n / 2);
                length += 6 * (n % 2);
            },
            // Binary: 8 x Character digits
            DataMode::Byte(_) => length += 8 * n,
            _ => {
                return Err(String::from("Unknown data mode"))
            },
        };
        Ok(length)
    }

    pub fn encode(&mut self, data: Vec<u8>) -> Result<DataMode, String> {
        self.data = data;
        if self.data.len() == 0 {
            return Err(String::from("no data to encode"));
        }

        let highest_required_mode = self.classify_data_mode();
        Ok(highest_required_mode)
    }

    fn classify_data_mode(&mut self) -> DataMode {
        let mut start = 0;
        let mut mode = DataMode::none();
        let mut highest_data_mode = DataMode::none();

        for (i, data) in self.data.iter().enumerate() {
            let mut new_mode = DataMode::none();
            new_mode = match data {
                0x30..=0x39 => DataMode::numeric(),
                0x20 | 0x24 | 0x25 | 0x2a | 0x2b | 0x2d | 0x2e | 0x2f | 0x3a | 0x41..=0x5a => DataMode::alphanumeric(),
                _ => DataMode::byte()
            };

            if mode != new_mode {
                if i > 0 {
                    self.actual.append(&mut vec![Segment{data_mode: mode, data: self.data[start..i].to_owned() }]);
                    start = i
                }
                mode = new_mode
            }

            if new_mode >= highest_data_mode {
                highest_data_mode = new_mode
            }
        }
        self.actual.append(&mut vec![Segment{data_mode: mode, data: self.data[start..self.data.len()].to_owned() }]);
        highest_data_mode
    }
}

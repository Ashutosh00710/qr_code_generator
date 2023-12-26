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
    num_numeric_char_count_bits: usize,
    num_alphanumeric_char_count_bits: usize,
    num_byte_char_count_bits: usize,

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
                numeric_mode_indicator:           BitSet::new(Some(vec![B0, B0, B0, B1])),
                alphanumeric_mode_indicator:      BitSet::new(Some(vec![B0, B0, B1, B0])),
                byte_mode_indicator:              BitSet::new(Some(vec![B0, B1, B0, B0])),
                num_numeric_char_count_bits:      10,
                num_alphanumeric_char_count_bits: 9,
                num_byte_char_count_bits:         8,
                data:                             vec![],
                actual:                           vec![],
                optimised:                        vec![],
            },
            Version10TO26 => DataEncoder{
                min_version:                      10,
                max_version:                      26,
                numeric_mode_indicator:           BitSet::new(Some(vec![B0, B0, B0, B1])),
                alphanumeric_mode_indicator:      BitSet::new(Some(vec![B0, B0, B1, B0])),
                byte_mode_indicator:              BitSet::new(Some(vec![B0, B1, B0, B0])),
                num_numeric_char_count_bits:      12,
                num_alphanumeric_char_count_bits: 11,
                num_byte_char_count_bits:         16,
                data:                             vec![],
                actual:                           vec![],
                optimised:                        vec![],
            },
            Version27TO40 => DataEncoder{
                min_version:                      27,
                max_version:                      40,
                numeric_mode_indicator:           BitSet::new(Some(vec![B0, B0, B0, B1])),
                alphanumeric_mode_indicator:      BitSet::new(Some(vec![B0, B0, B1, B0])),
                byte_mode_indicator:              BitSet::new(Some(vec![B0, B1, B0, B0])),
                num_numeric_char_count_bits:      14,
                num_alphanumeric_char_count_bits: 13,
                num_byte_char_count_bits:         16,
                data:                             vec![],
                actual:                           vec![],
                optimised:                        vec![],
            },
        }
    }

    /// mode_indicator returns the segment header bits for a segment of type DataMode.
    fn mode_indicator(&self, mode: DataMode) -> Result<BitSet, String> {
        match mode {
            DataMode::Numeric(_) => Ok(self.numeric_mode_indicator.clone()),
            DataMode::Alphanumeric(_) => Ok(self.alphanumeric_mode_indicator.clone()),
            DataMode::Byte(_) => Ok(self.byte_mode_indicator.clone()),
            _ => Err(String::from("Unknown data mode")),
        }
    }

    /// char_count_bits returns the number of bits used to encode the length of a data
    /// segment of type DataMode.
    fn char_count_bits(&self, mode: DataMode) -> Result<usize, String> {
        match mode {
            DataMode::Numeric(_) => Ok(self.num_numeric_char_count_bits),
            DataMode::Alphanumeric(_) => Ok(self.num_alphanumeric_char_count_bits),
            DataMode::Byte(_) => Ok(self.num_byte_char_count_bits),
            _ => Err(String::from("Unknown data mode")),
        }
    }

    /// encoded_length returns the number of bits required to encode n symbols in
    /// data_mode
    ///
    /// The number of bits required is affected by:
    ///    - QR code type - Mode Indicator length
    ///    - Data mode - number of bits used to represent data length
    ///    - Data mode - how the data is encoded (alphanumeric, numeric)
    ///    - Number of symbols encoded
    ///
    /// An error is returned if the mode is not supported, or the length requested is
    /// too long to be represented
    fn encoded_length(&self, data_mode: DataMode, n: usize) -> Result<usize, String> {
        let mode = self.mode_indicator(data_mode);
        let char_count_bits = self.char_count_bits(data_mode);

        if mode.is_err() {
            return Err(String::from("mode not supported"))
        }

        // TODO: code can panic, will review later
        // explanation behind max_length: char count indicator for version range 1-9 and
        // alphanumeric mode is 9, then the maximum number 9 bits can represent in
        // binary is 511 (binary of 512: 111111111)
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

    /// encode data as one or more segments and return the encoded data
    ///
    /// The returned data does not include the terminator bit sequence
    pub fn encode(&mut self, data: Vec<u8>) -> Result<BitSet, String> {
        self.data = data;
        if self.data.is_empty() {
            return Err(String::from("no data to encode"));
        }
        // classify data into unoptimised segments
        let highest_required_mode = self.classify_data_mode();
        // optimise segments.
        let err = self.optimise_data_mode_of_segments();
        if let Some(err) = err {
            return Err(err)
        }
        let mut optimised_length = 0;
        for segment in self.optimised.iter() {
            let result_length = self.encoded_length(segment.data_mode, segment.data.len())?;
            optimised_length += result_length;
        }
        let single_segment_length_result = self.encoded_length(highest_required_mode, self.data.len());

        match single_segment_length_result {
            Ok(single_segment_length) => {
                if single_segment_length <= optimised_length {
                    self.optimised = vec![Segment { data_mode: highest_required_mode, data: self.data.clone() }];
                }
            },
            Err(err) => return Err(err)
        }

        let mut encoded = BitSet::new(None);
        for segment in self.optimised.iter() {
            self.encode_data_raw(&segment.data, segment.data_mode, &mut encoded);
        }
        Ok(encoded)
    }

    /// encode_data_raw encodes data in data_mode. The encoded data is appended to
    /// encoded
    fn encode_data_raw(&self, data: &Vec<u8>, data_mode: DataMode, encoded: &mut BitSet) {
        let mode = self.mode_indicator(data_mode);
        // let char_count_bits = self.char_count_bits(data_mode);
        println!("{:?}", mode.clone().unwrap());
        encoded.append(mode.unwrap());
    }

    /// optimise_data_mode_of_segments optimises the list of segments to reduce the overall output
    /// encoded data length.
    ///
    /// The algorithm coalesces adjacent segments. segments are only coalesced when
    /// the Data Modes are compatible, and when the coalesced segment has a shorter
    /// encoded length than separate segments.
    ///
    /// Multiple segments may be coalesced. For example a string of alternating
    /// alphanumeric/numeric segments ANANANANA can be optimised to just A.
    fn optimise_data_mode_of_segments(&mut self) -> Option<String> {
        let mut i = 0;
        while i < self.actual.len() {
            let segment = self.actual.get(i).unwrap();
            // not handling Option (for next 2 lines) because I know there will be no
            // segfault in this statement
            let data_mode = segment.data_mode;
            let mut number_of_characters = segment.data.len();
            let mut j = i + 1;
            while j < self.actual.len() {
                let next_segment = self.actual.get(j).unwrap();
                let number_of_characters_for_next_segment = next_segment.data.len();
                let data_mode_for_next_segment = next_segment.data_mode;

                if data_mode < data_mode_for_next_segment {
                    break
                }

                let coalesced_length = self.encoded_length(
                    data_mode_for_next_segment,
                    number_of_characters + number_of_characters_for_next_segment
                ).ok()?;

                let segment_one_length = self.encoded_length(data_mode, number_of_characters).ok()?;

                let segment_two_length = self.encoded_length(
                    data_mode_for_next_segment,
                    number_of_characters_for_next_segment
                ).ok()?;

                if coalesced_length < segment_one_length + segment_two_length {
                    j += 1;
                    number_of_characters += number_of_characters_for_next_segment
                } else {
                    break;
                }
            }

            let mut optimized = Segment{
                data_mode,
                data: vec![],
            };

            for k in i..j {
                optimized.data.append(&mut self.actual.get(k).unwrap().data.clone());
            }

            self.optimised.push(optimized);
            i = j;
        }
        None
    }

    /// classify_data_mode classifies the raw data into unoptimised segments
    ///
    /// e.g. ```"123ZZ#!#!" =>```
    /// ```[numeric, 3, "123"] [alphanumeric, 2, "ZZ"] [byte, 4, "#!#!"]```
    ///
    /// Returns the highest data mode needed to encode the data. e.g. for a mixed
    /// numeric/alphanumeric input, the highest is alphanumeric.
    ///
    /// `data_mode` none < `data_mode` numeric < `data_mode` alphanumeric < `data_mode` byte
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

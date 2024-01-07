mod versions;

use crate::encoder::{DataEncoder, DataEncoderType};
use crate::version::versions::get_versions;

/// Error detection/recovery capacity.
///
/// There are several levels of error detection/recovery capacity. Higher levels
/// of error recovery are able to correct more errors, with the trade-off of
/// increased symbol size.
#[derive(PartialEq)]
pub enum ErrorCorrectionLevels {
    /// Level L: 7% error recovery.
    Low,

    /// Level M: 15% error recovery. Good default choice.
    Medium,

    /// Level Q: 25% error recovery.
    High,

    /// Level H: 30% error recovery.
    Highest,
}

/// QrCodeVersion describes the data length and encoding order of a single QR
/// Code version. There are 40 versions numbers x 4 recovery levels == 160
/// possible qrCodeVersion structures.
pub struct QrCodeVersion {
    /// Version number (1-40 inclusive).
    pub version: i32,

    /// Error recovery level.
    level: ErrorCorrectionLevels,

    data_encoder_type: DataEncoderType,

    /// Encoded data can be split into multiple blocks. Each block contains data
    /// and error recovery bytes.
    ///
    /// Larger QR Codes contain more blocks.
    block: Vec<Block>,

    /// Number of bits required to pad the combined data & error correction bit
    /// stream up to the symbol's full capacity.
    num_remainder_bits: i32,
}

struct Block {
    num_blocks: i32,

    /// Total codewords (numCodewords == numErrorCodewords+numDataCodewords).
    num_codewords: i32,

    /// Number of data codewords.
    num_data_codewords: i32,
}

impl QrCodeVersion {
    // num_data_bits returns the data capacity in bits.
    fn num_data_bits(&self) -> i32 {
        let mut num_data_bits = 0;
        for b in self.block.iter() {
            num_data_bits += 8 * b.num_blocks * b.num_data_codewords; // 8 bits in a byte
        }
        num_data_bits
    }
}

/// choose_qr_code_version chooses the most suitable QR Code version for a stated
/// data length in bits, the error recovery level required, and the data encoder
/// used.
///
/// The chosen QR Code version is the smallest version able to fit numDataBits
/// and the optional terminator bits required by the specified encoder.
///
/// On success the chosen QR Code version is returned.
pub fn choose_qr_code_version(
    level: ErrorCorrectionLevels,
    encoder: &DataEncoder,
    num_data_bits: usize,
) -> Option<QrCodeVersion> {
    let mut chosen_version: Option<QrCodeVersion> = None;
    for v in get_versions() {
        if v.level != level || v.version < encoder.min_version {
            continue;
        } else if v.version > encoder.max_version {
            break;
        }

        let num_free_bits = v.num_data_bits() - num_data_bits as i32;

        if num_free_bits >= 0 {
            chosen_version = Some(v);
            break;
        }
    }
    chosen_version
}

use crate::encoder::DataEncoderType;
use crate::version::{Block, ErrorCorrectionLevels, QrCodeVersion};

pub fn get_versions() -> Vec<QrCodeVersion> {
    vec![
        QrCodeVersion {
            version: 1,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 26,
                num_data_codewords: 19,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 1,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 26,
                num_data_codewords: 16,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 1,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 26,
                num_data_codewords: 13,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 1,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 26,
                num_data_codewords: 9,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 2,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 44,
                num_data_codewords: 34,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 2,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 44,
                num_data_codewords: 28,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 3,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 70,
                num_data_codewords: 55,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 3,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 70,
                num_data_codewords: 44,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 3,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 35,
                num_data_codewords: 17,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 3,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 35,
                num_data_codewords: 13,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 4,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 100,
                num_data_codewords: 80,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 4,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 50,
                num_data_codewords: 32,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 4,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 50,
                num_data_codewords: 24,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 4,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 25,
                num_data_codewords: 9,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 5,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 1,
                num_codewords: 134,
                num_data_codewords: 108,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 5,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 67,
                num_data_codewords: 43,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 6,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 86,
                num_data_codewords: 68,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 6,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 43,
                num_data_codewords: 27,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 6,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 43,
                num_data_codewords: 19,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 6,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 43,
                num_data_codewords: 15,
            }],
            num_remainder_bits: 7,
        },
        QrCodeVersion {
            version: 7,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 98,
                num_data_codewords: 78,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 7,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 49,
                num_data_codewords: 31,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 7,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 32,
                    num_data_codewords: 14,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 33,
                    num_data_codewords: 15,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 7,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 39,
                    num_data_codewords: 13,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 40,
                    num_data_codewords: 14,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 8,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 121,
                num_data_codewords: 97,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 8,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 60,
                    num_data_codewords: 38,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 61,
                    num_data_codewords: 39,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 8,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 40,
                    num_data_codewords: 18,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 41,
                    num_data_codewords: 19,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 8,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 40,
                    num_data_codewords: 14,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 41,
                    num_data_codewords: 15,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 9,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![Block {
                num_blocks: 2,
                num_codewords: 146,
                num_data_codewords: 116,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 9,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 58,
                    num_data_codewords: 36,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 59,
                    num_data_codewords: 37,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 9,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 36,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 37,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 9,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version1TO9,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 36,
                    num_data_codewords: 12,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 37,
                    num_data_codewords: 13,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 10,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 86,
                    num_data_codewords: 68,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 87,
                    num_data_codewords: 69,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 10,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 69,
                    num_data_codewords: 43,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 70,
                    num_data_codewords: 44,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 10,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 43,
                    num_data_codewords: 19,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 44,
                    num_data_codewords: 20,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 10,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 43,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 44,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 11,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 101,
                num_data_codewords: 81,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 11,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 1,
                    num_codewords: 80,
                    num_data_codewords: 50,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 81,
                    num_data_codewords: 51,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 11,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 50,
                    num_data_codewords: 22,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 51,
                    num_data_codewords: 23,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 11,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 36,
                    num_data_codewords: 12,
                },
                Block {
                    num_blocks: 8,
                    num_codewords: 37,
                    num_data_codewords: 13,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 12,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 116,
                    num_data_codewords: 92,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 117,
                    num_data_codewords: 93,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 12,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 58,
                    num_data_codewords: 36,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 59,
                    num_data_codewords: 37,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 12,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 46,
                    num_data_codewords: 20,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 47,
                    num_data_codewords: 21,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 12,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 7,
                    num_codewords: 42,
                    num_data_codewords: 14,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 43,
                    num_data_codewords: 15,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 13,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![Block {
                num_blocks: 4,
                num_codewords: 133,
                num_data_codewords: 107,
            }],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 13,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 59,
                    num_data_codewords: 37,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 60,
                    num_data_codewords: 38,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 13,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 44,
                    num_data_codewords: 20,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 45,
                    num_data_codewords: 21,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 13,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 12,
                    num_codewords: 33,
                    num_data_codewords: 11,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 34,
                    num_data_codewords: 12,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 14,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 145,
                    num_data_codewords: 115,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 14,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 64,
                    num_data_codewords: 40,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 65,
                    num_data_codewords: 41,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 14,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 36,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 37,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 14,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 36,
                    num_data_codewords: 12,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 37,
                    num_data_codewords: 13,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 15,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 109,
                    num_data_codewords: 87,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 110,
                    num_data_codewords: 88,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 15,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 65,
                    num_data_codewords: 41,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 66,
                    num_data_codewords: 42,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 15,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 15,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 36,
                    num_data_codewords: 12,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 37,
                    num_data_codewords: 13,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 16,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 122,
                    num_data_codewords: 98,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 123,
                    num_data_codewords: 99,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 16,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 7,
                    num_codewords: 73,
                    num_data_codewords: 45,
                },
                Block {
                    num_blocks: 3,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 16,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 15,
                    num_codewords: 43,
                    num_data_codewords: 19,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 44,
                    num_data_codewords: 20,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 16,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 13,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 17,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 1,
                    num_codewords: 135,
                    num_data_codewords: 107,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 136,
                    num_data_codewords: 108,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 17,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 10,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 17,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 1,
                    num_codewords: 50,
                    num_data_codewords: 22,
                },
                Block {
                    num_blocks: 15,
                    num_codewords: 51,
                    num_data_codewords: 23,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 17,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 42,
                    num_data_codewords: 14,
                },
                Block {
                    num_blocks: 17,
                    num_codewords: 43,
                    num_data_codewords: 15,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 18,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 150,
                    num_data_codewords: 120,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 151,
                    num_data_codewords: 121,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 18,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 9,
                    num_codewords: 69,
                    num_data_codewords: 43,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 70,
                    num_data_codewords: 44,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 18,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 17,
                    num_codewords: 50,
                    num_data_codewords: 22,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 51,
                    num_data_codewords: 23,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 18,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 42,
                    num_data_codewords: 14,
                },
                Block {
                    num_blocks: 19,
                    num_codewords: 43,
                    num_data_codewords: 15,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 19,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 141,
                    num_data_codewords: 113,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 142,
                    num_data_codewords: 114,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 19,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 70,
                    num_data_codewords: 44,
                },
                Block {
                    num_blocks: 11,
                    num_codewords: 71,
                    num_data_codewords: 45,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 19,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 17,
                    num_codewords: 47,
                    num_data_codewords: 21,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 48,
                    num_data_codewords: 22,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 19,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 9,
                    num_codewords: 39,
                    num_data_codewords: 13,
                },
                Block {
                    num_blocks: 16,
                    num_codewords: 40,
                    num_data_codewords: 14,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 20,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 135,
                    num_data_codewords: 107,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 136,
                    num_data_codewords: 108,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 20,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 67,
                    num_data_codewords: 41,
                },
                Block {
                    num_blocks: 13,
                    num_codewords: 68,
                    num_data_codewords: 42,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 20,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 15,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 20,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 15,
                    num_codewords: 43,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 44,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 21,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 144,
                    num_data_codewords: 116,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 145,
                    num_data_codewords: 117,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 21,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![Block {
                num_blocks: 17,
                num_codewords: 68,
                num_data_codewords: 42,
            }],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 21,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 17,
                    num_codewords: 50,
                    num_data_codewords: 22,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 51,
                    num_data_codewords: 23,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 21,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 47,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 22,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 139,
                    num_data_codewords: 111,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 140,
                    num_data_codewords: 112,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 22,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![Block {
                num_blocks: 17,
                num_codewords: 74,
                num_data_codewords: 46,
            }],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 22,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 7,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 16,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 22,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![Block {
                num_blocks: 34,
                num_codewords: 37,
                num_data_codewords: 13,
            }],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 23,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 151,
                    num_data_codewords: 121,
                },
                Block {
                    num_blocks: 5,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 23,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 23,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 23,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 16,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 24,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 147,
                    num_data_codewords: 117,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 148,
                    num_data_codewords: 118,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 24,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 73,
                    num_data_codewords: 45,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 24,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 16,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 24,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 30,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 47,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 25,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 132,
                    num_data_codewords: 106,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 133,
                    num_data_codewords: 107,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 25,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 13,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 25,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 7,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 22,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 25,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 22,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 13,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 26,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 10,
                    num_codewords: 142,
                    num_data_codewords: 114,
                },
                Block {
                    num_blocks: 2,
                    num_codewords: 143,
                    num_data_codewords: 115,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 26,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 26,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 28,
                    num_codewords: 50,
                    num_data_codewords: 22,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 51,
                    num_data_codewords: 23,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 26,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version10TO26,
            block: vec![
                Block {
                    num_blocks: 33,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 47,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 27,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 153,
                    num_data_codewords: 123,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 27,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 22,
                    num_codewords: 73,
                    num_data_codewords: 45,
                },
                Block {
                    num_blocks: 3,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 27,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 8,
                    num_codewords: 53,
                    num_data_codewords: 23,
                },
                Block {
                    num_blocks: 26,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 27,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 12,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 28,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 4,
        },
        QrCodeVersion {
            version: 28,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 147,
                    num_data_codewords: 117,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 148,
                    num_data_codewords: 118,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 28,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 3,
                    num_codewords: 73,
                    num_data_codewords: 45,
                },
                Block {
                    num_blocks: 23,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 28,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 31,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 28,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 31,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 29,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 7,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 147,
                    num_data_codewords: 117,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 29,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 21,
                    num_codewords: 73,
                    num_data_codewords: 45,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 29,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 1,
                    num_codewords: 53,
                    num_data_codewords: 23,
                },
                Block {
                    num_blocks: 37,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 29,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 26,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 30,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 5,
                    num_codewords: 145,
                    num_data_codewords: 115,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 30,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 30,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 15,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 25,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 30,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 23,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 25,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 31,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 13,
                    num_codewords: 145,
                    num_data_codewords: 115,
                },
                Block {
                    num_blocks: 3,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 31,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 29,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 31,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 42,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 31,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 23,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 28,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 32,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![Block {
                num_blocks: 17,
                num_codewords: 145,
                num_data_codewords: 115,
            }],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 32,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 10,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 23,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 32,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 10,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 35,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 32,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 35,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 33,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 17,
                    num_codewords: 145,
                    num_data_codewords: 115,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 33,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 14,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 21,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 33,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 29,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 19,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 33,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 11,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 46,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 34,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 13,
                    num_codewords: 145,
                    num_data_codewords: 115,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 146,
                    num_data_codewords: 116,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 34,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 14,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 23,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 34,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 44,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 34,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 59,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
                Block {
                    num_blocks: 1,
                    num_codewords: 47,
                    num_data_codewords: 17,
                },
            ],
            num_remainder_bits: 3,
        },
        QrCodeVersion {
            version: 35,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 12,
                    num_codewords: 151,
                    num_data_codewords: 121,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 35,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 12,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 26,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 35,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 39,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 35,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 22,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 41,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 36,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 151,
                    num_data_codewords: 121,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 36,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 6,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 34,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 36,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 46,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 36,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 2,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 64,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 37,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 17,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 153,
                    num_data_codewords: 123,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 37,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 29,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 37,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 49,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 10,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 37,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 24,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 46,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 38,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 4,
                    num_codewords: 152,
                    num_data_codewords: 122,
                },
                Block {
                    num_blocks: 18,
                    num_codewords: 153,
                    num_data_codewords: 123,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 38,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 13,
                    num_codewords: 74,
                    num_data_codewords: 46,
                },
                Block {
                    num_blocks: 32,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 38,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 48,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 14,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 38,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 42,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 32,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 39,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 20,
                    num_codewords: 147,
                    num_data_codewords: 117,
                },
                Block {
                    num_blocks: 4,
                    num_codewords: 148,
                    num_data_codewords: 118,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 39,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 40,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 7,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 39,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 43,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 22,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 39,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 10,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 67,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 40,
            level: ErrorCorrectionLevels::Low,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 19,
                    num_codewords: 148,
                    num_data_codewords: 118,
                },
                Block {
                    num_blocks: 6,
                    num_codewords: 149,
                    num_data_codewords: 119,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 40,
            level: ErrorCorrectionLevels::Medium,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 18,
                    num_codewords: 75,
                    num_data_codewords: 47,
                },
                Block {
                    num_blocks: 31,
                    num_codewords: 76,
                    num_data_codewords: 48,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 40,
            level: ErrorCorrectionLevels::High,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 34,
                    num_codewords: 54,
                    num_data_codewords: 24,
                },
                Block {
                    num_blocks: 34,
                    num_codewords: 55,
                    num_data_codewords: 25,
                },
            ],
            num_remainder_bits: 0,
        },
        QrCodeVersion {
            version: 40,
            level: ErrorCorrectionLevels::Highest,
            data_encoder_type: DataEncoderType::Version27TO40,
            block: vec![
                Block {
                    num_blocks: 20,
                    num_codewords: 45,
                    num_data_codewords: 15,
                },
                Block {
                    num_blocks: 61,
                    num_codewords: 46,
                    num_data_codewords: 16,
                },
            ],
            num_remainder_bits: 0,
        },
    ]
}

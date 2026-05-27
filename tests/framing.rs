#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use hdlc::{decode, decode_slice, encode, FrameReader, HDLCError, SpecialChars};

    #[test]
    fn packetizes() {
        let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
        let cmp: Vec<u8> = vec![126, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, 126];
        let chars = SpecialChars::default();

        let result = encode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn pack_byte_swaps() {
        let msg: Vec<u8> = vec![0x01, 0x7E, 0x00, 0x7D, 0x00, 0x05, 0x80, 0x09];
        let cmp: Vec<u8> = vec![126, 1, 125, 94, 0, 125, 93, 0, 5, 128, 9, 126];
        let chars = SpecialChars::default();

        let result = encode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn pack_custom_s_chars() {
        let msg: Vec<u8> = vec![0x01, 0x7E, 0x70, 0x7D, 0x00, 0x05, 0x80, 0x09];
        let cmp: Vec<u8> = vec![0x71, 1, 126, 112, 80, 125, 0, 5, 128, 9, 0x71];
        let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);

        let result = encode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn pack_rejects_dupe_s_chars() {
        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = encode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::DuplicateSpecialChar)
    }

    #[test]
    fn depacketizes() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];
        let cmp: Vec<u8> = vec![1, 80, 0, 0, 0, 5, 128, 9];

        let result = decode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_it_swaps() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend,
            0x01,
            chars.fesc,
            chars.tfesc,
            0x00,
            0x00,
            chars.fesc,
            chars.tfend,
            0x05,
            0x80,
            0x09,
            chars.fend,
        ];
        let cmp: Vec<u8> = vec![1, 125, 0, 0, 126, 5, 128, 9];

        let result = decode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_custom_s_chars() {
        let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
        let msg: Vec<u8> = vec![
            chars.fend,
            0x01,
            0x7E,
            chars.fesc,
            chars.tfend,
            0x00,
            0x05,
            0x80,
            chars.fesc,
            chars.tfesc,
            0x09,
            0x71,
        ];
        let cmp: Vec<u8> = vec![1, 126, 0x71, 0, 5, 128, 0x70, 9];

        let result = decode(&msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_rejects_dupe_s_chars() {
        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::DuplicateSpecialChar)
    }

    #[test]
    fn depack_rejects_stray_fend_char() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, 0x00, 0x69, 0x00, 0x05, 0x80, 0x09, chars.fend, chars.fend,
        ];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::FendCharInData)
    }

    #[test]
    fn depack_rejects_stray_fesc_char() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, chars.fesc, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::MissingTradeChar)
    }

    #[test]
    fn depack_rejects_incomplete_message() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend,
            0x01,
            chars.fesc,
            chars.tfesc,
            0x77,
            0x00,
            0x05,
            0x80,
            0x09,
        ];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::MissingFinalFend)
    }

    #[test]
    fn depacketizes_slice() {
        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];
        let cmp = [1, 80, 0, 0, 0, 5, 128, 9];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_slice_it_swaps() {
        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend,
            0x01,
            chars.fesc,
            chars.tfesc,
            0x00,
            0x00,
            chars.fesc,
            chars.tfend,
            0x05,
            0x80,
            0x09,
            chars.fend,
        ];
        let cmp = [1, 125, 0, 0, 126, 5, 128, 9];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_slice_custom_s_chars() {
        let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
        let mut msg = [
            chars.fend,
            0x01,
            0x7E,
            chars.fesc,
            chars.tfend,
            0x00,
            0x05,
            0x80,
            chars.fesc,
            chars.tfesc,
            0x09,
            0x71,
        ];
        let cmp = [1, 126, 0x71, 0, 5, 128, 0x70, 9];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), cmp)
    }

    #[test]
    fn depack_slice_rejects_dupe_s_chars() {
        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let mut msg = [0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::DuplicateSpecialChar)
    }

    #[test]
    fn depack_slice_rejects_stray_fend_char() {
        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend, 0x01, 0x00, 0x69, 0x00, 0x05, 0x80, 0x09, chars.fend, chars.fend,
        ];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::FendCharInData)
    }

    #[test]
    fn depack_slice_rejects_stray_fesc_char() {
        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend, 0x01, chars.fesc, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::MissingTradeChar)
    }

    #[test]
    fn depack_slice_rejects_incomplete_message() {
        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend,
            0x01,
            chars.fesc,
            chars.tfesc,
            0x77,
            0x00,
            0x05,
            0x80,
            0x09,
        ];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HDLCError::MissingFinalFend)
    }

    #[test]
    fn get_single_frame() {
        let chars = SpecialChars::default();
        let msg = [chars.fend, 0x01, 0x00, 0x05, 0x80, chars.fend];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }

        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec![126, 1, 0, 5, 128, 126]);
    }

    #[test]
    fn get_single_frame_and_rest() {
        let chars = SpecialChars::default();
        let msg = [
            chars.fend, 0x01, 0x00, 0x05, 0x80, chars.fend, 0x30, 0x10, 0x22,
        ];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec![126, 1, 0, 5, 128, 126]);
    }

    #[test]
    fn get_single_frame_invalid_prefix() {
        let chars = SpecialChars::default();
        let msg = [
            0x1, 0x2, 0x3, chars.fend, 0x01, 0x00, 0x05, 0x80, chars.fend,
        ];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec![126, 1, 0, 5, 128, 126]);
    }

    #[test]
    fn get_single_frame_with_package_end() {
        let chars = SpecialChars::default();
        let msg = [
            chars.fend, chars.fend, 0x53, 0x30, 0x10, 0x22, chars.fend, 0x51, 0x52,
        ];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec![126, 83, 48, 16, 34, 126]);
    }

    #[test]
    fn get_single_frame_with_package_end_as_prefix() {
        let chars = SpecialChars::default();
        let msg = [
            0x01, 0x50, chars.fend, chars.fend, 0x51, 0x53, 0x30, 0x10, 0x22, chars.fend,
        ];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec![126, 81, 83, 48, 16, 34, 126]);
    }

    #[test]
    fn get_multiple_frames() {
        let chars = SpecialChars::default();
        let msg = [
            chars.fend, 0x01, 0x00, 0x05, 0x80, chars.fend,
            chars.fend, 0x02, 0x00, 0x05, 0x80, chars.fend,
            // Start-frame delimeter inferred from previous packet:
            0x03, 0x00, 0x05, 0x80, chars.fend,
            0x11, 0x13, 0x17, 0x1d, 0x1f, chars.fend,
            chars.fend, 0x07, 0x0b, chars.fend
        ];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 5);
        assert_eq!(frames[0], vec![126, 1, 0, 5, 128, 126]);
        assert_eq!(frames[1], vec![126, 2, 0, 5, 128, 126]);
        assert_eq!(frames[2], vec![126, 3, 0, 5, 128, 126]);
        assert_eq!(frames[3], vec![126, 17, 19, 23, 29, 31, 126]);
        assert_eq!(frames[4], vec![126, 7, 11, 126]);
    }

    #[test]
    fn get_frames_no_data() {
        let chars = SpecialChars::default();
        let msg = [];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 0);
    }

    #[test]
    fn only_rest() {
        let chars = SpecialChars::default();
        let msg = [0x05, 0x80, chars.fend, 0x1];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 0);
    }

    #[test]
    fn get_frames_no_special_char() {
        let chars = SpecialChars::default();
        let msg = [0x05, 0x80, 0x1];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 0);
    }

    #[test]
    fn only_frame_start() {
        let chars = SpecialChars::default();
        let msg = [0x05, 0x80, 0x1, chars.fend, chars.fend];
        let mut frames: Vec<Vec<u8>> = vec![];
        let mut reader = Cursor::new(msg);
        let mut hdlc_reader = FrameReader::new(&mut reader, chars);
        loop {
            match hdlc_reader.read_frame() {
                Some(data) => {
                    frames.push(data);
                }
                None => {
                    break;
                }
            }
        }
        assert_eq!(frames.len(), 0);
    }
}

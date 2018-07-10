extern crate hdlc;

#[cfg(test)]
mod tests {
    use hdlc::{decode, decode_slice, encode, HDLCError, SpecialChars};

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
        use std::error::Error;

        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = encode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::DuplicateSpecialChar.description()
        )
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
        use std::error::Error;

        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::DuplicateSpecialChar.description()
        )
    }

    #[test]
    fn depack_rejects_stray_fend_char() {
        use std::error::Error;

        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, 0x00, 0x69, 0x00, 0x05, 0x80, 0x09, chars.fend, chars.fend,
        ];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::FendCharInData.description()
        )
    }

    #[test]
    fn depack_rejects_stray_fesc_char() {
        use std::error::Error;

        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, chars.fesc, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];

        let result = decode(&msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::MissingTradeChar.description()
        )
    }

    #[test]
    fn depack_rejects_incomplete_message() {
        use std::error::Error;

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
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::MissingFinalFend.description()
        )
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
        use std::error::Error;

        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let mut msg = [0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::DuplicateSpecialChar.description()
        )
    }

    #[test]
    fn depack_slice_rejects_stray_fend_char() {
        use std::error::Error;

        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend, 0x01, 0x00, 0x69, 0x00, 0x05, 0x80, 0x09, chars.fend, chars.fend,
        ];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::FendCharInData.description()
        )
    }

    #[test]
    fn depack_slice_rejects_stray_fesc_char() {
        use std::error::Error;

        let chars = SpecialChars::default();
        let mut msg = [
            chars.fend, 0x01, chars.fesc, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09, chars.fend,
        ];

        let result = decode_slice(&mut msg, chars);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::MissingTradeChar.description()
        )
    }

    #[test]
    fn depack_slice_rejects_incomplete_message() {
        use std::error::Error;

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
        assert_eq!(
            result.unwrap_err().description(),
            HDLCError::MissingFinalFend.description()
        )
    }
}

use crc::{Crc, CRC_8_MAXIM_DOW};
use log::error;

// crc checksum
fn crc_check(input: &[u8], len: usize) -> bool {
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let expected_checksum = crc8_checksum.checksum(&input[..=len - 2]);
    let real_sum = input[len - 1];

    match expected_checksum == real_sum {
        true => true,
        false => {
            error!(
                "crc error, expected 0x{:02X}, but 0x{:02X}",
                expected_checksum, real_sum
            );
            false
        }
    }
}

// package crc check
pub fn package_check(input: &[u8], len: usize) -> bool {
    if (input[0] == 0xAA) && (input[1] == 0x55) {
        crc_check(input, len)
    } else {
        false
    }
}

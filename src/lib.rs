mod bcd2bin;
mod bin2bcd;

///
pub const BCD_DIGITS_34: usize = 34;
///
pub const BCD_DIGITS_3: usize = 3;
///
const BIN_BITS_128: usize = 128;
///
const BIN_BITS_8: usize = 8;

pub use crate::bcd2bin::{bcd2bin128, bcd2bin8};
pub use crate::bin2bcd::{bin2bcd128, bin2bcd8};

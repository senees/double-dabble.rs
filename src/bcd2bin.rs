use crate::{BCD_DIGITS_3, BCD_DIGITS_34, BIN_BITS_128, BIN_BITS_8};

///
#[macro_export]
macro_rules! bcd2bin {
  ($bits:expr, $digits:expr, $size:ty, $bcd:tt) => {{
    let mut num: $size = 0;
    let mut u = 0;
    let msb: $size = 1 << $bits - 1;
    for i in 0..$digits {
      if $bcd[i] != 0 {
        break;
      }
      u += 1;
    }
    for _ in 0..$bits {
      num >>= 1;
      let mut k = $digits - 1;
      if ($bcd[k] & 0x1) == 1 {
        num |= msb;
      }
      while k > u {
        if ($bcd[k - 1] & 0x1) > 0 {
          $bcd[k] = ($bcd[k] >> 1) | 0x8;
        } else {
          $bcd[k] >>= 1;
        }
        if $bcd[k] > 7 {
          $bcd[k] -= 3;
        }
        k -= 1;
      }
      $bcd[k] >>= 1;
      if $bcd[k] > 7 {
        $bcd[k] -= 3;
      }
    }
    num
  }};
}

///
pub fn bcd2bin8(bcd: &mut [u8; BCD_DIGITS_3]) -> u8 {
  bcd2bin!(BIN_BITS_8, BCD_DIGITS_3, u8, bcd)
}

///
pub fn bcd2bin128(bcd: &mut [u8; BCD_DIGITS_34]) -> u128 {
  bcd2bin!(BIN_BITS_128, BCD_DIGITS_34, u128, bcd)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bin_1_digit() {
    assert_eq!(1, bcd2bin8(&mut [0, 0, 1]));
  }

  #[test]
  fn test_bin_2_digits() {
    assert_eq!(89, bcd2bin8(&mut [0, 8, 9]));
  }

  #[test]
  fn test_bin_3_digits() {
    assert_eq!(255, bcd2bin8(&mut [2, 5, 5]));
  }

  #[test]
  fn test_bin128_1_digit() {
    assert_eq!(
      1,
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
      ])
    );
  }

  #[test]
  fn test_bin128_8_bcd() {
    assert_eq!(
      89999999,
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_16_bcd() {
    assert_eq!(
      8999999999999999,
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_32_bcd() {
    assert_eq!(
      89999999999999999999999999999999,
      bcd2bin128(&mut [
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_34_bcd() {
    assert_eq!(
      8999999999999999999999999999999999,
      bcd2bin128(&mut [
        8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }
}

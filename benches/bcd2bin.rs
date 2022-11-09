#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::{bcd2bin128, bcd2bin8, BCD_DIGITS_3, BCD_DIGITS_34};
  use test::Bencher;

  #[bench]
  fn bench_bcd2bin_008_01_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_3] = [0, 0, 1];
    b.iter(|| {
      bcd2bin8(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_008_02_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_3] = [0, 8, 9];
    b.iter(|| {
      bcd2bin8(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_008_03_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_3] = [2, 5, 5];
    b.iter(|| {
      bcd2bin8(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_01_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_34] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_08_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_34] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 8, 9, 9, 9, 9, 9, 9, 9,
    ];
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_16_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_34] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_32_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_34] = [
      0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_34_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_34] = [
      8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }
}

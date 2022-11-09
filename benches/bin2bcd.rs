#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::{bin2bcd128, BCD_DIGITS_34};
  use test::Bencher;

  #[bench]
  fn bench_bin2bcd_008_01_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_008_02_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(89, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_008_03_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(255, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_128_01_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_128_08_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(89999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_128_16_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(8999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_128_32_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(89999999999999999999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bin2bcd_128_34_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_34];
    b.iter(|| {
      bin2bcd128(8999999999999999999999999999999999, &mut bcd);
    });
  }
}

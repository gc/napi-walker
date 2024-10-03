pub mod math {
  fn gcd(a: u32, b: u32) -> u32 {
    let (a, b) = if a < b { (b, a) } else { (a, b) };

    if a % b == 0 {
      b
    } else {
      gcd(b, a % b)
    }
  }

  pub fn gcd_for_slice(slice: &[u32]) -> u32 {
    if slice.is_empty() {
      return 0;
    }

    let mut iter = slice.iter().skip_while(|x| x == &&0);
    let first = match iter.next() {
      Some(v) => *v,
      None => return 1,
    };

    iter.fold(
      first,
      |acc, cur| {
        if *cur == 0 {
          acc
        } else {
          gcd(*cur, acc)
        }
      },
    )
  }
}

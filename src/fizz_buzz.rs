pub fn fizz_buzz(n: i64) -> String {
  return match (n % 15, n % 5, n % 3) {
    (0, _, _) => "FIZZ BUZZ".to_string(),
    (_, 0, _) => "BUZZ".to_string(),
    (_, _, 0) => "FIZZ".to_string(),
    _ => n.to_string(),
  };
  // if n % 15 == 0 {
  //   return "FIZZ BUZZ".to_string();
  // }

  // if n % 5 == 0 {
  //   return "BUZZ".to_string();
  // }

  // if n % 3 == 0 {
  //   return "FIZZ".to_string();
  // }
  // n.to_string()
}

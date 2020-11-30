pub fn fizz_buzz(n: i64) -> String {
  if n % 3 == 0 && n % 5 == 0 {
    return "FIZZ BUZZ".to_string();
  }

  if n % 5 == 0 {
    return "BUZZ".to_string();
  }

  if n % 3 == 0 {
    return "FIZZ".to_string();
  }
  n.to_string()
}

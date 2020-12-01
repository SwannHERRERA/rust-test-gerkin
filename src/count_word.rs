pub fn count_word(str: &str) -> usize {
  return str.split(' ').filter(|s| s.len() > 0).count();
}

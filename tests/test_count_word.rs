use fizz_buzz_lib::count_word;

use count_word::count_word;

#[test]
fn empty_string() {
  let result = count_word("");
  assert_eq!(result, 0);
}

#[test]

fn one_word() {
  let result = count_word("word");
  assert_eq!(result, 1);
}

#[test]
fn multiple_word() {
  let result = count_word("Jo la p b   g");
  assert_eq!(result, 5);
}

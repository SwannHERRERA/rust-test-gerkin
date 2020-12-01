use fizz_buzz_lib::fizz_buzz;

use fizz_buzz::fizz_buzz;

pub struct FizzBuzzHelper {
  number: i64,
  result: String,
}

impl FizzBuzzHelper {
  pub fn given_that_number_equals(n: i64) -> FizzBuzzHelper {
    FizzBuzzHelper {
      number: n,
      result: String::new(),
    }
  }

  pub fn when_i_call_fizz_buzz(&mut self) -> &FizzBuzzHelper {
    self.result.push_str(fizz_buzz(self.number).as_ref());
    return self;
  }

  pub fn then_i_shoud_return(&self, expected_result: &str) {
    assert_eq!(self.result.eq(expected_result), true);
  }
}

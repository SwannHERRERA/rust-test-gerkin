mod test_fizz_buzz_helper;

use test_fizz_buzz_helper::FizzBuzzHelper;

#[test]
fn test_fizz_buzz() {
  FizzBuzzHelper::given_that_number_equals(1)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("1");

  FizzBuzzHelper::given_that_number_equals(19)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("19");

  FizzBuzzHelper::given_that_number_equals(3)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("FIZZ");

  FizzBuzzHelper::given_that_number_equals(6)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("FIZZ");

  FizzBuzzHelper::given_that_number_equals(5)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("BUZZ");

  FizzBuzzHelper::given_that_number_equals(20)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("BUZZ");

  FizzBuzzHelper::given_that_number_equals(30)
    .when_i_call_fizz_buzz()
    .then_i_shoud_return("FIZZ BUZZ");
}
#[test]
fn test() {
  assert_eq!(2 + 2, 4);
}
